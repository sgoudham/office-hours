# office-hours

Are you tired of your code working all day every day? Don't you feel bad that your code keeps working while you're off
relaxing and having fun after work?

Well now you can use the power of `office-hours` to only run your code within the working hours of the day!

> [!IMPORTANT]  
> At the time of writing, the office hours are determined from the **Local Time Zone**
> of the host machine where the code is running. I might consider updating this library
> to support other timezones if I **really** want to suffer :P

## Usage

1. Add this library to your project

    ```shell
    cargo add office-hours
    ```

2. Import and use the `OfficeHours` struct

    ```rust
    use office_hours::OfficeHours;

    fn main() {
        // 9am to 5pm are the default office hours
        let office_hours = OfficeHours::default();
        if office_hours.now() {
            println!("Blimey! Is it time for work already?");
        } else {
            println!("Phew, still on break!");
        }
    }
    ```

3. **(Optional)** Import and use the `office_hours!` macro

    ```rust
    use office_hours::office_hours;
   
    fn main() {
        office_hours!({
            println!("Blimey! Is it time for work already?");
        }) 
    }
    ```

   Further examples can be found in the [examples/](/examples) directory. (`cargo run --example default_hours`)

## Development

> [!NOTE]
> The Minimum Supported Rust Version is **1.60.0**

1. Clone repository

    ```shell
    git clone https://github.com/sgoudham/office-hours.git
    cd office-hours
    ```

2. Build

    ```shell
    cargo build --release
    ```

3. Test

    ```shell
    cargo test --verbose
    ```

## License

[MIT](./LICENSE)