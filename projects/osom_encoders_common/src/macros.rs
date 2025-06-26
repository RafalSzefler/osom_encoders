/// Asserts that the condition is true. Otherwise panics
/// in debug mode. And does nothing in release mode.
#[cfg(debug_assertions)]
#[macro_export]
macro_rules! osom_debug_assert {
    ($cond: expr) => {
        if (!($cond)) {
            const _MSG: &str = $crate::_hidden::const_format::concatcp!("Osom assert failed: ", stringify!($cond));
            panic!("{}", _MSG);
        }
    };
    ($cond: expr, $msg: expr) => {
        if (!($cond)) {
            const _MSG: &str = $crate::_hidden::const_format::concatcp!("Osom assert failed: ", $msg);
            panic!("{}", _MSG);
        }
    };
}

/// Asserts that the condition is true. Otherwise panics
/// in debug mode. And does nothing in release mode.
#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! osom_debug_assert {
    ($cond:expr) => {};
    ($cond:expr, $msg: expr) => {};
}

pub use osom_debug_assert;
