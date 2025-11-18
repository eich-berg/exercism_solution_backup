use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        // todo!("Construct a new Clock from {hours} hours and {minutes} minutes");
        Clock { 
            hours: (hours + minutes.div_euclid(60)).rem_euclid(24), // py (hrs_as_mins // 60) % 24
            minutes: minutes.rem_euclid(60)
            }
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