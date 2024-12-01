use indoc::formatdoc;
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::meta::ParseNestedMeta;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{bracketed, parse_macro_input, Ident, ItemFn, LitInt, Token};

enum ExpectedExample {
    Single(LitInt),
    Multiple {
        values: Punctuated<LitInt, Token![,]>,
    },
}

impl Parse for ExpectedExample {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // let parser = Punctuated::parse_terminated;

        let output = if let Ok(value) = input.parse() {
            ExpectedExample::Single(value)
        } else {
            let bracket_content;
            let _bracket = bracketed!(bracket_content in input);
            let parser = Punctuated::parse_terminated;

            ExpectedExample::Multiple {
                values: bracket_content.call(parser)?,
            }
        };

        Ok(output)
    }
}

impl ExpectedExample {
    fn test_fn_ident(part: u8, index: Option<usize>) -> Ident {
        match index {
            Some(i) => format_ident!("part{part}_example_{}", i + 1),
            None => format_ident!("part{part}_example"),
        }
    }

    fn example_ident(part: u8, index: Option<usize>) -> Ident {
        match index {
            Some(i) => format_ident!("EXAMPLE_PART{part}_{}", i + 1),
            None => format_ident!("EXAMPLE_PART{part}"),
        }
    }

    fn expand_single(
        value: LitInt,
        fn_name: &Ident,
        part: u8,
        index: Option<usize>,
    ) -> proc_macro2::TokenStream {
        let test_fn_ident = Self::test_fn_ident(part, index);
        let example_ident = Self::example_ident(part, index);

        quote! {
            #[test]
            fn #test_fn_ident() {
                assert_eq!(#fn_name(#example_ident), #value)
            }
        }
    }

    fn expand(self, part: u8, fn_name: &Ident) -> proc_macro2::TokenStream {
        let expand = move |value, index| Self::expand_single(value, fn_name, part, index);
        match self {
            Self::Single(value) => expand(value, None),
            Self::Multiple { values } => values
                .into_iter()
                .enumerate()
                .map(|(i, value)| (expand.clone())(value, Some(i)))
                .collect(),
        }
    }
}

#[derive(Default)]
struct Args {
    day: Option<LitInt>,
    expected_example: Option<ExpectedExample>,
}

impl Args {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::Result<()> {
        if meta.path.is_ident("day") {
            self.day = Some(meta.value()?.parse()?);
        } else if meta.path.is_ident("example") {
            self.expected_example = Some(meta.value()?.parse()?);
        } else {
            return Err(meta.error("unsupported `solution` property"));
        }

        Ok(())
    }
}

pub fn expand(attrs: TokenStream, item: TokenStream) -> TokenStream {
    // TODO: Ugly. Does it really need to be this way? Taken from https://docs.rs/syn/latest/syn/meta/fn.parser.html
    let mut args = Args::default();
    let arg_parser = syn::meta::parser(|meta| args.parse(meta));
    parse_macro_input!(attrs with arg_parser);

    if args.day.is_none() {
        return quote! {
            compile_error!("Day needs to be set in `elvish::solution` macro");
        }
        .into();
    }

    let function = parse_macro_input!(item as ItemFn);

    let fn_name = function.sig.ident.clone();
    let part: u8 = match fn_name.to_string().as_str() {
        "part1" => 1,
        "part2" => 2,
        other => {
            let msg = format!(
                "Function name for solution needs to be either `part1` or `part2`, was {other}"
            );
            return quote! { compile_error!(#msg) }.into();
        }
    };

    let doc = get_doc(args.day.as_ref());

    let item = function.into_token_stream();

    let day = args.day;
    let registration = quote! {
        impl ::elvish::solution::Part<#part, #day> for crate::Solutions {
            fn solve(input: &str) -> impl ::std::fmt::Display {
                #fn_name(input)
            }
        }
    };

    let example = args.expected_example.map(|e| e.expand(part, &fn_name));

    quote! {
        #doc
        #item

        #registration

        #example
    }
    .into()
}

fn get_doc(day: Option<&LitInt>) -> Option<proc_macro2::TokenStream> {
    dotenvy::dotenv().ok()?;
    let session_token = elvish_core::data::get_session_token().ok()?;
    let year = elvish_core::data::get_env_year().ok()?;
    let dat:a = elvish_core::data::get(year, day?.base10_parse().ok()?, &session_token).ok()?;

    let doc_comment = formatdoc!(
        "
            # Description
            
            {}

            {}

            # Input
            
            ```text
            {}
            ```
        ",
        data.description_1,
        data.description_2
            .as_ref()
            .map(|v| v.as_str())
            .unwrap_or(""),
        data.input,
    );

    Some(quote! {
        #[doc = #doc_comment]
    })
}
