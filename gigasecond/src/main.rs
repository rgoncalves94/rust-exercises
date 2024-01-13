use time::{PrimitiveDateTime as DateTime, macros::*, Duration };

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime { 
    return start + Duration::seconds(1000000000)
}

fn main() {
    print!("Date: {} kiddo!", after(datetime!(2010-10-10 0:10)))
}