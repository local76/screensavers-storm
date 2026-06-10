//! Drawing and rendering implementation for the storm screensaver.

use library::core::TerminalCell;
use library::toolkit::sys_info::query_current_palette;

use crate::storm::Storm;
use crate::storm::types::{BirdState, AnimalType};

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
        for &(bx, by, bch, bcol) in &self.bg_cells {
            if bx < cols && by < rows {
                grid[by * cols + bx] = TerminalCell {
                    ch: bch,
                    fg: bcol,
                    bg: bg_color,
                    bold: false,
                };
            }
        }

        // 1. Background rain drops (is_background is true)
        let rain_char = if self.wind > 2.5 {
            '/'
        } else if self.wind < -2.5 {
            '\\'
        } else {
            '|'
        };

        for drop in &self.drops {
            if drop.is_background {
                let cx = drop.x as usize;
                let cy = drop.y as usize;
                if cx < cols && cy < rows {
                    grid[cy * cols + cx] = TerminalCell {
                        ch: rain_char,
                        fg: drop.color,
                        bg: bg_color,
                        bold: false,
                    };
                }
            }
        }

        // 1b. Background floor splashes (is_background is true)
        for s in &self.splashes {
            if s.is_background {
                let cx = s.x as usize;
                let cy = s.y as usize;
                if cx < cols && cy < rows {
                    let life_factor = (s.life * 2.0).min(1.0);
                    let fg = (
                        (s.color.0 as f32 * life_factor) as u8,
                        (s.color.1 as f32 * life_factor) as u8,
                        (s.color.2 as f32 * life_factor) as u8,
                    );
                    let ch = if s.vy < 0.0 {
                        'o'
                    } else if s.life > 0.35 {
                        '*'
                    } else if s.life > 0.18 {
                        '+'
                    } else {
                        '.'
                    };
                    grid[cy * cols + cx] = TerminalCell {
                        ch,
                        fg,
                        bg: bg_color,
                        bold: false,
                    };
                }
            }
        }

        // Helper to draw lightning bolts using thin characters
        let draw_lightning_bolts = |grid: &mut [TerminalCell], is_bg: bool| {
            if in_flash && self.lightning_is_background == is_bg {
                let bolt_fg = if is_bg { (210, 215, 240) } else { (255, 255, 255) };
                for bolt in &self.lightning_bolts {
                    for i in 0..bolt.len() {
                        let (lx, ly) = bolt[i];
                        if lx < cols && ly < rows {
                            let ch = if i == 0 {
                                if bolt.len() > 1 {
                                    let (nx, _) = bolt[1];
                                    if nx > lx { '\\' } else if nx < lx { '/' } else { '|' }
                                } else {
                                    '|'
                                }
                            } else {
                                let (px, _) = bolt[i - 1];
                                if px < lx { '\\' } else if px > lx { '/' } else { '|' }
                            };
                            grid[ly * cols + lx] = TerminalCell {
                                ch,
                                fg: bolt_fg,
                                bg: bg_color,
                                bold: true,
                            };
                        }
                    }
                }
            }
        };

        // 1c. Background lightning forks (lightning_is_background is true) using thin chars
        draw_lightning_bolts(grid, true);

        // 2. Midground scenery trees (mid_scenery)
        for &(mx, my, mch, mcol) in &self.mid_scenery {
            if mx < cols && my < rows {
                grid[my * cols + mx] = TerminalCell {
                    ch: mch,
                    fg: mcol,
                    bg: bg_color,
                    bold: false,
                };
            }
        }

        // 2b. Midground animals (draw Bigfoot as a 3-cell high entity)
        if let Some(ref animal) = self.active_animal {
            if animal.animal_type == AnimalType::Bigfoot {
                let ax = animal.x.round() as i32;
                let ay = animal.y.round() as i32;
                if ax >= 0 && ax < cols as i32 {
                    let ux = ax as usize;
                    // Head: '▲'
                    if ay >= 0 && ay < rows as i32 {
                        grid[(ay as usize) * cols + ux] = TerminalCell {
                            ch: '▲',
                            fg: (55, 45, 40),
                            bg: bg_color,
                            bold: false,
                        };
                    }
                    // Body: '█'
                    if ay + 1 >= 0 && ay + 1 < rows as i32 {
                        grid[((ay + 1) as usize) * cols + ux] = TerminalCell {
                            ch: '█',
                            fg: (55, 45, 40),
                            bg: bg_color,
                            bold: false,
                        };
                    }
                    // Legs: '╩' or '║'
                    if ay + 2 >= 0 && ay + 2 < rows as i32 {
                        let ch = if animal.frame_toggle { '╩' } else { '║' };
                        grid[((ay + 2) as usize) * cols + ux] = TerminalCell {
                            ch,
                            fg: (55, 45, 40),
                            bg: bg_color,
                            bold: false,
                        };
                    }
                }
            }
        }

        // 3. Foreground trees and Big Pine Tree (fg_scenery)
        for &(fx, fy, fch, fcol) in &self.fg_scenery {
            if fx < cols && fy < rows {
                grid[fy * cols + fx] = TerminalCell {
                    ch: fch,
                    fg: fcol,
                    bg: bg_color,
                    bold: false,
                };
            }
        }

        // Desaturate and cool down the accent color for a cold, miserable feel
        let cool_r = (accent.0 as f32 * 0.25 + 90.0 * 0.75) as u8;
        let cool_g = (accent.1 as f32 * 0.25 + 110.0 * 0.75) as u8;
        let cool_b = (accent.2 as f32 * 0.25 + 135.0 * 0.75) as u8;
        let cold_accent = (cool_r, cool_g, cool_b);

        // 4. Persistent logo cells (logo_cells) with dynamic rippling rain glows and puddle accumulation
        for cell in &self.logo_cells {
            if cell.active {
                if cell.x >= cols || cell.y >= rows {
                    continue;
                }
                // Active logo cells glow in bright white-blend, then fade to cold accent color base
                let glow_val = cell.glow.min(1.0);
                let fg_r = (cold_accent.0 as f32 + (255.0 - cold_accent.0 as f32) * glow_val * 0.7) as u8;
                let fg_g = (cold_accent.1 as f32 + (255.0 - cold_accent.1 as f32) * glow_val * 0.7) as u8;
                let fg_b = (cold_accent.2 as f32 + (255.0 - cold_accent.2 as f32) * glow_val * 0.7) as u8;
                grid[cell.y * cols + cell.x] = TerminalCell {
                    ch: cell.ch,
                    fg: (fg_r, fg_g, fg_b),
                    bg: bg_color,
                    bold: true,
                };
            }
        }

        for cell in &self.logo_cells {
            if cell.active && cell.water > 0.15 && cell.y > 0 {
                let above_x = cell.x;
                let above_y = cell.y - 1;
                if above_x >= cols || above_y >= rows {
                    continue;
                }
                let has_above_active = self.logo_cells.iter().any(|c| c.active && c.x == above_x && c.y == above_y);
                if !has_above_active {
                    let w_level = cell.water;
                    let ch = if w_level > 1.8 {
                        '~'
                    } else if w_level > 0.9 {
                        '_'
                    } else {
                        '.'
                    };
                    
                    grid[above_y * cols + above_x] = TerminalCell {
                        ch,
                        fg: (100, 135, 170),
                        bg: bg_color,
                        bold: w_level > 1.0,
                    };
                }
            }
        }

        // 5. Foreground animals (draw Bear as 2-cell high entity, Deer as 2-cell high entity)
        if let Some(ref animal) = self.active_animal {
            let ax = animal.x.round() as i32;
            let ay = animal.y.round() as i32;
            if ax >= 0 && ax < cols as i32 {
                let ux = ax as usize;
                match animal.animal_type {
                    AnimalType::Bear => {
                        // Head: '∩'
                        if ay >= 0 && ay < rows as i32 {
                            grid[(ay as usize) * cols + ux] = TerminalCell {
                                ch: '∩',
                                fg: (80, 55, 35),
                                bg: bg_color,
                                bold: false,
                            };
                        }
                        // Body: '█'
                        if ay + 1 >= 0 && ay + 1 < rows as i32 {
                            grid[((ay + 1) as usize) * cols + ux] = TerminalCell {
                                ch: '█',
                                fg: (80, 55, 35),
                                bg: bg_color,
                                bold: false,
                            };
                        }
                    }
                    AnimalType::Deer => {
                        // Antlers: '¥'
                        if ay >= 0 && ay < rows as i32 {
                            grid[(ay as usize) * cols + ux] = TerminalCell {
                                ch: '¥',
                                fg: (160, 110, 65),
                                bg: bg_color,
                                bold: false,
                            };
                        }
                        // Legs: '╩' or '╚'
                        if ay + 1 >= 0 && ay + 1 < rows as i32 {
                            let ch = if animal.frame_toggle { '╩' } else { '╚' };
                            grid[((ay + 1) as usize) * cols + ux] = TerminalCell {
                                ch,
                                fg: (160, 110, 65),
                                bg: bg_color,
                                bold: false,
                            };
                        }
                    }
                    _ => {}
                }
            }
        }

        // 6. Bird rendering (render Sitting, Scared, Flying, or Explores wing flaps)
        if self.bird_state != BirdState::Dead {
            let bx = self.bird_x.round() as i32;
            let by = self.bird_y.round() as i32;
            if bx >= 0 && bx < cols as i32 && by >= 0 && by < rows as i32 {
                let ubx = bx as usize;
                let uby = by as usize;
                let ch = match self.bird_state {
                    BirdState::Sitting => 'v',
                    BirdState::Scared => 'V',
                    BirdState::Flying => if self.bird_wing_flap { 'w' } else { 'v' },
                    BirdState::Explores => if self.bird_wing_flap { 'W' } else { 'V' },
                    _ => 'v',
                };
                
                let fg = match self.bird_state {
                    BirdState::Scared => (255, 100, 100),
                    BirdState::Explores => {
                        let flash_cycle = (self.phase_timer * 15.0) as usize % 3;
                        match flash_cycle {
                            0 => (255, 255, 255),
                            1 => (255, 235, 140),
                            _ => (100, 220, 255),
                        }
                    }
                    _ => (200, 210, 230),
                };
                
                grid[uby * cols + ubx] = TerminalCell {
                    ch,
                    fg,
                    bg: bg_color,
                    bold: self.bird_state == BirdState::Scared || self.bird_state == BirdState::Explores,
                };
                
                // Draw chirp alert above it if scared
                if self.bird_state == BirdState::Scared && uby > 0 {
                    let alert_str = "Chirp!";
                    for (i, ach) in alert_str.chars().enumerate() {
                        let ax = ubx + i;
                        if ax < cols {
                            grid[(uby - 1) * cols + ax] = TerminalCell {
                                ch: ach,
                                fg: (255, 200, 100),
                                bg: bg_color,
                                bold: true,
                            };
                        }
                    }
                }
            }
        }

        // 7. Floor puddles (puddle and puddle_color)
        for x in 0..cols {
            if x < self.puddle.len() && self.puddle[x] > 0.05 {
                let p_level = self.puddle[x];
                let ch = if p_level > 1.8 {
                    '█'
                } else if p_level > 0.9 {
                    '▄'
                } else {
                    '_'
                };
                
                let col = self.puddle_color[x];
                let intensity = (p_level / 2.0).min(1.0);
                let fg = (
                    (col.0 as f32 * intensity) as u8,
                    (col.1 as f32 * intensity) as u8,
                    (col.2 as f32 * intensity) as u8,
                );
                
                let y = rows - 1;
                grid[y * cols + x] = TerminalCell {
                    ch,
                    fg,
                    bg: bg_color,
                    bold: p_level > 1.0,
                };
            }
        }

        // 8. Foreground rain drops (is_background is false)
        for drop in &self.drops {
            if !drop.is_background {
                let cx = drop.x as usize;
                let cy = drop.y as usize;
                if cx < cols && cy < rows {
                    grid[cy * cols + cx] = TerminalCell {
                        ch: rain_char,
                        fg: drop.color,
                        bg: bg_color,
                        bold: false,
                    };
                }
            }
        }

        // 8b. Foreground splashes/sparks (is_background is false)
        for s in &self.splashes {
            if !s.is_background {
                let cx = s.x as usize;
                let cy = s.y as usize;
                if cx < cols && cy < rows {
                    let life_factor = (s.life * 2.0).min(1.0);
                    let fg = (
                        (s.color.0 as f32 * life_factor) as u8,
                        (s.color.1 as f32 * life_factor) as u8,
                        (s.color.2 as f32 * life_factor) as u8,
                    );
                    let ch = if s.vy < 0.0 {
                        'o'
                    } else if s.life > 0.35 {
                        '*'
                    } else if s.life > 0.18 {
                        '+'
                    } else {
                        '.'
                    };
                    grid[cy * cols + cx] = TerminalCell {
                        ch,
                        fg,
                        bg: bg_color,
                        bold: false,
                    };
                }
            }
        }

        // 8c. Foreground lightning forks (lightning_is_background is false) using thin chars
        draw_lightning_bolts(grid, false);

        // 9. Subtitles rendering centered at rows - 2 in warm white-yellow (230, 230, 200)
        if !self.subtitle.is_empty() && rows > 2 {
            let sub_y = rows - 2;
            let chars: Vec<char> = self.subtitle.chars().collect();
            let start_x = cols.saturating_sub(chars.len()) / 2;
            for (i, &ch) in chars.iter().enumerate() {
                let cx = start_x + i;
                if cx < cols {
                    grid[sub_y * cols + cx] = TerminalCell {
                        ch,
                        fg: (230, 230, 200),
                        bg: bg_color,
                        bold: true,
                    };
                }
            }
        }
    }
}
