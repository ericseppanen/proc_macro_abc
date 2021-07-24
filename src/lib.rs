pub use abc_macros::{enum_ranges, file_words, DescribeStruct};

trait DescribeStruct {
    fn struct_name(&self) -> &'static str;
}

#[cfg(test)]
mod describe_tests {
    use super::*;

    #[derive(DescribeStruct)]
    struct Foo;

    #[test]
    fn test_struct_name() {
        assert_eq!(Foo.struct_name(), "Foo");
    }

    #[test]
    fn describe_fail() {
        let t = trybuild::TestCases::new();
        t.compile_fail("tests/build_failures/describe_fail.rs");
    }
}

// Uncomment this to work on the file_words! macro.
/*
#[cfg(test)]
mod words_tests {
    use super::*;

    #[test]
    fn test_file_words() {
        let words = file_words!("tests/words/turbofish.txt");
        assert_eq!(words, ["The", "turbofish", "remains", "undefeated."]);
    }
}
*/

// Uncomment this to work on the enum_ranges! macro.
/*
#[cfg(test)]
mod enum_ranges_tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_enum_ranges() {
        use abc_macros::enum_ranges;

        enum_ranges!(
            #[derive(PartialEq, Debug)]
            LogTen {
                Zero: 0,
                Ones: 1..10,
                Tens: 10..100,
            }
        );

        assert_eq!(LogTen::try_from(0).unwrap(), LogTen::Zero);
        assert_eq!(LogTen::try_from(9).unwrap(), LogTen::Ones);
        assert_eq!(LogTen::try_from(10).unwrap(), LogTen::Tens);
        LogTen::try_from(101).unwrap_err();
    }
}
*/
