use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hr : i32,
    min: i32,
}

impl Clock {
    // time adjustment functions

    fn rollover(hours: i32, minutes: i32) -> (i32, i32) {
        // perform rollovers
        let mut hours_from_mins: i32;
        let mut clock_mins: i32;
        let mut clock_hour: i32;

        if minutes >= 0 {
            hours_from_mins = minutes / 60;
            clock_mins = minutes % 60;
        } else {
            hours_from_mins = ((-1 * minutes) / 60) * -1 ;
            clock_mins = (hours_from_mins * -1 * 60) + minutes;
        }

        clock_hour = (hours + hours_from_mins) % 24;
        if clock_hour < 0 {
            clock_hour = (24 - (clock_hour * -1)) % 24;
        }

        (clock_hour, clock_mins)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        let clock_time = Self::rollover(hours, minutes); 

        Clock{
            hr: clock_time.0,
            min: clock_time.1,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        unimplemented!("Add {} minutes to existing Clock time", minutes);
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hr, self.min)
    }
}