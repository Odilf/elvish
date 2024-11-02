#[macro_export]
macro_rules! year {
    ($year:literal) => {
        const YEAR: u16 = $year;
    };
}

#[macro_export]
macro_rules! day {
    ($day:literal) => {
        #[cfg(feature = "part1")]
        impl ::elvish::solution::Part<1, $day> for crate::Solutions {
            fn solve(input: &str) -> impl std::fmt::Display {
                part1(input)
            }
        }

        #[cfg(feature = "part2")]
        impl ::elvish::solution::Part<2, $day> for crate::Solutions {
            fn solve(input: &str) -> impl std::fmt::Display {
                part2(input)
            }
        }
    };
}

#[macro_export]
macro_rules! main {
    () => {
        #[cfg(feature = "all")]
        fn main() -> eyre::Result<()> {
            elvish::dotenvy::dotenv()?;
            let session_token = elvish::data::get_session_token()?;
            todo!("Get cli args and run it, somehow");
            // for day in 1..=25 {
            //     let day_data = elvish::data::get(YEAR, day, &session_token)?;
            //     elvish::solution::run_day::<Solutions, _>(&day_data.input);
            // }
            Ok(())
        }

        #[cfg(not(feature = "all"))]
        fn main() -> eyre::Result<()> {
            #[cfg(feature = "day01")]
            const DAY: u8 = 1;
            #[cfg(feature = "day02")]
            const DAY: u8 = 2;
            // TODO:
            // ...and so on

            ::elvish::dotenvy::dotenv()?;

            let session_token = ::elvish::data::get_session_token()?;
            let day_data = elvish::data::get(YEAR, DAY, &session_token)?;

            let input = &day_data.input;

            let copy_and_print = |solution: &str, part| {
                ::elvish::copy_to_clipboard(solution);
                println!("Part {part}: {solution}")
            };

            #[cfg(feature = "part1")]
            copy_and_print(
                &::elvish::solution::run_day_part::<Solutions, DAY, 1>(&input).to_string(),
                1,
            );

            #[cfg(feature = "part2")]
            copy_and_print(
                &::elvish::solution::run_day_part::<Solutions, DAY, 2>(&input).to_string(),
                2,
            );

            Ok(())
        }
    };
}

#[macro_export]
macro_rules! declare_all_days {
    () => {
        ::elvish::macros::declare_days_up_to!(25);
    };
}
