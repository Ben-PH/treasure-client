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



#[derive(Component)]
#[storage(VecStorage)]
pub struct Edge {
    pub left: Entity,
    pub right: Entity,
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Text {
    pub st: String,
}

pub struct LineStart {
    pub x: f64,
    pub y: f64,
}

pub struct LineEnd {
    pub x: f64,
    pub y: f64,
}
#[derive(Component)]
#[storage(VecStorage)]
pub struct Line {
    pub start: LineStart,
    pub end: LineEnd,
}
#[derive(PartialEq, Component)]
#[storage(VecStorage)]
pub enum Interactable {
    MouseDown(f64, f64),
    MouseUp,
    Hover,
    Nothing,
}

impl Default for Interactable {

    fn default() -> Self {
        Self::Nothing
    }
}

#[derive(Debug, Default, Component)]
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

#[derive(Debug, Default, Component)]
#[storage(VecStorage)]
pub struct Dimension {
    pub w: f64,
    pub h: f64,
}

