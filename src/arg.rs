use crate::{Argument, Opt};

/// An option or positional argument.
///
/// This enum can be returned by calls to
/// [`Options::next_arg`][crate::Options::next_arg] and represents a
/// short or long command-line option name (but not value) like [`Opt`],
/// or a positional argument.
pub enum Arg<A: Argument> {
    /// A short option, like `-f`. Does not include the leading `-`.
    Short(A::ShortOpt),
    /// A long option, like `--file`. Does not include the leading `--`.
    Long(A::LongOpt),
    /// A positional argument, like `foo.txt`.
    Positional(A::Positional),
}

impl<A: Argument> Arg<A> {
    /// Retrieves an equivalent [`Opt`] represented by this [`Arg`], if
    /// it is [`Arg::Short`] or [`Arg::Long`], otherwise `None`.
    pub fn opt(self) -> Option<Opt<A>> {
        match self {
            Self::Short(short) => Some(Opt::Short(short)),
            Self::Long(long) => Some(Opt::Long(long)),
            _ => None,
        }
    }

    /// Returns the positional [`Argument`] represented by this [`Arg`],
    /// if it is [`Arg::Positional`], otherwise `None`.
    pub fn positional(self) -> Option<A::Positional> {
        match self {
            Self::Positional(arg) => Some(arg),
            _ => None,
        }
    }
}

impl<A: Argument> From<Opt<A>> for Arg<A> {
    fn from(opt: Opt<A>) -> Self {
        match opt {
            Opt::Short(short) => Self::Short(short),
            Opt::Long(long) => Self::Long(long),
        }
    }
}

include!("impls/arg.rs");
