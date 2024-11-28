use proc_macro::TokenStream;
use syn::parse_macro_input;

mod solution;
mod declare;
mod example;

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    solution::expand(attr, item)
}

#[proc_macro]
pub fn example(input: TokenStream) -> TokenStream {
    parse_macro_input!(input as example::Example).expand()
}

#[proc_macro]
pub fn declare_modules(_input: TokenStream) -> TokenStream {
    declare::modules()
}

#[proc_macro]
pub fn declare_run_fn(_input: TokenStream) -> TokenStream {
    declare::run_fn()
}

#[proc_macro]
pub fn available_days(_input: TokenStream) -> TokenStream {
    declare::available_days()
}
