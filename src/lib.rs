pub mod numbers;

pub(crate) mod macros {
    /// Implements a trait with a single required const to a type
    /// using the given value.
    ///
    /// ```rust
    /// trait Minimum {
    ///     const MIN: Self;
    /// }
    ///
    /// // `bool` implements `Minimum` with a `MIN` of `false`
    /// give_const!(bool, Minimum, MIN, false);
    ///
    /// // `Option<usize>` implements `Minimum` with a `MIN` of `None`
    /// give_const!(Option<usize>, Minimum, MIN, None);
    /// ```
    #[macro_export]
    macro_rules! give_const {
        ($type:ty, $trait:ty, $const:ident, $val:expr) => {
            impl $trait for $type {
                const $const: Self = $val;
            }
        };
    }
}
