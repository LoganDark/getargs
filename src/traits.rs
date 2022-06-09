/// The argument trait for types that can be parsed by
/// [`Options`][crate::Options].
///
/// This trait is implemented for both [`&str`] and [`&[u8]`][slice],
/// and allows them to be understood by `getargs` enough to parse them -
/// `getargs` is entirely generic over the type of its arguments.
///
/// Adding `#[inline]` to implementations of this trait can improve
/// performance by up to 50% in release mode. This is because `Options`
/// is so blazingly fast (nanoseconds) that the overhead of function
/// calls becomes quite significant. `rustc` should be able to apply
/// this optimization automatically, but doesn't for some reason.
///
/// This trait should not need to be implemented unless you are using
/// arguments that cannot be coerced into `&str` or `&[u8]` for whatever
/// reason. If they can be in any way, you should use an
/// [`Iterator::map`] instead of implementing [`Argument`].
pub trait Argument: Sized {
    /// The type of a short option cluster. A short option cluster
    /// consists of 1 or more short options and optionally a value for
    /// the last short option.
    type ShortCluster;

    /// The type of each option in a short option cluster, for example
    /// [`char`].
    type ShortOpt: Clone;

    /// The type of each long option.
    type LongOpt: Clone;

    /// The type of option values.
    type Value;

    /// The type of freestanding, positional arguments.
    type Positional;

    /// Returns `true` if this argument signals that no additional
    /// options should be parsed. If this method returns `true`, then
    /// [`Options::next_opt`][crate::Options::next_opt] will not attempt
    /// to parse it as one ([`parse_long_opt`][Self::parse_long_opt] and
    /// [`parse_short_cluster`][Self::parse_short_cluster] will not be
    /// called).
    ///
    /// This method should only return `true` if [`Self`] is equal to
    /// the string `"--"` (or equivalent in your datatype). It should
    /// not return `true` if [`Self`] merely *starts* with `"--"`, as
    /// that signals a [long option][Self::parse_long_opt].
    fn ends_opts(&self) -> bool;

    /// Attempts to parse this argument as a long option. Returns the
    /// result of the parsing operation, with the leading `--` stripped.
    ///
    /// A long option is defined as an argument that follows the pattern
    /// `--flag` or `--flag=VALUE`, where `VALUE` may be empty. For
    /// example, `"--flag"` would parse as `Some(("flag", None))` and
    /// `"--flag=value"` would parse as `Some(("flag", Some("value")))`.
    /// `"--flag="` would parse as `Some(("flag", Some("")))`.
    fn parse_long_opt(self) -> Result<(Self::LongOpt, Option<Self::Value>), Self>;

    /// Attempts to parse this argument as a "short option cluster".
    /// Returns the short option cluster if present.
    ///
    /// A "short option cluster" is defined as any [`Self`] such that
    /// either at least one [`ShortOpt`][Self::ShortOpt] can be
    /// extracted from it using
    /// [`consume_short_opt`][Self::consume_short_opt], or it can be
    /// converted to a value for a preceding short option using
    /// [`consume_short_val`][Self::consume_short_val].
    ///
    /// A short option cluster is signaled by the presence of a leading
    /// `-` in an argument, and does not include the leading `-`. The
    /// returned "short option cluster" must be valid for at least one
    /// [`consume_short_opt`][Self::consume_short_opt] or
    /// [`consume_short_val`][Self::consume_short_val].
    ///
    /// This method does not need to guard against `--` long options.
    /// [`parse_long_opt`][Self::parse_long_opt] will be called first by
    /// [`Options::next_opt`][crate::Options::next_opt].
    fn parse_short_cluster(self) -> Result<Self::ShortCluster, Self>;

    /// Attempts to consume one short option from a "short option
    /// cluster", as defined by
    /// [`parse_short_cluster`][Self::parse_short_cluster]. Returns the
    /// short option that was consumed and the rest of the cluster (if
    /// non-empty).
    ///
    /// The returned cluster is subject to the same requirements as the
    /// return value of
    /// [`parse_short_cluster`][Self::parse_short_cluster]; namely, its
    /// validity for [`consume_short_opt`][Self::consume_short_opt] or
    /// [`consume_short_val`][Self::consume_short_val].
    fn consume_short_opt(
        cluster: Self::ShortCluster,
    ) -> (Self::ShortOpt, Option<Self::ShortCluster>);

    /// Consumes the value of a short option from a "short
    /// option cluster", as defined by
    /// [`parse_short_cluster`][Self::parse_short_cluster]. Returns the
    /// value that was consumed.
    fn consume_short_val(rest: Self::ShortCluster) -> Result<Self::Value, Self::ShortCluster>;

    /// Converts this argument into an implicit value for an option.
    fn into_value(self) -> Self::Value;

    /// Converts this argument into a positional argument.
    fn into_positional(self) -> Self::Positional;
}

impl Argument for &'_ str {
    type ShortCluster = Self;
    type ShortOpt = char;
    type LongOpt = Self;
    type Value = Self;
    type Positional = Self;

    #[inline]
    fn ends_opts(&self) -> bool {
        *self == "--"
    }

    #[inline]
    fn parse_long_opt(self) -> Result<(Self::LongOpt, Option<Self::Value>), Self> {
        // Using iterators is slightly faster in release, but many times
        // (>400%) as slow in dev

        let option = self
            .strip_prefix("--")
            .filter(|s| !s.is_empty())
            .ok_or(self)?;

        if let Some((option, value)) = option.split_once('=') {
            Ok((option, Some(value)))
        } else {
            Ok((option, None))
        }
    }

    #[inline]
    fn parse_short_cluster(self) -> Result<Self::ShortCluster, Self> {
        self.strip_prefix('-').filter(|s| !s.is_empty()).ok_or(self)
    }

    #[inline]
    fn consume_short_opt(
        cluster: Self::ShortCluster,
    ) -> (Self::ShortOpt, Option<Self::ShortCluster>) {
        let ch = cluster
            .chars()
            .next()
            .expect("<&str as getargs::Argument>::consume_short_opt called on an empty string");

        // using `unsafe` here only improves performance by ~10% and is
        // not worth it for losing the "we don't use `unsafe`" guarantee
        (
            ch,
            Some(&cluster[ch.len_utf8()..]).filter(|s| !s.is_empty()),
        )
    }

    #[inline]
    fn consume_short_val(rest: Self::ShortCluster) -> Result<Self::Value, Self::ShortCluster> {
        Ok(rest)
    }

    #[inline]
    fn into_value(self) -> Self::Value {
        self
    }

    #[inline]
    fn into_positional(self) -> Self::Positional {
        self
    }
}

impl Argument for &'_ [u8] {
    type ShortCluster = Self;
    type ShortOpt = u8;
    type LongOpt = Self;
    type Value = Self;
    type Positional = Self;

    #[inline]
    fn ends_opts(&self) -> bool {
        self == b"--"
    }

    #[inline]
    fn parse_long_opt(self) -> Result<(Self::LongOpt, Option<Self::Value>), Self> {
        let option = self
            .strip_prefix(b"--")
            .filter(|a| !a.is_empty())
            .ok_or(self)?;

        // This is faster than iterators in dev
        let name = option.split(|b| *b == b'=').next().unwrap();
        let value = if name.len() < option.len() {
            Some(&option[name.len() + 1..])
        } else {
            None
        };

        Ok((name, value))
    }

    #[inline]
    fn parse_short_cluster(self) -> Result<Self::ShortCluster, Self> {
        self.strip_prefix(b"-")
            .filter(|a| !a.is_empty())
            .ok_or(self)
    }

    #[inline]
    fn consume_short_opt(
        cluster: Self::ShortCluster,
    ) -> (Self::ShortOpt, Option<Self::ShortCluster>) {
        let (byte, rest) = cluster
            .split_first()
            .expect("<&[u8] as getargs::Argument>::consume_short_opt called on an empty slice");

        (*byte, Some(rest).filter(|s| !s.is_empty()))
    }

    #[inline]
    fn consume_short_val(rest: Self::ShortCluster) -> Result<Self::Value, Self::ShortCluster> {
        Ok(rest)
    }

    #[inline]
    fn into_value(self) -> Self::Value {
        self
    }

    #[inline]
    fn into_positional(self) -> Self::Positional {
        self
    }
}
