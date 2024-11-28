use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Expr, Ident, Token,
};

pub enum Example {
    Single(Expr),
    Multiple {
        parts: syn::punctuated::IntoIter<PartExample>,
    },
}

impl Parse for Example {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parser = Punctuated::<PartExample, Token![,]>::parse_terminated;

        let Ok(punctuated) = input.call(parser) else {
            return Ok(Example::Single(input.parse()?));
        };

        Ok(Example::Multiple {
            parts: punctuated.into_iter(),
        })
    }
}

impl Example {
    pub fn expand(self) -> TokenStream {
        match self {
            Example::Single(expr) => [0, 1]
                .map(|part| {
                    PartExample {
                        part,
                        expr: expr.clone(),
                    }
                    .expand(0)
                })
                .into_iter()
                .collect(),

            Example::Multiple { parts } => {
                let mut part_indices = [0, 0];

                parts
                    .map(|part| {
                        let index = &mut part_indices[part.part as usize];
                        let expansion = part.expand(*index);
                        *index += 1;
                        expansion
                    })
                    .collect()
            }
        }
    }
}

pub struct PartExample {
    part: u8,
    expr: Expr,
}

impl Parse for PartExample {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let part_ident: Ident = input.parse()?;
        let part = match part_ident.to_string().as_str() {
            "part1" => 0,
            "part2" => 1,
            _ => {
                return syn::Result::Err(syn::Error::new(
                    part_ident.span(),
                    "Part needs to be either `part1` or `part2`",
                ))
            }
        };

        let _colon: Token![:] = input.parse()?;
        let expr = input.parse()?;

        Ok(Self { part, expr })
    }
}

impl PartExample {
    // TODO: Naming here is ass
    /// Generates
    ///
    /// ```rust
    /// const EXAMPLE_PARTx_y = {expr};
    /// ```
    ///
    /// And, additionally
    ///
    /// ```rust
    /// const EXAMPLE_PARTx = EXAMPLE_PARTx_y; // Refers to the specific example
    /// ```
    ///
    /// if it's the first one.
    fn expand(&self, index: u32) -> TokenStream {
        let ident = format_example(self.part, Some(index));

        let expr = &self.expr;
        let example = quote! {
            #[cfg(test)]
            const #ident: &str = elvish::indoc!(#expr);
        };

        if index == 0 {
            let ident_general = format_example(self.part, None);
            quote! {
                #example

                #[cfg(test)]
                const #ident_general: &str = #ident;
            }
            .into()
        } else {
            example.into()
        }
    }
}

fn format_example(part: u8, index: Option<u32>) -> Ident {
    match index {
        Some(index) => format_ident!("EXAMPLE_PART{}_{}", part + 1, index + 1),
        None => format_ident!("EXAMPLE_PART{}", part + 1),
    }
}
