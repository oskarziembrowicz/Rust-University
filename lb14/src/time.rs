pub struct Time {
    minutes : u32,
}

impl Time {
    pub fn new(hours: u32, minutes: u32) -> Time {
        Time {
            minutes : hours * 60 + minutes
        }
    }

    pub fn get_time(&self) -> String {
        let mut time = self.minutes;
        let h = time / 60;
        let m = time % 60;
        let mut output = String::new();
        output.push_str(&(h.to_string()));
        output.push(':');
        if m / 10 == 0 {
            output.push_str("0");
        }
        output.push_str(&(m.to_string()));
        output
    }

    pub fn set_time(&mut self, hours: u32, minutes: u32) -> () {
        self.minutes = hours * 60 + minutes;
    }
}

#[cfg(test)]
mod tests {

    use super::Time;

    #[test]
    fn test_getting() {
        let some_time = Time::new(13, 3);
        assert_eq!(some_time.get_time(), "13:03");
    }

    #[test]
    fn test_setting() {
        let mut some_time = Time::new(13, 3);
        some_time.set_time(5, 56);
        assert_eq!(some_time.get_time(), "5:56");
    }
}