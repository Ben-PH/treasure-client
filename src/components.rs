use specs::prelude::*;

#[derive(Component)]
#[storage(VecStorage)]
pub enum Origin {
    TopLeft,
    Center,
}

impl Default for Origin {

    fn default() -> Self { Self::Center }
}

#[derive(Default, Component)]
#[storage(NullStorage)]
pub struct Interactable;

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Pos {
    pub x: f64,
    pub y: f64
}

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Layer {
    pub z: f32,
}

#[derive(Default, Component)]
#[storage(VecStorage)]
pub struct Dimension {
    pub w: f64,
    pub h: f64,
}
