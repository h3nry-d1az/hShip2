use bevy::prelude::*;

use crate::resources::WinSize;

#[derive(Clone, Component)]
pub struct Formation {
    pub start: (f32, f32),
    pub radius: (f32, f32),
    pub pivot: (f32, f32),
    pub speed: f32,
    pub angle: f32
}

#[derive(Default)]
pub struct FormationMaker {
    current_template: Option<Formation>,
    current_members: u32
}
impl FormationMaker {
    pub fn make(&mut self, wsize: &WinSize) -> Formation {
        todo!()
    }
}