extern crate chrono;
extern crate serde;
extern crate serde_json;

use std::fs;
use std::io;
use std::path::Path;
use chrono::{NaiveDate, Utc, Duration};
use serde::{Deserialize, Serialize};
use super::editing;

// This is used to implement Serialize and Deserialise on the NaiveDate type
mod json_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Serializer, Deserializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    pub fn serialize<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer, {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D,) -> Result<NaiveDate, D::Error> where D: Deserializer<'de>, {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct Message {
    #[serde(with = "json_date_format")]
    date: NaiveDate,
    text: String,
}

pub struct History {
    list: Vec<Message>,
}

impl History {

    pub fn new() -> Self {
        History{
            list: Vec::new(),
        }
    }

    pub fn load_history(&mut self, history_path: &Path) -> Result<(), io::Error> {
        self.list.clear();
        
/*         let json_string = r#"[
                {
                    "date" : "2019-04-04",
                    "text" : "Never gonna give you up"
                },
                {
                    "date" : "2019-04-05",
                    "text" : "Never gonna let you down"
                },
                {
                    "date" : "2019-04-06",
                    "text" : "Never gonna run around and desert you"
                }
        ]"#; */

        let json_string = fs::read_to_string(history_path)?;
        self.list = serde_json::from_str(&json_string)?;

        Ok(())
    }

    pub fn write_history(&self, history_path: &Path) -> Result<(), io::Error> {

        let json_config = serde_json::to_string(&self.list)?;
        fs::write(history_path, json_config)?;

        Ok(())
    }

    pub fn add_message(&mut self, date: NaiveDate) -> Result<(), io::Error> {

        let text = match self.pop_message_by_date(date) {
            Some(message) => {
                editing::edit_existing_message(&message)?
            },
            None => {
                editing::edit_message()?
            }
        };

        self.list.push(
            Message{
                date: date,
                text: text,
            }
        );

        Ok(())
    }

    pub fn add_delayed_message(&mut self, nb_days: i64) -> Result<(), io::Error> {
        let date: NaiveDate = (Utc::today() + Duration::days(nb_days)).naive_utc();

        self.add_message(date)?;
        Ok(())
    }

    pub fn add_message_now(&mut self, text: String) {
        self.list.push(
            Message{
                date: Utc::today().naive_utc(),
                text: text,
            }
        )
    }
    
    #[allow(dead_code)]
    pub fn print_history(&self) {
        println!("----------History---------");
        for message in &self.list 
        {
            println!("Date : {}", message.date);
            println!("{}", message.text);
        }
        println!("--------------------------");
    }

    pub fn find_message_by_nb_day(&self, nb_days: i64) -> Option<&String> {
        let lookup_date: NaiveDate = (Utc::today() + Duration::days(nb_days)).naive_utc();

        for message in &self.list 
        {
            if message.date == lookup_date 
            {
                return Some(&message.text)
            }
        }
        return None;
    }

    fn find_message_by_date(&self, date: NaiveDate) -> Option<&String> {

        for message in &self.list 
        {
            if message.date == date 
            {
                return Some(&message.text);
            }
        }

        return None;
    }

    fn pop_message_by_date(&mut self, date: NaiveDate) -> Option<String> {

        let mut pop_message: Option<String> = None;

        for message in &self.list 
        {
            if message.date == date 
            {
                pop_message = Some(message.text.clone());
            }
        }

        match pop_message {
            Some(pop_message) => {
                self.list.retain(|s| s.date != date);
                return Some(pop_message);
            },
            None => {
                return None;
            }
        }
    }
}
