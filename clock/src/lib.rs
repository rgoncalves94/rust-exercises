use std::fmt;
#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
  hours: i32,
  minutes: i32,
}

impl Clock {
  pub fn new(hours: i32, minutes: i32) -> Self {

  let (new_hours, new_minutes) = calculate_clock(hours, minutes);

   return  Clock {
      hours: new_hours,
      minutes: new_minutes,
    };
  }
  
  pub fn add_minutes(&self, minutes: i32) -> Self {
    let (zhours, zminutes)  = calculate_clock(self.hours, self.minutes + minutes);
  
    Clock { 
      hours: zhours,
      minutes: zminutes,
    }
  }
}

pub fn calculate_clock(z_hours: i32, z_minutes: i32) -> (i32, i32) {
  let total_minutes = z_minutes;
  let hours_to_add = total_minutes / 60;
  let mut minutes_mod = total_minutes % 60;
  let mut hours = z_hours + hours_to_add;

  while hours >= 24 {
    hours -= 24; 
  }

  while hours <= -24 {
    hours += 24;
  }

  if minutes_mod < 0 {
    println!("pasosu aqui");
    hours -=1;
    minutes_mod = 60 + minutes_mod;
  }

  if hours < 0 {
    hours = 24 + hours
  }

  (hours, minutes_mod)
}

impl fmt::Display for Clock {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let hours = if self.hours < 10 { format!("0{}", self.hours) } else { self.hours.to_string() };
    let minutes = if self.minutes < 10 { format!("0{}", self.minutes) } else { self.minutes.to_string() };
    write!(f, "{}:{}", hours, minutes)
  }
}

