use std::fmt;
#[derive(Debug)]
pub struct Clock {
  minutes: i32,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {

   return  Clock {
      minutes:  hours * 60 + minutes,
    };
  }
  
  pub fn add_minutes(&self, minutes: i32) -> Self {
    Clock { 
      minutes: self.minutes + minutes,
    }
  }
}

pub fn calculate_clock(minutes: i32) -> (i32, i32) {
  let mut hours =  minutes / 60 ;
  let mut minutes_mod = minutes % 60;

  while hours <= -24 {
    hours += 24;
  }

  while hours >= 24 {
    hours -= 24;
  }

  if minutes_mod < 0 {
    minutes_mod = 60 + minutes_mod;
    hours -= 1;
  }

  if hours < 0 {
    hours = 24 + hours
  }

  if hours < 0 {
    hours *= -1
  }

  if minutes_mod < 0 {
    minutes_mod *= -1
  }

  (hours, minutes_mod)
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let (hours, minutes) = calculate_clock(self.minutes);
    let str_hours = if hours < 10 { format!("0{}", hours) } else { hours.to_string() };
    let str_minutes = if minutes < 10 { format!("0{}", minutes) } else { minutes.to_string() };
    write!(f, "{}:{}", str_hours, str_minutes)
  }
}

impl PartialEq for Clock {
  fn eq(&self, other: &Self) -> bool {
      self.to_string() == other.to_string()
  }
}
