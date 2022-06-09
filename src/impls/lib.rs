use core::fmt::{Debug, Formatter};

impl<S, C, L, V, P, A, I> Clone for Options<A, I>
where
    S: Clone,
    C: Clone,
    L: Clone,
    V: Clone,
    P: Clone,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
    I: Iterator<Item = A> + Clone,
{
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            iter: self.iter.clone(),
        }
    }
}

impl<S, C, L, V, P, A, I> Copy for Options<A, I>
where
    S: Copy + Clone,
    C: Copy + Clone,
    L: Copy + Clone,
    V: Copy + Clone,
    P: Copy + Clone,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
    I: Iterator<Item = A> + Copy + Clone,
{
}

impl<S, C, L, V, P, A, I> Debug for Options<A, I>
where
    S: Debug,
    C: Debug,
    L: Debug,
    V: Debug,
    P: Debug,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
    I: Iterator<Item = A> + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Options")
            .field("iter", &self.iter)
            .field("state", &self.state)
            .finish()
    }
}

impl<S, C, L, V, P, A> Clone for State<A>
where
    S: Clone,
    C: Clone,
    L: Clone,
    V: Clone,
    P: Clone,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Start { ended_opts } => Self::Start {
                ended_opts: *ended_opts,
            },
            Self::Positional(positional) => Self::Positional(positional.clone()),
            Self::EndOfOption(opt) => Self::EndOfOption(opt.clone()),
            Self::ShortOptionCluster(opt, rest) => {
                Self::ShortOptionCluster(opt.clone(), rest.clone())
            }
            Self::LongOptionWithValue(opt, val) => {
                Self::LongOptionWithValue(opt.clone(), val.clone())
            }
            Self::End { ended_opts } => Self::End {
                ended_opts: *ended_opts,
            },
            Self::Taken => Self::Taken,
        }
    }
}

impl<S, C, L, V, P, A> Copy for State<A>
where
    S: Copy + Clone,
    C: Copy + Clone,
    L: Copy + Clone,
    V: Copy + Clone,
    P: Copy + Clone,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
{
}

impl<S, C, L, V, P, A> PartialEq for State<A>
where
    S: PartialEq,
    C: PartialEq,
    L: PartialEq,
    V: PartialEq,
    P: PartialEq,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Self::Start {
                    ended_opts: ended_opts1,
                },
                Self::Start {
                    ended_opts: ended_opts2,
                },
            ) => ended_opts1.eq(ended_opts2),
            (Self::Positional(positional1), Self::Positional(positional2)) => {
                positional1.eq(positional2)
            }
            (Self::EndOfOption(opt1), Self::EndOfOption(opt2)) => opt1.eq(opt2),
            (Self::ShortOptionCluster(name1, value1), Self::ShortOptionCluster(name2, value2)) => {
                name1.eq(name2) && value1.eq(value2)
            }
            (
                Self::LongOptionWithValue(name1, value1),
                Self::LongOptionWithValue(name2, value2),
            ) => name1.eq(name2) && value1.eq(value2),
            (
                Self::End {
                    ended_opts: ended_opts1,
                },
                Self::End {
                    ended_opts: ended_opts2,
                },
            ) => ended_opts1.eq(ended_opts2),
            _ => false,
        }
    }
}

impl<S, C, L, V, P, A> Eq for State<A>
where
    S: Eq + PartialEq,
    C: Eq + PartialEq,
    L: Eq + PartialEq,
    V: Eq + PartialEq,
    P: Eq + PartialEq,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
{
}

impl<S, C, L, V, P, A> Debug for State<A>
where
    S: Debug,
    C: Debug,
    L: Debug,
    V: Debug,
    P: Debug,
    A: Argument<ShortOpt = S, ShortCluster = C, LongOpt = L, Value = V, Positional = P>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Start { ended_opts } => f
                .debug_struct("State::Start")
                .field("ended_opts", ended_opts)
                .finish(),

            Self::Positional(positional) => f
                .debug_tuple("State::Positional")
                .field(positional)
                .finish(),

            Self::EndOfOption(opt) => f.debug_tuple("State::EndOfOption").field(opt).finish(),

            Self::ShortOptionCluster(name, value) => f
                .debug_tuple("State::ShortOptionCluster")
                .field(name)
                .field(value)
                .finish(),

            Self::LongOptionWithValue(name, value) => f
                .debug_tuple("State::LongOptionWithValue")
                .field(name)
                .field(value)
                .finish(),

            Self::End { ended_opts } => f
                .debug_struct("State::End")
                .field("ended_opts", ended_opts)
                .finish(),

            Self::Taken => f.debug_struct("State::Taken").finish(),
        }
    }
}
