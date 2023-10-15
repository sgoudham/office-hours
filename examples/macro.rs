use office_hours::office_hours;

fn main() {
    office_hours!({ println!("Between 9am and 5pm") });
    office_hours!(Clock::FivePm, Clock::TenPm, {
        println!("Between 5pm and 10pm")
    });
}
