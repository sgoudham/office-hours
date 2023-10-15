use office_hours::{Clock, OfficeHours};

fn main() {
    let office_hours = OfficeHours::new(Clock::TwelvePm, Clock::FivePm);
    if office_hours.now() {
        println!("Blimey! Is it time for work already?")
    } else {
        println!("Phew, still on break!")
    }
}
