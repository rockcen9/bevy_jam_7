use crate::prelude::*;
#[derive(Component, Reflect, Debug)]
#[relationship(relationship_target = RootStation)]
pub struct BelongTo(pub Entity);

#[derive(Component, Deref, Default, Reflect)]
#[relationship_target(relationship = BelongTo)]
pub struct RootStation(Vec<Entity>);
