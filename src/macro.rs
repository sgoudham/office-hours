/// Creates an [`OfficeHours`] that takes in a list of statements to execute
/// when the current time is between office hours.
///
/// - Only execute code between the default hours of 9am and 5pm.
///
/// ```
/// use office_hours::office_hours;
/// office_hours!({ println!("Between 9am and 5pm") });
/// ```
///
/// - Only execute code between the custom hours of 5pm and 10pm.
///
/// ```
/// use office_hours::office_hours;
/// office_hours!(Clock::FivePm, Clock::TenPm, {
///     println!("Between 5pm and 10pm")
/// });
/// ```
/// [`OfficeHours`]: crate::OfficeHours
#[macro_export]
macro_rules! office_hours {
    ({ $($code:stmt)* }) => {{
        use office_hours::Clock;
        office_hours!(Clock::NineAm, Clock::FivePm, { $($code)* })
    }};
    ($start:expr, $finish:expr, { $($code:stmt)* }) => {{
        use office_hours::{Clock, OfficeHours};
        let office_hours = OfficeHours::new($start, $finish);
        if office_hours.now() {
            $($code)*
        }
    }};
    ($start_var:expr, $finish_var:expr, | $start:pat_param, $finish:pat_param | { $($code:stmt)* }) => {{
        use office_hours::{Clock, OfficeHours};
        let office_hours = OfficeHours::new($start_var, $finish_var);
        let $start = &office_hours.start;
        let $finish = &office_hours.finish;
        if office_hours.now() {
            $($code)*
        }
    }};
}
