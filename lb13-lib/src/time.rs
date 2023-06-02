use std::cmp::Ordering;
use std::ops;

#[derive(Debug)]
pub struct Time {
    pub t: u32
}

pub fn buildTime() -> Time {
    Time{t:0}
}

pub fn new(h: u32, m: u32, s: u32) -> Time {
    Time {
        t: h*3600 + m*60 + s
    }
}

impl Time {
    pub fn print_time(&self) -> String {
        let mut time = self.t;
        let h = time / 3600;
        let m = time % 3600;
        let s = time;
        let mut output = "".to_string();
        output.push_str(&(h.to_string()));
        output.push(':');
        output.push_str(&(m.to_string()));
        output.push(':');
        output.push_str(&(s.to_string()));
        output
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        if self.t == other.t {
            return true;
        }
        false
    }
}

impl ops::Add for Time {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            t: self.t + rhs.t,
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.t < other.t {
            return Some(Ordering::Less);
        }
        if self.t > other.t {
            return Some(Ordering::Greater);
        }
        Some(Ordering::Equal)
    }
}

// Time < Time
// Time == Time
// Time + Time
// Time - Time