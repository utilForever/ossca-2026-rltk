#![allow(clippy::multiple_crate_versions)]

mod rex;
mod xpcolor;

pub mod prelude {
    pub use crate::rex::*;
    pub use crate::xpcolor::*;
}
