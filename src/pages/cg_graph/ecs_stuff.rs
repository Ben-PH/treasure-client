use specs::{Join, World, WorldExt, Builder, Read, DispatcherBuilder, WriteStorage, ReadStorage, Component, VecStorage, System, RunNow};
use std::time::Duration;
use specs::prelude::*;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl MyWorld {
    pub fn init_world() -> MyWorld {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<Velocity>();
        world.register::<Color>();
        MyWorld{inner: world}
    }
}
#[derive(Default)]
pub struct DTime(std::time::Duration);

#[derive(Default)]
pub struct MyWorld {
    pub inner: World,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}
pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (Read<'a, DTime>,
                       ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>);
    fn run(&mut self, data: Self::SystemData) {
        let (delta, vel, mut pos) = data;
        let delta = delta.0;
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x = pos.x + (delta.as_millis() as f32 / 1000.) * vel.x;
            pos.y = pos.y + (delta.as_millis() as f32 / 1000.) * vel.y;
        }
    }
}
impl std::fmt::Debug for MyWorld {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("World")
            .finish()
    }
}
