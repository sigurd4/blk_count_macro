#![no_std]

/// Counts number of comma-separated elements
/// 
/// Elements may be any token
/// 
/// ```rust
/// use blk_count_macro::count;
/// 
/// const COUNT: usize = count!(0, f32, const, something);
/// const COUNT_NONE: usize = count!();
/// 
/// assert_eq!(COUNT, 4);
/// assert_eq!(COUNT_NONE, 0);
/// ```
#[macro_export]
macro_rules! count {
    () => (0usize);
    ( $x:tt $(,$xs:tt)* ) => (1usize + count!($($xs),*));
}
