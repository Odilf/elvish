use proc_macro::{TokenStream, Span};
use quote::{format_ident, quote};
use syn::LitInt;

#[proc_macro_attribute]
pub fn part(attr: TokenStream, item: TokenStream) -> TokenStream {
    let parse_stream = syn::parse_macro_input!(attr as LitInt);
    let part = parse_stream.base10_parse::<usize>().unwrap();
    let span = Span::call_site();

    let item = proc_macro2::TokenStream::from(item);
    // let day_data = elvish_core::data::get();

    let doc_comment = format!("
        This is part {part}
    ");

    quote! {
        #[doc = #doc_comment]
        #item
    }.into()
}

#[proc_macro]
pub fn declare_days_up_to(input: TokenStream) -> TokenStream {
    let parse_stream = syn::parse_macro_input!(input as LitInt);
    let day = parse_stream.base10_parse::<usize>().unwrap();

    fn declare_days_impl(day: usize) -> proc_macro2::TokenStream {
        if day == 0 {
            return quote! {};
        }

        let day_number = format!("{:0>2}", day);
        let mod_ident = format_ident!("day{day_number}");
        let day_full = format!("day{day_number}");

        let prev_day_tokens = declare_days_impl(day - 1);
        let day_tokens = quote! {
            #prev_day_tokens

            #[cfg(feature = #day_full)]
            mod #mod_ident;
        };

        day_tokens
    }

    declare_days_impl(day).into()
}
