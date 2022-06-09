use core::fmt::{Debug, Display, Formatter};

impl<S, L, A> Clone for Opt<A>
where
    S: Clone,
    L: Clone,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Short(short) => Self::Short(short.clone()),
            Self::Long(long) => Self::Long(long.clone()),
        }
    }
}

impl<S, L, A> Copy for Opt<A>
where
    S: Copy + Clone,
    L: Copy + Clone,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
}

impl<S, L, A> PartialEq for Opt<A>
where
    S: PartialEq,
    L: PartialEq,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Long(long1), Self::Long(long2)) => long1.eq(long2),
            (Self::Short(short1), Self::Short(short2)) => short1.eq(short2),
            _ => false,
        }
    }
}

impl<S, L, A> Eq for Opt<A>
where
    S: Eq + PartialEq,
    L: Eq + PartialEq,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
}

impl<S, L, A> Debug for Opt<A>
where
    S: Debug,
    L: Debug,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Short(short) => f.debug_tuple("Opt::Short").field(&short).finish(),
            Self::Long(long) => f.debug_tuple("Opt::Long").field(&long).finish(),
        }
    }
}

impl<S, L, A> Display for Opt<A>
where
    S: Display,
    L: Display,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            Opt::Short(short) => write!(f, "-{}", short),
            Opt::Long(long) => write!(f, "--{}", long),
        }
    }
}
