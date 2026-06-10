//! Consolidated storm screensaver effect module.
//!
//! **Taxonomy Classification**: System Role (Purpose - Application Software).

pub mod types;
pub mod physics;
pub mod draw;

use library::core::{LcgRng, TerminalCell};
use std::time::Duration;
use library::core::screensaver::Screensaver;

use library::platform::native::sys_info::get_system_info;
use library::toolkit::sys_info::query_current_palette;

use library::toolkit::rgb_controller::{RgbController, is_openrgb_enabled};
use library::toolkit::rgb_protocol::RgbColor;

#[allow(unused_imports)]
pub use self::types::{LogoCell, Drop, Splash, Phase, BirdState, AnimalType, AnimalState, Animal, SceneryCell};

pub struct Storm {
    pub(crate) rng: LcgRng,
    pub(crate) logo_cells: Vec<LogoCell>,
    pub(crate) drops: Vec<Drop>,
    pub(crate) splashes: Vec<Splash>,
    pub(crate) phase: Phase,
    pub(crate) phase_timer: f32,
    pub(crate) last_cols: usize,
    pub(crate) last_rows: usize,
    pub(crate) drop_count_opt: u32,
    pub(crate) assemble_speed_opt: u32,

    // Live system dynamics
    pub(crate) sys_refresh_timer: f32,
    pub(crate) mem_pressure: f32,
    pub(crate) cpu_load: f32,
    pub(crate) host_bias: f32,

    // Puddle accumulation
    pub(crate) puddle: Vec<f32>,
    pub(crate) puddle_color: Vec<(u8, u8, u8)>,

    // Wind dynamics
    pub(crate) wind: f32,

    // Lightning
    pub(crate) lightning_timer: f32,
    pub(crate) lightning_flash: f32,
    pub(crate) lightning_bolts: Vec<Vec<(usize, usize)>>,
    pub(crate) lightning_is_background: bool,
    pub(crate) lightning_delay: f32,

    // Scenery
    pub(crate) bg_cells: Vec<SceneryCell>,
    pub(crate) mid_scenery: Vec<SceneryCell>,
    pub(crate) fg_scenery: Vec<SceneryCell>,

    // Bird state
    pub(crate) bird_x: f32,
    pub(crate) bird_y: f32,
    pub(crate) bird_state: BirdState,
    pub(crate) bird_timer: f32,
    pub(crate) bird_wing_flap: bool,
    pub(crate) bird_vx: f32,
    pub(crate) bird_vy: f32,
    pub(crate) bird_perch_x: f32,
    pub(crate) bird_perch_y: f32,
    pub(crate) perch_points: Vec<(usize, usize)>,

    // Active Animal
    pub(crate) active_animal: Option<Animal>,
    pub(crate) animal_spawn_timer: f32,

    // Subtitles
    pub(crate) subtitle: String,
    pub(crate) subtitle_timer: f32,
    pub(crate) rgb: Option<RgbController>,
}

impl Default for Storm {
    fn default() -> Self {
        Self::new()
    }
}

impl Storm {
    pub fn new() -> Self {
        // Pre-4.1 HKEY_CURRENT_USER registry reads (DropCount, AssembleSpeed)
        // collapsed to defaults for the inline migration. Re-added in 4.2.
        let drop_count_opt: u32 = 1;
        let assemble_speed_opt: u32 = 1;

        let sys = get_system_info();
        Self {
            rng: LcgRng::new(2468),
            logo_cells: Vec::new(),
            drops: Vec::new(),
            splashes: Vec::new(),
            phase: Phase::Building,
            phase_timer: 0.0,
            last_cols: 0,
            last_rows: 0,
            drop_count_opt,
            assemble_speed_opt,
            sys_refresh_timer: 0.0,
            mem_pressure: sys.mem_used_pct / 100.0,
            cpu_load: 0.4,
            host_bias: sys.hostname.chars().map(|c| c as u32).sum::<u32>() as f32 / 1000.0 % 1.0,
            puddle: Vec::new(),
            puddle_color: Vec::new(),
            wind: 0.0,
            lightning_timer: 0.0,
            lightning_flash: 0.0,
            lightning_bolts: Vec::new(),
            lightning_is_background: false,
            lightning_delay: 0.0,
            bg_cells: Vec::new(),
            mid_scenery: Vec::new(),
            fg_scenery: Vec::new(),
            bird_x: 0.0,
            bird_y: 0.0,
            bird_state: BirdState::Sitting,
            bird_timer: 0.0,
            bird_wing_flap: false,
            bird_vx: 0.0,
            bird_vy: 0.0,
            bird_perch_x: 0.0,
            bird_perch_y: 0.0,
            perch_points: Vec::new(),
            active_animal: None,
            animal_spawn_timer: 15.0,
            subtitle: String::new(),
            subtitle_timer: 0.0,
            rgb: if is_openrgb_enabled() { Some(RgbController::new()) } else { None },
        }
    }
}

impl Screensaver for Storm {
    fn update(&mut self, dt: Duration, cols: usize, rows: usize) {
        let delta = dt.as_secs_f32();
        self.phase_timer += delta;

        self.wind = (self.phase_timer * 0.35).sin() * 9.0 + (self.phase_timer * 1.5).cos() * 2.0;

        self.sys_refresh_timer += delta;
        if self.sys_refresh_timer >= 1.0 {
            let sys = get_system_info();
            self.mem_pressure = sys.mem_used_pct / 100.0;
            self.cpu_load = (self.mem_pressure * 0.6 + 0.3).min(0.9);
            if self.host_bias > 0.6 { self.cpu_load = (self.cpu_load + 0.08).min(0.95); }
            self.sys_refresh_timer = 0.0;

            if let Some(ref r) = self.rgb {
                // library 4.0: pull from the cached ScreenPalette.
                let accent = query_current_palette().accent;
                r.set_color(RgbColor::new(accent.0 / 4, accent.1 / 4, accent.2 / 4));
            }
        }

        self.check_resize(cols, rows);

        let load_mult = 1.0 + self.cpu_load * 0.6 + self.mem_pressure * 0.3;
        let speed_mult = match self.assemble_speed_opt {
            0 => 0.6f32,
            2 => 1.6f32,
            _ => 1.0f32,
        } * load_mult;

        self.update_drops(delta, cols, rows, speed_mult);
        self.update_bird(delta, cols, rows);
        self.update_scenery_and_animals(delta, cols, rows);
        self.update_lightning(delta, cols, rows);
    }

    fn draw(&self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        self.draw_impl(grid, cols, rows);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storm_creation() {
        let storm = Storm::new();
        assert_eq!(storm.last_cols, 0);
        assert_eq!(storm.last_rows, 0);
    }

    #[test]
    fn test_storm_update_and_draw() {
        let mut storm = Storm::new();
        storm.update(Duration::from_millis(16), 80, 24);
        let mut grid = vec![TerminalCell::default(); 80 * 24];
        storm.draw(&mut grid, 80, 24);
        // Ensure state variables get initialized
        assert_eq!(storm.last_cols, 80);
        assert_eq!(storm.last_rows, 24);
    }
}

