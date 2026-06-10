//! Auxiliary types and structs for the storm screensaver.

pub struct LogoCell {
    pub x: usize,
    pub y: usize,
    pub ch: char,
    pub active: bool,
    pub glow: f32,
    pub water: f32,
}

pub struct Drop {
    pub x: f32,
    pub y: f32,
    pub vy: f32,
    pub color: (u8, u8, u8),
    pub is_background: bool,
}

pub struct Splash {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub life: f32,
    pub color: (u8, u8, u8),
    pub is_background: bool,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Phase {
    Building,
    Complete,
    #[allow(dead_code)]
    WashingAway,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BirdState {
    Sitting,
    Flying,
    Scared,
    Explores,
    Dead,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalType {
    Deer,
    Bear,
    Bigfoot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalState {
    Walking,
    Idle,
    Startled,
    WalkingOff,
}

pub struct Animal {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub animal_type: AnimalType,
    pub state: AnimalState,
    pub timer: f32,
    pub frame_toggle: bool,
}

pub type SceneryCell = (usize, usize, char, (u8, u8, u8));
