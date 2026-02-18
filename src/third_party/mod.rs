//! Third-party plugins.
//!
//! We use one file per plugin to encapsulate setup or boilerplate necessary for that plugin.
//! Many plugins don't require any setup, but it's still nice to have them in an own file so
//! that we are ready to add convenience methods or similar when needed.

// mod bevy_aseprite_ultra;
pub mod bevy_ecs_ldtk_ext;
pub mod bevy_framepace_ext;
pub mod fixes_cursor_unlock_ext;
pub(crate) use bevy_ecs_ldtk_ext::*;
pub(crate) use bevy_ui_anchor_ext::*;
pub mod bevy_image_font_ext;
pub mod bevy_prefab_ext;
pub mod bevy_rand_ext;
pub mod bevy_sprite_layer_ext;
pub mod bevy_tweening_ext;
pub mod bevy_ui_anchor_ext;
pub mod fps_tool_ext;
pub mod pan_camera_ext;
pub mod pyri_tooltip_ext;

pub mod bevy_jornet_ext;

pub mod bevy_seedling_ext;
