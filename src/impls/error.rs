use core::fmt::{Debug, Display, Formatter};

impl<S, L, A> Clone for Error<A>
where
    S: Clone,
    L: Clone,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn clone(&self) -> Self {
        match self {
            Self::RequiresValue(opt) => Self::RequiresValue(opt.clone()),
            Self::DoesNotRequireValue(opt) => Self::DoesNotRequireValue(opt.clone()),
        }
    }
}

impl<S, L, A> Copy for Error<A>
where
    S: Copy + Clone,
    L: Copy + Clone,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
}

impl<S, L, A> PartialEq for Error<A>
where
    S: PartialEq,
    L: PartialEq,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::RequiresValue(opt1), Self::RequiresValue(opt2))
            | (Self::DoesNotRequireValue(opt1), Self::DoesNotRequireValue(opt2)) => opt1.eq(opt2),
            _ => false,
        }
    }
}

impl<S, L, A> Eq for Error<A>
where
    S: Eq + PartialEq,
    L: Eq + PartialEq,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
}

impl<S, L, A> Debug for Error<A>
where
    S: Debug,
    L: Debug,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::RequiresValue(opt) => f.debug_tuple("Error::RequiresValue").field(&opt).finish(),
            Self::DoesNotRequireValue(opt) => f
                .debug_tuple("Error::DoesNotRequireValue")
                .field(&opt)
                .finish(),
        }
    }
}

impl<S, L, A> Display for Error<A>
where
    S: Display,
    L: Display,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
    fn fmt(&self, f: &mut Formatter) -> core::fmt::Result {
        match self {
            Error::RequiresValue(opt) => write!(f, "option requires a value: {}", opt),
            Error::DoesNotRequireValue(opt) => {
                write!(f, "option does not require a value: {}", opt)
            }
        }
    }
}

#[cfg(feature = "std")]
impl<S, L, A> std::error::Error for Error<A>
where
    S: Debug + Display,
    L: Debug + Display,
    A: Argument<ShortOpt = S, LongOpt = L>,
{
}
