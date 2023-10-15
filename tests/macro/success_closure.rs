use office_hours::office_hours;

fn main() {
    office_hours!(Clock::TwelveAm, Clock::ElevenPm, |start, end| {
        println!(
            "Man, I just love when this code works only within my macro hours of {:?} -> {:?}",
            start, end
        )
    });
    office_hours!(Clock::TwelveAm, Clock::ElevenPm, |start, _| {
        println!(
            "Man, I just love when this code works only within my macro hours of {:?} -> Unknown",
            start
        )
    });
    office_hours!(Clock::TwelveAm, Clock::ElevenPm, |_, end| {
        println!(
            "Man, I just love when this code works only within my macro hours of Unknown -> {:?}",
            end
        )
    });
}