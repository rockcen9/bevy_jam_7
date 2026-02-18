use crate::prelude::*;
mod big_eye;
mod big_hand;
mod golden_heart;
mod ra;
pub fn plugin(app: &mut App) {
    let Ok(test_scene) = std::env::var("TEST_SCENE") else {
        return;
    };
    println!("TEST_SCENE: {}", test_scene);

    if test_scene == "big_eye" {
        big_eye::plugin(app);
    }
    if test_scene == "big_hand" {
        big_hand::plugin(app);
    }
    if test_scene == "ra" {
        ra::plugin(app);
    }
    if test_scene == "golden_heart" {
        golden_heart::plugin(app);
    }
}
