use core::fmt::{Debug, Display, Formatter};

impl<S, L, P, A> Clone for Arg<A>
where
    S: Clone,
    L: Clone,
    P: Clone,
    A: Argument<ShortOpt = S, LongOpt = L, Positional = P>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Short(short) => Self::Short(short.clone()),
            Self::Long(long) => Self::Long(long.clone()),
            Self::Positional(positional) => Self::Positional(positional.clone()),
        }
    }
}

impl<S, L, P, A> Copy for Arg<A>
where
    S: Copy + Clone,
    L: Copy + Clone,
    P: Copy + Clone,
    A: Argument<ShortOpt = S, LongOpt = L, Positional = P>,
{
}

impl<S, L, P, A> PartialEq for Arg<A>
where
    S: PartialEq,
    L: PartialEq,
    P: PartialEq,
    A: Argument<ShortOpt = S, LongOpt = L, Positional = P>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Short(short1), Self::Short(short2)) => short1.eq(short2),
            (Self::Long(long1), Self::Long(long2)) => long1.eq(long2),
            (Self::Positional(arg1), Self::Positional(arg2)) => arg1.eq(arg2),
            _ => false,
        }
    }
}

impl<S, L, P, A> Eq for Arg<A>
where
    S: Eq + PartialEq,
    L: Eq + PartialEq,
    P: Eq + PartialEq,
    A: Argument<ShortOpt = S, LongOpt = L, Positional = P>,
{
}

impl<S, L, P, A> Debug for Arg<A>
where
    S: Debug,
    L: Debug,
    P: Debug,
    A: Argument<ShortOpt = S, LongOpt = L, Positional = P>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Short(short) => f.debug_tuple("Arg::Short").field(&short).finish(),
            Self::Long(long) => f.debug_tuple("Arg::Long").field(&long).finish(),
            Self::Positional(arg) => f.debug_tuple("Arg::Positional").field(&arg).finish(),
        }
    }
}

impl<S, L, P, A> Display for Arg<A>
where
    S: Display,
    L: Display,
    P: Display,
    A: Argument<ShortOpt = S, LongOpt = L, Positional = P>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Short(short) => write!(f, "-{}", short),
            Self::Long(long) => write!(f, "--{}", long),
            Self::Positional(arg) => Display::fmt(arg, f),
        }
    }
}
