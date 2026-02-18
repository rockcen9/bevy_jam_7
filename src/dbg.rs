use crate::prelude::*;

// pub const DEBUG_PHYSICS_VISUAL: bool = false;

pub const _ACTIVE_SCENARIO: bool = false;
#[allow(dead_code)]
#[allow(dead_code)]
pub const DEBUG_ENABLE_TELEMETRY: bool = true;

pub(crate) fn plugin(app: &mut App) {
    app.insert_resource(DebugConfig::default());
}
/// Global debug configuration resource for controlling various debug features
#[allow(dead_code)]
#[derive(Resource)]
pub struct DebugConfig {
    pub side_car_windows: bool,
    pub make_window_bigger: bool,

    pub debug_shop: bool,
}

impl Default for DebugConfig {
    fn default() -> Self {
        #[cfg(feature = "dev")]
        {
            Self {
                side_car_windows: false,
                make_window_bigger: false,

                debug_shop: false,
            }
        }
        #[cfg(not(feature = "dev"))]
        {
            Self {
                side_car_windows: false,
                make_window_bigger: false,

                debug_shop: false,
            }
        }
    }
}
