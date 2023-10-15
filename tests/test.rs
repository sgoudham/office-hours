#[cfg(test)]
mod macros {
    #[test]
    fn office_hours_should_compile() {
        let t = trybuild::TestCases::new();
        t.pass("tests/macro/success*.rs");
        t.compile_fail("tests/macro/failure*.rs");
    }
}

#[cfg(test)]
mod lib {
    mod now {
        use chrono::NaiveTime;

        use office_hours::{Clock, FromNaiveTime, OfficeHours};

        #[test]
        fn true_when_start_and_finish_are_equal() {
            let OfficeHours { start, finish } = OfficeHours::new(Clock::TwelvePm, Clock::TwelvePm);
            assert!(OfficeHours::now_from_time(
                &start,
                &finish,
                &NaiveTime::from_time(Clock::TwelvePm)
            ));
        }

        #[test]
        fn true_when_at_hour_boundary() {
            let OfficeHours { start, finish } = OfficeHours::default();
            let start_boundary = NaiveTime::from_slice_u32(&[9, 0, 0, 1]).unwrap();
            assert!(OfficeHours::now_from_time(&start, &finish, &start_boundary));
            let end_boundary = NaiveTime::from_slice_u32(&[16, 59, 59, 59]).unwrap();
            assert!(OfficeHours::now_from_time(&start, &finish, &end_boundary));
        }

        #[test]
        fn true_when_crossing_day_boundary() {
            let office_hours = OfficeHours::new(Clock::EightPm, Clock::FourAm);
            for i in office_hours.iter() {
                assert!(OfficeHours::now_from_time(
                    &office_hours.start,
                    &office_hours.finish,
                    &i
                ));
            }
        }

        #[test]
        fn default_happy_path() {
            let OfficeHours { start, finish } = OfficeHours::default();
            let current = NaiveTime::from_time(Clock::EightPm);
            assert!(!OfficeHours::now_from_time(&start, &finish, &current));

            // before office hours
            for i in 0..9 {
                assert!(!OfficeHours::now_from_time(
                    &start,
                    &finish,
                    &NaiveTime::from_time_u32(i as u32).unwrap()
                ))
            }
            // during office hours
            for i in 9..17 {
                assert!(OfficeHours::now_from_time(
                    &start,
                    &finish,
                    &NaiveTime::from_time_u32(i as u32).unwrap()
                ))
            }
            // after office hours
            for i in 17..23 {
                assert!(!OfficeHours::now_from_time(
                    &start,
                    &finish,
                    &NaiveTime::from_time_u32(i as u32).unwrap()
                ))
            }
        }
    }
}
