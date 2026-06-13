//! Drawing and rendering implementation for the storm screensaver.

pub mod helpers;
pub mod rain_lightning;
pub mod entities;

use crate::runner::core::TerminalCell;
use crate::runner::toolkit::sys_info::query_current_palette;

use crate::storm::Storm;

impl Storm {
    pub fn draw_impl(&self, grid: &mut [TerminalCell], cols: usize, rows: usize) {
        // library 4.0: pull the accent per-frame from the canonical
        // ScreenPalette (the pre-4.0 `get_theme_accent()` is a Windows
        // registry read; the library helper is cross-platform + cached).
        let accent = query_current_palette().accent;
        
        let in_flash = self.lightning_flash > 0.0;
        // Background sky flashes light-blue/grey during lightning strikes
        let bg_color = if in_flash { (30, 36, 48) } else { (0, 0, 0) };

        // Initialize grid cells to clear space and apply flash background color
        for cell in grid.iter_mut() {
            cell.ch = ' ';
            cell.fg = (0, 0, 0);
            cell.bg = bg_color;
            cell.bold = false;
        }

        // 0. Distant mountains & background trees (bg_cells)
        self.draw_bg_cells(grid, cols, rows, bg_color);

        // 1. Background rain drops (is_background is true)
        let rain_char = if self.wind > 2.5 {
            '/'
        } else if self.wind < -2.5 {
            '\\'
        } else {
            '|'
        };

        self.draw_rain(grid, cols, rows, bg_color, rain_char, true);

        // 1b. Background floor splashes (is_background is true)
        self.draw_splashes(grid, cols, rows, bg_color, true);

        // 1c. Background lightning forks (lightning_is_background is true) using thin chars
        self.draw_lightning_bolts(grid, cols, rows, bg_color, in_flash, true);

        // 2. Midground scenery trees (mid_scenery)
        self.draw_midground_scenery(grid, cols, rows, bg_color);

        // 2b. Midground animals (draw Bigfoot as a 3-cell high entity)
        self.draw_midground_animals(grid, cols, rows, bg_color);

        // 3. Foreground trees and Big Pine Tree (fg_scenery)
        self.draw_foreground_scenery(grid, cols, rows, bg_color);

        // Desaturate and cool down the accent color for a cold, miserable feel
        let cool_r = (accent.0 as f32 * 0.25 + 90.0 * 0.75) as u8;
        let cool_g = (accent.1 as f32 * 0.25 + 110.0 * 0.75) as u8;
        let cool_b = (accent.2 as f32 * 0.25 + 135.0 * 0.75) as u8;
        let cold_accent = (cool_r, cool_g, cool_b);

        // 4. Persistent logo cells (logo_cells) with dynamic rippling rain glows and puddle accumulation
        self.draw_logo_cells(grid, cols, rows, bg_color, cold_accent);

        // 5. Foreground animals (draw Bear as 2-cell high entity, Deer as 2-cell high entity)
        self.draw_foreground_animals(grid, cols, rows, bg_color);

        // 6. Bird rendering (render Sitting, Scared, Flying, or Explores wing flaps)
        self.draw_bird(grid, cols, rows, bg_color);

        // 7. Floor puddles (puddle and puddle_color)
        self.draw_puddles(grid, cols, rows, bg_color);

        // 8. Foreground rain drops (is_background is false)
        self.draw_rain(grid, cols, rows, bg_color, rain_char, false);

        // 8b. Foreground splashes/sparks (is_background is false)
        self.draw_splashes(grid, cols, rows, bg_color, false);

        // 8c. Foreground lightning forks (lightning_is_background is false) using thin chars
        self.draw_lightning_bolts(grid, cols, rows, bg_color, in_flash, false);

        // 9. Subtitles rendering centered at rows - 2 in warm white-yellow (230, 230, 200)
        self.draw_subtitles(grid, cols, rows, bg_color);
    }
}
