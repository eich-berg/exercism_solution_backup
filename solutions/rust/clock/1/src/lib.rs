use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let total_minutes_norm = (hours * 60 + minutes).rem_euclid(1440); // 1440 minutes in a day
        let adjusted_hours = total_minutes_norm / 60;
        let adjusted_minutes = total_minutes_norm % 60;

        Clock { hours: adjusted_hours, minutes: adjusted_minutes }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        // todo!("Add {minutes} minutes to existing Clock time");
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}