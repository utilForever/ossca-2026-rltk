#![allow(clippy::multiple_crate_versions)]

mod fastnoise;

pub mod prelude {
    pub use crate::fastnoise::*;
}
