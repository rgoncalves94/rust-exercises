use std::fmt;
#[derive(Debug, PartialEq)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

const SIXTY_MINUTES: i32 = 60;
const TWENTY_FOUR_HOURS: i32 = 24;

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {
   let (clock_hours, clock_minutes) = calculate_clock(hours * SIXTY_MINUTES + minutes);
   Clock { hours: clock_hours, minutes: clock_minutes }
  }
  
  pub fn add_minutes(&self, minutes: i32) -> Self {
    let (clock_hours, clock_minutes) = calculate_clock(self.hours * SIXTY_MINUTES + self.minutes + minutes);
    Clock { hours: clock_hours, minutes: clock_minutes }
  }
}

pub fn calculate_clock(minutes: i32) -> (i32, i32) {
  let hours = minutes.div_euclid(SIXTY_MINUTES).rem_euclid(TWENTY_FOUR_HOURS);
  let minutes_mod = minutes.rem_euclid(SIXTY_MINUTES);
  (hours, minutes_mod)
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{:02}:{:02}", self.hours, self.minutes)
  }
}