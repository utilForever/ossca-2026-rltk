// Provides Bracket-Lib style CP437/ASCII terminal options to Bevy
#![allow(clippy::multiple_crate_versions)]

mod builder;
mod cp437;
mod fonts;
pub use builder::*;
mod context;
pub use context::*;
mod consoles;
use consoles::*;
mod random_resource;
pub use consoles::{DrawBatch, VirtualConsole};
pub use random_resource::*;
mod textblock;

pub type FontCharType = u16;

pub mod prelude {
    pub use crate::{
        BTermBuilder, BracketContext, DrawBatch, RandomNumbers, TerminalScalingMode,
        VirtualConsole, consoles::TextAlign, cp437::*, textblock::*,
    };
    pub use bracket_color::prelude::*;
    pub use bracket_geometry::prelude::*;
}
