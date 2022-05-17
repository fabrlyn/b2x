pub mod arg {
    use clap::{Arg, ArgMatches};

    pub struct Input;

    impl Input {
        pub fn arg<'a>() -> Arg<'a> {
            Arg::new(Self::name())
                .takes_value(true)
                .required(true)
                .about("The binary string input")
        }

        pub fn value(matches: &ArgMatches) -> &str {
            matches.value_of(Self::name()).unwrap()
        }

        const fn name() -> &'static str {
            "input"
        }
    }

    pub struct BigEndian;

    impl BigEndian {
        pub fn arg<'a>() -> Arg<'a> {
            Arg::new(Self::name())
                .long(Self::name())
                .takes_value(false)
                .required(false)
                .about("Parse the input digits as big endian bits")
        }

        pub fn value(matches: &ArgMatches) -> bool {
            matches.is_present(Self::name())
        }

        const fn name() -> &'static str {
            "big-endian"
        }
    }

    pub struct GroupSize;

    impl GroupSize {
        pub fn arg<'a>() -> Arg<'a> {
            Arg::new(Self::name())
                .long(Self::name())
                .takes_value(true)
                .required(false)
                .default_value("64")
                .about("Set the number of digits for each binary number")
        }

        pub fn value(matches: &ArgMatches) -> u8 {
            matches.value_of_t(Self::name()).unwrap()
        }

        const fn name() -> &'static str {
            "group-size"
        }
    }

    pub struct Signed;

    impl Signed {
        pub fn arg<'a>() -> Arg<'a> {
            Arg::new(Self::name())
                .long(Self::name())
                .takes_value(false)
                .about("If the number should be parsed as a signed number")
        }

        pub fn value(matches: &ArgMatches) -> bool {
            matches.is_present(Self::name())
        }

        const fn name() -> &'static str {
            "signed"
        }
    }

    pub struct Spaced;

    impl Spaced {
        pub fn arg<'a>() -> Arg<'a> {
            Arg::new(Self::name())
                .long(Self::name()).takes_value(false).about("Each binary number can be variable length and is parsed based the space character as a delimiter")
        }

        pub fn value(matches: &ArgMatches) -> bool {
            matches.is_present(Self::name())
        }

        const fn name() -> &'static str {
            "spaced"
        }
    }
}
