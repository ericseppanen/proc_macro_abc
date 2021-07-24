//! ## abc-macros: macros used by the `abc` crate.
//!

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, quote_spanned, ToTokens};
use std::fs::File;
use std::io::Read;
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

    // Retrieve the Ident that is the struct name, and convert it to a String.
    let name = &input.ident;
    let name_str = name.to_string();

    // Return a compile error if the name of the struct is "OhNo"
    if name_str == "OhNo" {
        //panic!("That name is not allowed");
        return quote_spanned! {
            name.span() =>
            compile_error!("That name is not allowed");
        }
        .into();
    }

    // Generate the output tokens.
    let expanded = quote! {
        impl DescribeStruct for #name {
            fn struct_name(&self) -> &'static str {
                #name_str
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
    let filename = parse_macro_input!(tokens as LitStr);

    // Read the file into a string.
    let file_result = File::open(filename.value()).and_then(|mut file| {
        let mut buf = String::new();
        file.read_to_string(&mut buf).map(|_| buf)
    });

    // Handle any I/O errors that happened while opening or reading the file.
    // They should result in a compile error.
    let file_data = match file_result {
        Ok(data) => data,
        Err(_e) => {
            // We want to return a compile_error. But we also want to ensure
            // that the tokens returned don't trigger any additional compiler
            // errors, so we will put our compile_error inside an array.
            // Try removing the [] brackets to see what happens without them.
            return quote_spanned! {
                filename.span() =>
                [ compile_error!("Failed to read file") ]

            }
            .into();
        }
    };

    let words = file_data.split_whitespace().collect::<Vec<_>>();

    // A string will be exponded by `quote` into a string literal token.
    // This is exactly what we want.
    //
    // See the `quote::quote` documentation for the interpolation syntax.
    // Note we are interleaving ',` in between word literals.
    //
    let expanded = quote! {
        [
            #(#words),*
        ]
    };

    // proc_macro2::TokenStream -> proc_macro::TokenStream
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
