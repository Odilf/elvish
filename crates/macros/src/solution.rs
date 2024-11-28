use indoc::formatdoc;
use proc_macro::TokenStream;
use quote::ToTokens;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, LitInt};

pub fn expand(args: TokenStream, item: TokenStream) -> TokenStream {
    let mut day: Option<LitInt> = None;
    let mut expected_example_values = Vec::<LitInt>::new();

    let arg_parser = syn::meta::parser(|meta| {
        if meta.path.is_ident("day") {
            day = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("example") {
            expected_example_values.push(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("unsupported solution property"))
        }
    });

    parse_macro_input!(args with arg_parser);

    if day.is_none() {
        return quote! {
            compile_error!("Day needs to be set in `elvish::solution` macro");
        }
        .into();
    }

    let function = parse_macro_input!(item as ItemFn);

    let part_name = function.sig.ident.clone();
    let part: u8 = match part_name.to_string().as_str() {
        "part1" => 1,
        "part2" => 2,
        other => {
            let msg = format!(
                "Function name for solution needs to be either `part1` or `part2`, was {other}"
            );
            return quote! { compile_error!(#msg) }.into();
        }
    };

    let doc = get_doc(day.as_ref());

    let item = function.into_token_stream();

    let registration = quote! {
        impl ::elvish::solution::Part<#part, #day> for crate::Solutions {
            fn solve(input: &str) -> impl ::std::fmt::Display {
                #part_name(input)
            }
        }
    };

    let example: proc_macro2::TokenStream = {
        expected_example_values
            .into_iter()
            .enumerate()
            .map(|(i, example_value)| {
                // TODO: Don't show `partX_example_Y` if there is only one example
                let test_fn_ident = format_ident!("part{part}_example_{}", i + 1);
                let example_ident = format_ident!("EXAMPLE_PART{part}_{}", i + 1);

                quote! {
                    #[test]
                    fn #test_fn_ident() {
                        assert_eq!(#part_name(#example_ident), #example_value)
                    }
                }
            })
            .collect()
    };

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
    let year = elvish_core::data::get_year().ok()?;
    let data = elvish_core::data::get(year, day?.base10_parse().ok()?, &session_token).ok()?;

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
            .unwrap_or_else(|| ""), // God awful for so fucking simple shit
        data.input,
    );

    Some(quote! {
        #[doc = #doc_comment]
    })
}
