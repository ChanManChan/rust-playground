use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Health<'ele> {
    pub value: f32,
    pub id: &'ele str,
}

impl<'ele> Health<'ele> {
    pub fn new(value: f32, id: &'ele str) -> Self {
        Self { value, id }
    }
}
