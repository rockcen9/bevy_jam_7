use crate::{asset_tracking::LoadResource, prelude::*};

pub(crate) fn plugin(app: &mut bevy::app::App) {
    app.load_resource::<ProcreatesAssets>();
}

// This is for loading only, don't access it from other modules, use file path directly plz.
#[allow(dead_code)]
#[derive(Resource, Asset, Clone, TypePath)]
struct ProcreatesAssets {
    #[dependency]
    pub(crate) calvary: Handle<Image>,
    pub(crate) spear: Handle<Image>,
    pub(crate) archer: Handle<Image>,
    pub(crate) shield: Handle<Image>,
    pub(crate) arrow: Handle<Image>,
    pub(crate) background: Handle<Image>,
    pub(crate) big_eye: Handle<Image>,
    pub(crate) big_hand: Handle<Image>,
    pub(crate) golden_heart: Handle<Image>,
    pub(crate) new_group: Handle<Image>,
    pub(crate) ra: Handle<Image>,
    pub(crate) shadow1: Handle<Image>,
    pub(crate) shadow2: Handle<Image>,
    pub(crate) shadow3: Handle<Image>,
    pub(crate) shadow4: Handle<Image>,
    pub(crate) b1: Handle<Image>,
    pub(crate) b2: Handle<Image>,
    pub(crate) b3: Handle<Image>,
    pub(crate) b4: Handle<Image>,
    pub(crate) c1: Handle<Image>,
    pub(crate) c2: Handle<Image>,
    pub(crate) c3: Handle<Image>,
    pub(crate) c4: Handle<Image>,
    pub(crate) c5: Handle<Image>,
}

impl FromWorld for ProcreatesAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();

        Self {
            calvary: assets.load("procreate/Cavalry.png"),
            spear: assets.load("procreate/Spear.png"),
            archer: assets.load("procreate/Archer.png"),
            shield: assets.load("procreate/Shield.png"),
            arrow: assets.load("procreate/Arrow.png"),
            background: assets.load("procreate/Background.png"),
            big_eye: assets.load("procreate/BigEye.png"),
            big_hand: assets.load("procreate/BigHand.png"),
            golden_heart: assets.load("procreate/GoldenHeart.png"),
            new_group: assets.load("procreate/New group.png"),
            ra: assets.load("procreate/RA.png"),
            shadow1: assets.load("procreate/Shadow1.png"),
            shadow2: assets.load("procreate/Shadow2.png"),
            shadow3: assets.load("procreate/Shadow3.png"),
            shadow4: assets.load("procreate/Shadow4.png"),
            b1: assets.load("procreate/B1.png"),
            b2: assets.load("procreate/B2.png"),
            b3: assets.load("procreate/B3.png"),
            b4: assets.load("procreate/B4.png"),
            c1: assets.load("procreate/C1.png"),
            c2: assets.load("procreate/C2.png"),
            c3: assets.load("procreate/C3.png"),
            c4: assets.load("procreate/C4.png"),
            c5: assets.load("procreate/C5.png"),
        }
    }
}
