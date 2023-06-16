// usuwanie
// modyfikacje
// wyświetlanie
// zapis i odczyt pliku

use std::ffi::OsString;
use crate::date::Date;
use crate::time::Time;

pub struct Event {
    name: String,
    date: Date,
    time: Time,
    description: String,
}

impl Event {
    pub fn new(name: String, date: Date, time: Time, description: String) -> Event {
        Event {
            name,
            date,
            time,
            description,
        }
    }

    pub fn get_event(&self) -> String {
        let mut output = String::new();
        output.push_str(&self.name);
        output.push_str("\nDate: ");
        output.push_str(&self.date.get_date());
        output.push_str("\nTime: ");
        output.push_str(&self.time.get_time());
        output.push_str("\nDescription: ");
        output.push_str(&self.description);
        output
    }

    pub fn set_name(&mut self, name: String) -> () {
        self.name = name;
    }

    pub fn set_date(&mut self, date: Date) -> () {
        self.date = date;
    }

    pub fn set_time(&mut self, time: Time) -> () {
        self.time = time;
    }

    pub fn set_description(&mut self, description: String) -> () {
        self.description = description;
    }
}

#[cfg(test)]
mod tests {
    use crate::date::Date;
    use crate::time::Time;
    use super::Event;

    #[test]
    fn test_getting() {
        let some_event = Event::new("Doctor".to_string(), Date::new(13, 7, 2023), Time::new(8,0), "Room 56".to_string());
        assert_eq!(some_event.get_event(), "Doctor\nDate: 13/07/2023\nTime: 8:00\nDescription: Room 56");
    }
}