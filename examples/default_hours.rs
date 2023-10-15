use office_hours::OfficeHours;

fn main() {
    let office_hours = OfficeHours::default();
    if office_hours.now() {
        println!("Blimey! Is it time for work already?")
    } else {
        println!("Phew, still on break!")
    }
}
