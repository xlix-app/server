pub mod logs;
pub mod consts;
mod as_res;
pub mod console;
pub mod access_code;
pub mod time;

pub use as_res::*;
use std::borrow::Cow;
use sessionless::Sessionless;

pub type AnyString = Cow<'static, str>;

lazy_static! {
    pub static ref SESSIONLESS: Sessionless = Sessionless::new();
}
