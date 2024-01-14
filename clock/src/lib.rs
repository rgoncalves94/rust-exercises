use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
   let (clock_hours, clock_minutes) = calculate_clock(hours * 60 + minutes);
   Clock { hours: clock_hours, minutes: clock_minutes }
  }
  
  pub fn add_minutes(&self, minutes: i32) -> Self {
    let (clock_hours, clock_minutes) = calculate_clock(self.hours * 60 + self.minutes + minutes);
    Clock { hours: clock_hours, minutes: clock_minutes }
  }
}

pub fn calculate_clock(minutes: i32) -> (i32, i32) {
  let hours = minutes.div_euclid(60).rem_euclid(24);
  let minutes_mod = minutes.rem_euclid(60);
  (hours, minutes_mod)
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hours, self.minutes)
  }
}