mod end;

mod status;
pub(crate) use status::{BattleStatus, BattleStatusType};

mod progress;
pub(crate) use progress::GameProgress;

use bevy::prelude::*;
pub(crate) fn plugin(app: &mut App) {
    app.add_plugins(end::plugin);
    app.add_plugins(status::plugin);
    app.add_plugins(progress::plugin);
}
