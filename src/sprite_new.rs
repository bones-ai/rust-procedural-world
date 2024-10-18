use bevy::utils::default;

#[derive(Default)]
pub struct Cell {
    position: (usize, usize),
    color: (f32, f32, f32, f32),
}

#[derive(Default)]
pub struct Component {
    cells: Vec<Cell>,
}

#[derive(Default)]
pub struct ComponentGroup {
    components: Vec<Component>,
}

#[derive(Default)]
pub struct Sprite {
    component_groups: Vec<ComponentGroup>,
}

impl Sprite {
    pub fn new(seed: u32) -> Self {
        Sprite { ..default() }
    }
}
