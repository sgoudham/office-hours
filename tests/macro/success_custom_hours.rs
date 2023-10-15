use office_hours::office_hours;

fn main() {
    office_hours!(Clock::TenAm, Clock::FourPm, {
        println!("I love my macro hours from 10:00 to 16:00")
    });
}