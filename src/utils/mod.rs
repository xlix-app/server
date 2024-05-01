pub mod logs;
pub mod consts;
mod as_res;
pub mod console;

pub use as_res::*;
use std::borrow::Cow;

pub type AnyString = Cow<'static, str>;
