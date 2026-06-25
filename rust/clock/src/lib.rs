use std::fmt::Display;


const MINUTES_IN_A_DAY: i32 = 1440;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    time: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let time = hours * 60 + minutes;
        Self { time: time.rem_euclid(MINUTES_IN_A_DAY)}
    }

    fn hours(&self) -> i32 {
        self.time / 60
    }

    fn minutes(&self) -> i32 {
        self.time % 60
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        let time = self.time;
        let new_time = time + minutes;
        Clock { time: new_time.rem_euclid(MINUTES_IN_A_DAY)}
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:02}:{:02}", self.hours(), self.minutes()))
    }
}

mod test {

    #[test]
    fn test_negative_remainders() {
        // let num: i32 = -12;
        // assert_eq!(num.rem_euclid(24), 7);

        let num: i32 = -20;
        assert_eq!(num.rem_euclid(60), 40);
    }
}
