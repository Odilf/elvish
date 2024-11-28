use proc_macro::TokenStream;
use quote::{format_ident, quote};

pub fn modules() -> TokenStream {
    (1..=25)
        .map(|day| {
            let day_number = format!("{:0>2}", day);
            let day_full = format!("day{day_number}");
            let mod_ident = format_ident!("day{day_number}");

            TokenStream::from(quote! {
                #[cfg(feature = #day_full)]
                mod #mod_ident;
            })
        })
        .collect()
}

pub fn run_fn() -> TokenStream {
    let body: proc_macro2::TokenStream = (1..=25u8)
        .map(|day| {
            let day_number = format!("{:0>2}", day);
            let day_full = format!("day{day_number}");

            quote! {
                #[cfg(feature = #day_full)]
                if day == #day {
                    #[cfg(feature = "part1")]
                    if part == 0 {
                        return Ok(elvish::solution::run_day_part::<Solutions, #day, 1>(input));
                    }

                    #[cfg(feature = "part2")]
                    if part == 1 {
                        return Ok(elvish::solution::run_day_part::<Solutions, #day, 2>(input));
                    }
                }
            }
        })
        .collect();

    quote! {
        fn run_day_part(day: u8, part: u8, input: &str) -> eyre::Result<String> {
            #body

            panic!("The given day ({day}) and part ({part}) was not available");
        }
    }
    .into()
}

pub fn available_days() -> TokenStream {
    let body: proc_macro2::TokenStream = (1..=25u8)
        .map(|day| {
            let day_number = format!("{:0>2}", day);
            let day_full = format!("day{day_number}");

            quote! {
                #[cfg(feature = #day_full)]
                #day ,
            }
        })
        .collect();

    quote! {
        [
            #body
        ]
    }
    .into()
}
