

# proc-macro ABC

This is a set of exercises, for practicing writing procedural macros.

Prerequisites:
- Install [cargo-expand]
- Nighly Rust (for `cargo expand`)
- Docs for [`syn`], [`quote`], [`proc-macro2`]

### Discussion

#### Should I create a macro for this?

Macros should only be used when it would be impractical to write regular code. Macros are harder to debug, and often result in a less pleasant experience for the user. Simple user mistakes may emit hard to understand compiler warnings or errors.

#### Kinds of proc-macros

There are three different flavors of proc-macros:
- *derive macros* are used to add code after observing a data structure (a struct or enum). Often this means implementing a trait, though there's no restriction on the code actually emitted (it can't rewrite the affected data structure, though).
- *attribute macros* attach to some bit of existing Rust syntax (e.g. a data structure or function), and the macro output replaces that syntax.
- *function-like macros* are called with parameters (like `vec!` or `assert!`). Unlike the other two kinds, they can return expressions. They can also be used to emit arbitrary code (e.g. impl blocks, functions, data structures, etc.)

#### Crate organization when building proc-macros

proc-macros have some unusual constraints:
- A proc-macro can only be called from a crate separate from the crate that defines it.
- proc-macros can only be defined in the top module of a crate.
- proc-macro crates must specify `extern crate proc_macro;` to gain access to compiler built-in macro functionality.
- A crate defining proc-macros must have in `Cargo.toml`:
```
[lib]
proc-macro = true
```

#### Macro input/output

Each proc_macro function accepts a `TokenStream` as an input, and returns a `TokenStream` as its output.

`TokenStream` is a list of Rust tokens. Tokens include keywords, operators, identifiers, punctuation, etc.

#### Things macros cannot do

Macros run before types are evaluated. So it's not possible for macros to know things like:
- whether an identifier exists in this context
- the type of an identifier
- what traits a type implements
- the size of a type

#### `syn`, `quote`, and `proc-macro2`

A `TokenStream` is not a syntax tree. Since many macros will want to examine rust syntax, the [`syn`] crate can be used to parse some tokens into a Rust syntax tree. It also has tools (like the `Parse` trait) for parsing other things that may not be valid Rust syntax (though macro inputs must be compatible with the Rust compiler's tokenizer).

The [`quote`] crate provides a convenient macro for creating a `TokenStream`: you can just type regular Rust code and it will be tokenized. In addition, `quote` allows some interpolation of local variables and expansion of iterable items (e.g. expanding a `Vec` containing tokens or syntax nodes).

The [`proc-macro2`] crate provides a wrapper around the compiler's built-in macro functionality. It allows you to do additional things like write unit tests for macros that wouldn't otherwise be possible. Most of the time you won't notice the difference. The most obvious time you will need to be aware of this is when using `quote!`, which returns a `proc_macro2::TokenStream` Because proc_macro functions return a `proc_macro::TokenStream`, you will need to call `.into()` to convert between them.

#### Macro names

Note that macro names exist in a separate namespace; that name will need to be imported (`use foo_macros::some_macro;`) before it can be used. This can be a little confusing with derive macros, which probably have the same name as a trait.

#### Errors and spans

Macros do not return `Result`. A macro failure can either succeed, or return a compile error. There are three ways to return an error:
- `panic!` can be used. It's not as pretty as the other options, though.
- `compile_error!` can be used.
- The `proc_macro_error` crate can be used.

Errors need to be associated with a `Span`, which describes a range of characters in the input file. All `syn` syntax nodes have an associated span; it should be easy to clone this if needed. `quote!` will automatically acquire the span of the entire macro input.

It's not yet possible (in July 2021) for to emit complex diagnostics that mention multiple spans. the way the compiler can-- a macro error will only be associated with a single span.

#### Macro output hazards

There are many subtle hazards to think about when emitting code from a macro:
- Traits, types, etc. may not be imported, or may be overriden or renamed. The most common example is that many modules define a `Result` type. If you need to refer to `Result` in a macro output, use `::std::result::Result`. If a symbol needs to be imported, do it in a context that won't leak to the outside.
- Conflicting names may exist in the same scope. If you need to emit helper functions or data structures, you may need to obfuscate the names or find a way to conceal your symbols in a local scope.
- If returning an expression, the surrounding context may result in different evaluation than expected. 
- Inputs may not be what you expect (e.g. you may expect `Foo` but the user specifies `::mylib::amod::Foo<'a, Vec<&'static str>>`). It may take extra work to determine all of the possible valid inputs and emit the correct outputs.
- There are many lints that may be triggered by macro outputs: unused code, wrong-case identifiers (camel case, snake case, etc.) You may want to disable some lints in the emitted code.

#### Debugging macros

Debugging macros can be an adventure. The experience is nowhere near as polished as developing regular Rust code. In general, more patience and much more careful coding will be required.

To see the code produces by a macro, you can use [cargo-expand]. It can be interesting to run this against more well-established macro crates (e.g. `serde`)-- there are some useful tricks that can be learned this way.

Macros can print to stdout/stderr. It may be a little strange to see the compiler chattering at you while building, but a few strategic `dbg!(my_syn_node)` placements can be really helpful.

Rust-analyzer often behaves strangely while developing proc-macros. Some errors won't show up in the editor at all, and other errors don't clear once fixed. Get used to building in a separate shell.

#### Unit testing compile-time errors

To test that a macro fails gracefully, and returns the expected error message, use the [`trybuild`] crate.

### Exercises

Exercise 1:
- Run `cargo test`; the test will fail because some code is missing.
- Find the `TODO #1` comment in `derive_describe_struct`; add the missing code.
  Verify that the unit test passes.

Exercise 2:
- Un-comment the describe_fail test. It will fail.
- Find the `TODO #2` comment in `derive_describe_struct`.
  Use `compile_error!` to return an error instead of `panic!`.
  The unit test should pass if you get it right.

Exercise 3:
- Add a new trait method `DescribeStruct::field_count()` that returns the
  number of fields in a struct. Add a unit test to verify that it works.

Exercise 4:
- Add the missing implementation of `file_words!` (marked by `TODO` comments).
- Un-comment `test_file_words` and verify the test passes.

Exercise 5:
- Enable the unit tests in `enum_ranges.rs`.
- Add the missing implementations (marked by `TODO` comments).
- Enable `test_enum_ranges` and verify the test passes.

### Solutions

If you'd like to peek at some solutions, look at the `solutions` branch of this repo (<https://github.com/ericseppanen/proc_macro_abc>)

### Want more?

This tutorial was inspired by David Tolnay's [proc-macro workshop]. There isn't a video of the workshop itself, but you can watch Jon Gjengset [work on the exercises][jonhoo-macros].


[cargo-expand]: https://github.com/dtolnay/cargo-expand
[`syn`]: https://docs.rs/syn
[`quote`]: https://docs.rs/quote
[`proc-macro2`]: https://docs.rs/proc-macro2/latest/proc_macro2/
[`proc_macro_error`]: https://docs.rs/proc-macro-error/latest/proc_macro_error/
[`compile_error!`]: https://doc.rust-lang.org/std/macro.compile_error.html
[`trybuild`]: https://docs.rs/trybuild
[proc-macro workshop]: https://github.com/dtolnay/proc-macro-workshop
[jonhoo-macros]: https://www.youtube.com/playlist?list=PLqbS7AVVErFgwC_HByFYblghsDsD5wZDv
