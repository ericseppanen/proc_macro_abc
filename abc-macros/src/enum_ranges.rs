use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream};
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

        // TODO: finish this function. Need to parse the optional
        // "..N" (where N is an integer literal).
        // The parse_one_range unit test may help.

        // Placeholder value, to be replaced by the real implementation.
        let end = None;

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
    fn parse(_input: ParseStream) -> syn::parse::Result<Self> {
        // TODO: Parse a series of NamedRange inputs, separated by commas.

        // A placeholder value, to be replaced by the actual implementation.
        let list = vec![];
        Ok(NamedRangeList {
            // We don't need the punctuation; just iterate then collect
            // to pull the NamedRange elements into a Vec.
            list,
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
    fn to_tokens(&self, _tokens: &mut TokenStream) {
        // TODO: implement this!
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use quote::format_ident;

    #[test]
    #[ignore]
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
    #[ignore]
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
    #[ignore]
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
