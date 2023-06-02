mod time;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let t = time::buildTime();
        assert_eq!(t.t, 0);
    }

    #[test]
    fn equal() {
        let t1 = time::new(2, 30, 0);
        let t2 = time::new(2, 30, 0);
        assert_eq!(t1, t2);
    }

    #[test]
    fn lesser_than() {
        let t1 = time::new(2, 30, 0);
        let t2 = time::new(3, 45, 5);
        assert_eq!(t1 < t2, true);
    }

    #[test]
    fn print() {
        let t = time::new(2, 30, 0);
        assert_eq!(t.print_time(), "2:30:0")
    }

    // #[test]
    // fn adding() {
    //     let t1 = time::new(2, 30, 0);
    //     let t2 = time::new(3, 45, 5);
    // }
}