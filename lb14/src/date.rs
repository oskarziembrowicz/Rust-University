use std::io::stdout;

pub struct Date {
    day: u32,
    month: u32,
    year: u32,
}

impl Date{

    pub fn new(day: u32, month: u32, year: u32) -> Date {
        Date { day, month, year }
    }

    pub fn get_date(&self) -> String{
        let mut output = String::new();
        if self.day / 10 == 0 {
            output.push_str("0");
        }
        output.push_str(&self.day.to_string());
        output.push('/');
        if self.month / 10 == 0 {
            output.push_str("0");
        }
        output.push_str(&self.month.to_string());
        output.push('/');
        output.push_str(&self.year.to_string());
        output
    }

    pub fn set_date(&mut self, day: u32, month: u32, year: u32) -> () {
        self.year = year;
        self.month = month;
        self.day = day;
    }
}

#[cfg(test)]
mod tests {

    use super::Date;

    #[test]
    fn test_getting() {
        let some_date = Date::new(12, 12, 2002);
        assert_eq!(some_date.get_date(), "12/12/2002")
    }

    #[test]
    fn test_setting() {
        let mut some_date = Date::new(12, 12, 2002);
        some_date.set_date(8, 7, 2004);
        assert_eq!(some_date.get_date(), "08/07/2004")
    }
}