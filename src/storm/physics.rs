//! Physics submodules and core resize checks/color generation helpers.

pub mod drops;
pub mod entities;
pub mod lightning;

use library::core::LcgRng;
use library::core::logo_block::render_logo_block;
use library::toolkit::sys_info::get_system_info;

use crate::storm::Storm;
use crate::storm::types::{LogoCell, Phase, BirdState};

impl Storm {
    pub(crate) fn cold_rain_color(rng: &mut LcgRng) -> (u8, u8, u8) {
        let r = rng.next_range(0.0, 1.0);
        if r < 0.60 {
            let brightness = rng.next_range(60.0, 120.0);
            (
                (brightness * 0.8) as u8,
                (brightness * 0.9) as u8,
                brightness as u8,
            )
        } else if r < 0.90 {
            let b = rng.next_range(110.0, 180.0);
            let g = b * rng.next_range(0.6, 0.85);
            let r = g * rng.next_range(0.5, 0.7);
            (r as u8, g as u8, b as u8)
        } else {
            let val = rng.next_range(180.0, 230.0);
            (
                (val * 0.9) as u8,
                (val * 0.95) as u8,
                val as u8,
            )
        }
    }

    pub fn check_resize(&mut self, cols: usize, rows: usize) {
        if cols != self.last_cols || rows != self.last_rows {
            self.logo_cells.clear();
            self.splashes.clear();
            self.drops.clear();
            self.puddle = vec![0.0f32; cols];
            self.puddle_color = vec![(0u8, 0u8, 0u8); cols];

            // library 4.1: render the centered system logo from the live OS info
            // (replaces pre-4.1 `trance_core::logo_lines()` + `logo_dimensions()`).
            let logo_text = get_system_info().logo_text;
            let lines = render_logo_block(&logo_text, None);
            let logo_h = lines.len();
            let logo_w = lines.iter().map(|l| l.chars().count()).max().unwrap_or(0);
            let logo_x = cols.saturating_sub(logo_w) / 2;
            let logo_y = rows.saturating_sub(logo_h) / 2;

            for (r_offset, line) in lines.iter().enumerate().take(logo_h) {
                for (c_offset, ch) in line.chars().enumerate() {
                    if ch != ' ' {
                        self.logo_cells.push(LogoCell {
                            x: logo_x + c_offset,
                            y: logo_y + r_offset,
                            ch,
                            active: true,
                            glow: 0.0,
                            water: 0.0,
                        });
                    }
                }
            }

            self.phase = Phase::Complete;
            self.phase_timer = 0.0;
            self.last_cols = cols;
            self.last_rows = rows;
            let (bg, mid, fg) = Self::generate_scenery(&mut self.rng, cols, rows);
            self.bg_cells = bg;
            self.mid_scenery = mid;
            self.fg_scenery = fg;

            // Populate all perch points (Big Tree branch + top of logo cells)
            let tree_x = 8;
            let mut perch_points = Vec::new();
            perch_points.push((tree_x + 2, rows.saturating_sub(5)));
            for cell in &self.logo_cells {
                let has_above = self.logo_cells.iter().any(|c| c.x == cell.x && c.y == cell.y - 1);
                if !has_above && cell.y > 0 {
                    perch_points.push((cell.x, cell.y - 1));
                }
            }
            self.perch_points = perch_points;

            // Choose starting perch point
            if !self.perch_points.is_empty() {
                let p_idx = self.rng.next_usize(self.perch_points.len());
                self.bird_perch_x = self.perch_points[p_idx].0 as f32;
                self.bird_perch_y = self.perch_points[p_idx].1 as f32;
            } else {
                self.bird_perch_x = 0.0;
                self.bird_perch_y = 0.0;
            }
            self.bird_x = self.bird_perch_x;
            self.bird_y = self.bird_perch_y;
            self.bird_state = BirdState::Sitting;
            self.bird_timer = self.rng.next_range(5.0, 15.0);
            self.bird_wing_flap = false;
        }
    }
}
