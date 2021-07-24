use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{braced, Attribute, Ident, LitInt, Token};

/// This represents macro input syntax for a single variant range.
///
/// Example: `Foo: 1..10` or `Bar: 11`
///
#[derive(Debug, PartialEq)]
struct NamedRange {
    name: Ident,
    start: u64,
    end: Option<u64>,
}

/// Parse a `NamedRange` from macro input.
impl Parse for NamedRange {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Try to parse an Ident (the variant name).
        let name: Ident = input.parse()?;
        // Try to parse the ':' after the name.
        input.parse::<Token![:]>()?;
        // Try to parse a literal integer.
        let start_lit: LitInt = input.parse()?;
        let start = start_lit.base10_parse::<u64>()?;
        // Optional: there may be a ".." followed by another integer.
        // If dots are present, the integer must be too.
        let dots = input.parse::<Token![..]>().ok();
        let end = match dots {
            None => None,
            Some(_) => {
                let end_lit: LitInt = input.parse()?;
                let end = end_lit.base10_parse::<u64>()?;
                Some(end)
            }
        };

        Ok(NamedRange { name, start, end })
    }
}

/// Macro syntax for a list of NamedRange types
#[derive(Debug, PartialEq)]
struct NamedRangeList {
    list: Vec<NamedRange>,
}

/// Parse a `NamedRangeList` from macro input.
impl Parse for NamedRangeList {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // syn has some special functions for handling Punctuated syntax.
        // Use those rather than try to parse each token ourselves.
        type PunctList = Punctuated<NamedRange, Token![,]>;
        let id_list = PunctList::parse_terminated(input)?;

        Ok(NamedRangeList {
            // We don't need the punctuation; just iterate then collect
            // to pull the NamedRange elements into a Vec.
            list: id_list.into_iter().collect(),
        })
    }
}

/// This is the entire input to the `enum_ranges!` macro.
///
/// The input is expected to be in the form:
/// ```ignore
/// enum_ranges!(
///     Color {
///         Blue: 450..495,
///         Green: 495..570,
///         Yellow: 570..590,
///     }
/// )
/// ```
///
#[derive(Debug, PartialEq)]
pub struct RangedEnum {
    // If the user wants to attach e.g. #[derive(...)] attributes, we should
    // permit them inside the macro, because there's no way to attach them
    // outside.
    attributes: Vec<Attribute>,
    name: Ident,
    variants: NamedRangeList,
}

/// Parse the macro syntax for `enum_ranges!`
impl Parse for RangedEnum {
    fn parse(input: ParseStream) -> syn::parse::Result<Self> {
        // Parse any attributes. We won't do anything with them, other
        // than emit them in the final output.
        // FIXME: this probably swallows some errors that it shouldn't.
        let attributes = syn::Attribute::parse_outer(input).unwrap_or(vec![]);

        // Try to parse the enum name.
        let name: Ident = input.parse()?;

        // Generate a sub-stream containing whatever is contained in braces.
        let content;
        braced!(content in input);

        // Parse the list that's inside the braces.
        let variants: NamedRangeList = content.parse()?;

        Ok(RangedEnum {
            attributes,
            name,
            variants,
        })
    }
}

/// Emit the tokens that will be returned by the macro.
///
/// It's probably wrong that Parse and ToTokens aren't symmetrical (Parse
/// consumes macro input syntax, while ToTokens emits the macro output).
///
impl ToTokens for RangedEnum {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let attributes = &self.attributes;
        let name = &self.name;

        // Build a Vec<TokenStream>. Each element is one variant, for use
        // constructing the enum.
        let variants = self
            .variants
            .list
            .iter()
            .map(|named_range| {
                let name = &named_range.name;
                quote! { #name, }
            })
            .collect::<Vec<_>>();

        // Build a Vec<TokenStream>. Each element is the "if" statement that
        // handles one variant, in the From<u64> implementation.
        let branches = self
            .variants
            .list
            .iter()
            .enumerate()
            .map(|(n, named_range)| {
                let variant_name = &named_range.name;
                // Generate the "else" token that's needed in between each "if".
                // The first "if" doesn't need one.
                let else_token = match n {
                    0 => TokenStream::new(),
                    _ => quote! { else },
                };

                // Generate the actual "if" logic. There are two cases to handle:
                // 1. The range is a single integer.
                // 2. The range is [start..end].
                let test_tokens = match (named_range.start, named_range.end) {
                    (start, None) => quote! {
                        if x == #start { Ok(#name::#variant_name) }
                    },
                    (start, Some(end)) => quote! {
                        if (#start .. #end).contains(&x) { Ok(#name::#variant_name) }
                    },
                };

                // Assemble the tokens for this variant.
                quote! {
                    #else_token
                    #test_tokens
                }
            })
            .collect::<Vec<_>>();

        // Assemble the final macro output. This is two parts:
        // 1. The enum definition.
        // 2. The From<u64> impl.
        //
        let new_tokens = quote! {
            #(#attributes)*
            enum #name {
                #(#variants)*
            }

            impl ::core::convert::TryFrom<u64> for #name {
                type Error = u64;

                fn try_from(x: u64) -> Result<Self, u64> {
                    #(#branches)*
                    else { Err(x) }
                }
            }
        };

        // ToTokens::to_tokens works by appending its result to an existing
        // TokenStream.
        tokens.extend(new_tokens);
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use quote::format_ident;

    #[test]
    fn parse_one_range() {
        let ranged: NamedRange = syn::parse_str("Foo: 1..10").unwrap();

        assert_eq!(
            ranged,
            NamedRange {
                name: format_ident!("Foo"),
                start: 1,
                end: Some(10),
            }
        );

        let ranged: NamedRange = syn::parse_str("Foo: 7").unwrap();
        assert_eq!(
            ranged,
            NamedRange {
                name: format_ident!("Foo"),
                start: 7,
                end: None,
            }
        );
    }

    #[test]
    fn parse_range_list() {
        let ranges: NamedRangeList = syn::parse_str("Foo: 1..10, Bar: 11").unwrap();

        assert_eq!(
            ranges.list,
            vec![
                NamedRange {
                    name: format_ident!("Foo"),
                    start: 1,
                    end: Some(10),
                },
                NamedRange {
                    name: format_ident!("Bar"),
                    start: 11,
                    end: None,
                }
            ]
        );
    }

    #[test]
    fn parse_ranges() {
        let ranged: RangedEnum = syn::parse_str("MyRanges { Foo: 1..10, Bar: 11 }").unwrap();
        assert_eq!(ranged.name, "MyRanges");
        assert_eq!(
            ranged.variants.list,
            vec![
                NamedRange {
                    name: format_ident!("Foo"),
                    start: 1,
                    end: Some(10),
                },
                NamedRange {
                    name: format_ident!("Bar"),
                    start: 11,
                    end: None,
                }
            ]
        );
    }
}
