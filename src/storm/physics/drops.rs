//! Rain drop updates, collision handling, and splash/puddle updates.

use crate::storm::Storm;
use crate::storm::types::{Drop, Splash, Phase};

impl Storm {
    pub fn update_drops(&mut self, delta: f32, cols: usize, rows: usize, speed_mult: f32) {
        // Adjust drop count (increased for heavy cold rain)
        let mut target_drops = match self.drop_count_opt {
            0 => (cols).clamp(20, 100),
            2 => (cols * 3).clamp(60, 400),
            _ => (cols * 2).clamp(40, 250),
        };
        if self.on_battery {
            target_drops = (target_drops as f32 * 0.55) as usize;
        }
        target_drops = (target_drops as f32 * self.quality_scale) as usize;

        if self.drops.len() < target_drops {
            while self.drops.len() < target_drops {
                let x = if self.phase == Phase::Building && self.rng.next_bool(0.6) {
                    let inactive: Vec<&crate::storm::types::LogoCell> = self.logo_cells.iter().filter(|c| !c.active).collect();
                    if !inactive.is_empty() {
                        let selected = inactive[self.rng.next_usize(inactive.len())];
                        selected.x as f32
                    } else {
                        self.rng.next_range(0.0, cols as f32)
                    }
                } else {
                    self.rng.next_range(0.0, cols as f32)
                };

                let is_bg = self.rng.next_bool(0.5); // 50% background rain
                let mut color = Self::cold_rain_color(&mut self.rng);
                if is_bg {
                    color = (
                        (color.0 as f32 * 0.35) as u8,
                        (color.1 as f32 * 0.35) as u8,
                        (color.2 as f32 * 0.35) as u8,
                    );
                }
                
                self.drops.push(Drop {
                    x,
                    y: -self.rng.next_range(1.0, rows as f32),
                    vy: self.rng.next_range(25.0, 45.0) * speed_mult * (if is_bg { 0.75 } else { 1.0 }),
                    color,
                    is_background: is_bg,
                });
            }
        } else if self.drops.len() > target_drops {
            self.drops.truncate(target_drops);
        }

        // Update drops position & collisions
        let mut drops = std::mem::take(&mut self.drops);
        for drop in &mut drops {
            drop.y += drop.vy * delta;

            // Wind drifts the drop horizontally
            drop.x += self.wind * delta;
            // Wrap horizontally around the screen columns
            if drop.x < 0.0 {
                drop.x += cols as f32;
            } else if drop.x >= cols as f32 {
                drop.x -= cols as f32;
            }

            let col = drop.x as usize;
            if drop.y >= 0.0 {
                let row = drop.y as usize;

                if col < cols && row < rows {
                    // Background drops do NOT collide with foreground elements
                    if !drop.is_background {
                        // Check if we hit any logo cell (whether active or inactive)
                        let mut hit = false;
                        for cell in &mut self.logo_cells {
                            if cell.x == col && cell.y == row {
                                if !cell.active && self.phase == Phase::Building {
                                    cell.active = true;
                                    cell.glow = 1.0;
                                }
                                
                                if cell.active {
                                    // Rain water piles up on the active OS/Kernel cells
                                    cell.water = (cell.water + 0.45).min(2.5);
                                    
                                    // Spawn splash particles
                                    let splash_count = if self.on_battery { 1 } else { 3 };
                                    let splash_count = (splash_count as f32 * self.quality_scale).max(1.0) as usize;
                                    for _ in 0..splash_count {
                                        self.splashes.push(Splash {
                                            x: col as f32,
                                            y: row as f32,
                                            vx: self.rng.next_range(-3.0, 3.0),
                                            vy: self.rng.next_range(-2.0, -0.5),
                                            life: 0.5,
                                            color: drop.color,
                                            is_background: false,
                                        });
                                    }

                                    // Reset drop
                                    let is_bg = self.rng.next_bool(0.5);
                                    let mut color = Self::cold_rain_color(&mut self.rng);
                                    if is_bg {
                                        color = (
                                            (color.0 as f32 * 0.35) as u8,
                                            (color.1 as f32 * 0.35) as u8,
                                            (color.2 as f32 * 0.35) as u8,
                                        );
                                    }
                                    drop.is_background = is_bg;
                                    drop.color = color;
                                    drop.y = -self.rng.next_range(1.0, rows as f32);
                                    drop.vy = self.rng.next_range(25.0, 45.0) * speed_mult * (if is_bg { 0.75 } else { 1.0 });
                                    hit = true;
                                    break;
                                }
                            }
                        }
                        if hit {
                            continue;
                        }
                    }
                }
            }

            // Reset drop if it falls off bottom
            if drop.y >= (rows as f32 - 1.0) && cols > 0 {
                let col = (drop.x as usize).min(cols - 1);
                
                // Foreground drops spawn floor splash particles and accumulate puddles
                if !drop.is_background {
                    let splash_count = if self.on_battery { 1 } else { 2 };
                    let splash_count = (splash_count as f32 * self.quality_scale).max(1.0) as usize;
                    for _ in 0..splash_count {
                        self.splashes.push(Splash {
                            x: col as f32,
                            y: (rows as f32 - 1.0),
                            vx: self.rng.next_range(-4.0, 4.0),
                            vy: self.rng.next_range(-3.0, -1.0),
                            life: self.rng.next_range(0.3, 0.6),
                            color: drop.color,
                            is_background: false,
                        });
                    }
                    
                    // Accumulate puddle on the floor
                    if col < self.puddle.len() {
                        self.puddle[col] = (self.puddle[col] + 0.38).min(3.0);
                        let p_col = self.puddle_color[col];
                        let drop_color = drop.color;
                        self.puddle_color[col] = (
                            (p_col.0 as f32 * 0.6 + drop_color.0 as f32 * 0.4) as u8,
                            (p_col.1 as f32 * 0.6 + drop_color.1 as f32 * 0.4) as u8,
                            (p_col.2 as f32 * 0.6 + drop_color.2 as f32 * 0.4) as u8,
                        );
                    }
                } else {
                    // Background splashes
                    for _ in 0..1 {
                        self.splashes.push(Splash {
                            x: col as f32,
                            y: (rows as f32 - 1.0),
                            vx: self.rng.next_range(-2.0, 2.0),
                            vy: self.rng.next_range(-1.5, -0.5),
                            life: self.rng.next_range(0.2, 0.4),
                            color: drop.color,
                            is_background: true,
                        });
                    }
                }

                let is_bg = self.rng.next_bool(0.5);
                let mut color = Self::cold_rain_color(&mut self.rng);
                if is_bg {
                    color = (
                        (color.0 as f32 * 0.35) as u8,
                        (color.1 as f32 * 0.35) as u8,
                        (color.2 as f32 * 0.35) as u8,
                    );
                }
                drop.is_background = is_bg;
                drop.color = color;
                drop.y = -self.rng.next_range(1.0, rows as f32);
                drop.vy = self.rng.next_range(25.0, 45.0) * speed_mult * (if is_bg { 0.75 } else { 1.0 });
            }
        }
        self.drops = drops;

        // Update splashes
        for s in &mut self.splashes {
            s.x += s.vx * delta;
            s.y += s.vy * delta;
            // Splashes are blown slightly by the wind
            s.vx += self.wind * delta * 0.25;
            s.vy += 9.8 * delta;
            s.life -= delta;
        }
        self.splashes.retain(|s| s.life > 0.0);

        // Decay logo cell water and glow
        for cell in &mut self.logo_cells {
            if cell.glow > 0.0 {
                cell.glow -= delta * 1.5;
            }
            if cell.water > 0.0 {
                cell.water -= delta * 0.45;
                if cell.water < 0.0 {
                    cell.water = 0.0;
                }
            }
        }

        // Decay/drain puddles on the ground
        for x in 0..cols {
            if x < self.puddle.len() && self.puddle[x] > 0.0 {
                self.puddle[x] -= delta * 0.28;
                if self.puddle[x] < 0.0 {
                    self.puddle[x] = 0.0;
                }
            }
        }
    }
}
