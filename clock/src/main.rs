use clock::Clock;

fn main() {
  let clock = Clock::new(12, -1);  

  println!("{}", clock.add_minutes(-361).to_string());
}