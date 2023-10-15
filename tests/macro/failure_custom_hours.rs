use office_hours::office_hours;

fn main() {
    office_hours!(10, Clock::FourPm, {
        println!("I love my macro hours from 10:00 to 16:00")
    });
}