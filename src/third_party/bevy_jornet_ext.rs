#[cfg(feature = "backend")]
use bevy_jornet::JornetPlugin;

use crate::prelude::*;

#[cfg(not(feature = "backend"))]
pub(crate) fn plugin(_app: &mut App) {}

#[cfg(feature = "backend")]
pub(crate) fn plugin(app: &mut App) {
    #[cfg(feature = "dev")]
    app.add_plugins(JornetPlugin::with_leaderboard(
        "b2c834e2-6952-4397-98fe-afabab9323a9",
        "cebe0384-f246-456f-ac04-b25172ef2f37",
    ));
    #[cfg(not(feature = "dev"))]
    app.add_plugins(JornetPlugin::with_leaderboard(
        "fd547c18-8f81-4e41-be94-0d346d8ec8cc",
        "c4567f9f-c3ad-4465-ac9e-e8be357f808a",
    ));
}
