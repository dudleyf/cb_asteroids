use bevy::prelude::*;

#[derive(Component, Clone)]
pub enum PlayerShipType {
    A,
    B,
    C,
}

impl PlayerShipType {
    pub fn base_atlas_index(&self) -> usize {
        match &self {
            PlayerShipType::A => 200,
            PlayerShipType::B => 207,
            PlayerShipType::C => 214,
        }
    }
    pub fn all_ships() -> Vec<PlayerShipType> {
        vec![PlayerShipType::A, PlayerShipType::B, PlayerShipType::C]
    }
    pub fn base_ship_speed(&self) -> BaseShipSpeed {
        match self {
            PlayerShipType::A => BaseShipSpeed::new(500.0, 260.0),
            PlayerShipType::B => BaseShipSpeed::new(500.0, 260.0),
            PlayerShipType::C => BaseShipSpeed::new(500.0, 260.0),
        }
    }
}

pub struct BaseShipSpeed {
    pub movement_meters_per_sec: f32,
    pub rotation_degrees_per_sec: f32,
}

impl BaseShipSpeed {
    pub fn new(speed: f32, rotation: f32) -> Self {
        BaseShipSpeed {
            movement_meters_per_sec: speed,
            rotation_degrees_per_sec: f32::to_radians(rotation),
        }
    }
}
