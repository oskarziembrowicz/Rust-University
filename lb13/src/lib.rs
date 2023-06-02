pub fn add (left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

let x = buildTime();

mod time {
    struct Time {
        t: u32
    }

    pub fn buildTime() -> Time {
        Time{t:0}
    }
}