#![allow(dead_code)]

use geng::prelude::*;

/// If `f` if `true`, returns `1`, else `0`.
pub fn one<T: UNum>(f: bool) -> T {
    if f { T::ONE } else { T::ZERO }
}
