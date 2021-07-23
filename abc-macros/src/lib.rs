//! ## abc-macros: macros used by the `abc` crate.
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput, LitStr};

/// Derive the `DescribeStruct` trait on a struct (or enum).
///
/// This macro will output code like:
/// ```ignore
/// impl DescribeStruct for Foo {
///     fn struct_name(&self) -> &'static str {
///         "Foo"
///     }
/// }
/// ```
///
/// As a special case, if the name of the struct is `OhNo`, the
/// macro will return a compile error.
///
#[proc_macro_derive(DescribeStruct)]
pub fn derive_describe_struct(input: TokenStream) -> TokenStream {
    // parse the input into a DeriveInput syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Retrieve the Ident that is the struct name
    let name = &input.ident;

    // TODO #2: Return a compile error if the name of the struct is "OhNo"

    // Generate the output tokens.
    // TODO #1: return the correct result instead of "STRUCT_NAME_HERE"
    let expanded = quote! {
        impl DescribeStruct for #name {
            fn struct_name(&self) -> &'static str {
                "STRUCT_NAME_HERE"
            }
        }
    };

    // proc_macro2::TokenStream -> proc_macro::TokenStream
    expanded.into()
}

/// Read a file and return an array of words.
///
/// If the file contains "hello world", this macro will return:
/// `["hello", "world"]`
#[proc_macro]
pub fn file_words(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // Since the macro input already matches an existing rust syntax item (LitStr),
    // we can have syn parse it for us.
    let _filename = parse_macro_input!(tokens as LitStr);

    // TODO: read the file, split words, and then produce the right output.

    // A placeholder output. Replace this with the real implementation.
    let expanded = quote! {
        []
    };
    expanded.into()
}

// Uncomment this section to try the enum_ranges! macro
mod enum_ranges;
use enum_ranges::RangedEnum;

/// enum_ranges! will create an enum to represent numeric ranges.
///
/// For example,
///
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
/// This will emit the following code:
///
/// ```
/// # use core::convert::TryFrom;
/// enum Color {
///     Blue,
///     Green,
///     Yellow,
/// }
///
/// impl TryFrom<u64> for Color {
///     type Error = u64;
///
///     fn try_from(x: u64) -> Result<Self, u64> {
///         if (450..495).contains(&x) { Ok(Color::Blue) }
///         else if (495..570).contains(&x) { Ok(Color::Green) }
///         else if (570..590).contains(&x) { Ok(Color::Yellow) }
///         else { Err(x) }
///     }
/// }
/// ```
#[proc_macro]
pub fn enum_ranges(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ranged = parse_macro_input!(tokens as RangedEnum);
    ranged.into_token_stream().into()
}
