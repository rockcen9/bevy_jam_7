use bevy::{camera::visibility::RenderLayers, prelude::*};

use bitflags::bitflags;
mod camera;
mod scene;
pub(crate) use scene::*;

mod actor;
pub(crate) use actor::*;

mod pawn;
use crate::{
    dbg::{self, DebugConfig},
    screens::Screen,
};
pub(crate) use pawn::*;

mod vfx;
pub(crate) use vfx::*;

pub(crate) use camera::*;

mod land;

mod army;
pub(crate) use army::*;

mod ui;

mod battle;
mod ui_no_root;
pub(crate) use battle::*;

pub(crate) mod audio;
pub(crate) use audio::*;

mod animation;
pub(crate) use animation::*;

mod score;
pub(crate) use score::*;

mod balance;
pub(crate) use balance::*;

mod shop;
pub(crate) use shop::*;

mod memory;
pub(crate) use memory::*;

mod procreate;

mod background;

mod float_damage;

/// System sets for ordering battle systems.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum BattleSystems {
    /// Systems that update unit values (e.g., health, stats).
    UpdateUnitValue,
    /// Systems that calculate combat flux based on unit values.
    CalculateCombatFlux,
}

pub fn game_manager_plugin(app: &mut App) {
    app.add_sub_state::<GameState>();
    // app.configure_sets(
    //     OnEnter(GameState::Preparing),
    //     DaySetupSystems::InitResources.before(DaySetupSystems::SpawnEntities),
    // );
    app.configure_sets(
        Update,
        BattleSystems::UpdateUnitValue
            .before(BattleSystems::CalculateCombatFlux)
            .run_if(in_state(GameState::Battle).or(in_state(GameState::Preparing))),
    );
    // app.add_systems(Update, leave_loading_state);
    // Order new `AppSet` variants by adding them here:
    app.configure_sets(
        Update,
        (
            PostPhysicsAppSystems::TickTimers,
            PostPhysicsAppSystems::ChangeUi,
            PostPhysicsAppSystems::PlaySounds,
            PostPhysicsAppSystems::PlayAnimations,
            PostPhysicsAppSystems::Update,
        )
            .chain(),
    );
    // Set up the `Pause` state.
    app.init_state::<Pause>();
    app.configure_sets(Update, PausableSystems.run_if(in_state(Pause(false))));
    camera::plugin(app);

    actor::plugin(app);
    pawn::plugin(app);
    vfx::plugin(app);
    scene::plugin(app);
    land::plugin(app);
    army::plugin(app);
    ui::plugin(app);
    battle::plugin(app);
    audio::plugin(app);
    dbg::plugin(app);
    balance::plugin(app);
    animation::plugin(app);
    score::plugin(app);
    shop::plugin(app);
    ui_no_root::plugin(app);
    memory::plugin(app);
    procreate::plugin(app);
    rock_materials::plugin(app);
    background::plugin(app);
    float_damage::plugin(app);
    app.add_systems(Startup, auto_start_new_game);
}
#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum PostPhysicsAppSystems {
    /// Tick timers.
    TickTimers,
    /// Change UI.
    ChangeUi,
    /// Play sounds.
    PlaySounds,
    /// Play animations.
    PlayAnimations,
    /// Do everything else (consider splitting this into further variants).
    Update,
}
// #[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default, Reflect, Copy)]
// pub enum Screen {
//     #[default]
//     Splash,
//     Loading,
//     Title,
//     Gameplay,
// }

#[derive(SubStates, Debug, Hash, PartialEq, Eq, Clone, Default, Reflect)]
#[source(Screen = Screen::Gameplay)]
pub enum GameState {
    #[default]
    Preparing,
    Battle,
    WinAndNextDay,
    Lose,
    Leaderboard,
}

fn auto_start_new_game(mut next_screen: ResMut<NextState<Screen>>, _config: Res<DebugConfig>) {
    if std::env::var("AUTO_START_GAME").is_ok() {
        next_screen.set(Screen::Loading);
    }
}
// /// Whether or not the game is paused.
// #[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
// #[states(scoped_entities)]
// pub(crate) struct Pause(pub(crate) bool);

// /// A system set for systems that shouldn't run while the game is paused.
// #[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
// pub(crate) struct PausableSystems;

// /// This enum is converted to an `isize` to be used as a camera's order.
// /// Since we have three camera, we use three enum variants.
// /// This ordering here mean UI > ViewModel > World.
// enum CameraOrder {
//     World,
//     ViewModel,
//     Ui,
// }

// impl From<CameraOrder> for isize {
//     fn from(order: CameraOrder) -> Self {
//         order as isize
//     }
// }

bitflags! {
    struct RenderLayer: u32 {
        /// Used implicitly by all entities without a `RenderLayers` component.
        /// Our world model camera and all objects other than the player are on this layer.
        /// The light source belongs to both layers.
        const DEFAULT = 0b00000001;
        /// Used by the view model camera and the player's arm.
        /// The light source belongs to both layers.
        const VIEW_MODEL = 0b00000010;
        /// Since we use multiple cameras, we need to be explicit about
        /// which one is allowed to render particles.
        const PARTICLES = 0b00000100;
        /// 3D gizmos. These need to be rendered only by a 3D camera, otherwise the UI camera will render them in a buggy way.
        /// Specifically, the UI camera is a 2D camera, which by default is placed at a far away Z position,
        /// so it will effectively render a very zoomed out view of the scene in the center of the screen.
        const GIZMO3 = 0b0001000;
    }
}

impl From<RenderLayer> for RenderLayers {
    fn from(layer: RenderLayer) -> Self {
        // Render layers are just vectors of ints, so we convert each active bit to an int.
        RenderLayers::from_iter(layer.iter().map(|l| (l.bits() >> 1) as usize))
    }
}

/// Whether or not the game is paused.
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub(crate) struct Pause(pub(crate) bool);

/// A system set for systems that shouldn't run while the game is paused.
#[derive(SystemSet, Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub(crate) struct PausableSystems;
