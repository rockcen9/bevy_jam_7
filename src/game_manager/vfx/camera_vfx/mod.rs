use crate::prelude::*;

mod fade;
mod shake;

pub use fade::*;
pub use shake::*;

/// Plugin for screen fade functionality using UI and tweening
pub fn plugin(app: &mut App) {
    fade::plugin(app);
    shake::plugin(app);
}
