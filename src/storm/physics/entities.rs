//! Bird, scenery generation, and animal logic and updates.

use library::core::LcgRng;
use crate::storm::Storm;
use crate::storm::types::{BirdState, Splash, SceneryCell, Animal, AnimalType, AnimalState};

impl Storm {
    pub(crate) fn generate_scenery(rng: &mut LcgRng, cols: usize, rows: usize) -> (
        Vec<SceneryCell>,
        Vec<SceneryCell>,
        Vec<SceneryCell>,
    ) {
        let mut bg = Vec::new();
        let mut mid = Vec::new();
        let mut fg = Vec::new();
        if rows < 10 { return (bg, mid, fg); }

        let mountain_color = (18, 22, 28);
        let snow_color = (65, 75, 85);
        
        let mut mountain_heights = vec![0; cols];
        for (x, height) in mountain_heights.iter_mut().enumerate().take(cols) {
            let h = (rows as f32 * 0.20) + 
                    (x as f32 * 0.04).sin() * (rows as f32 * 0.08) + 
                    (x as f32 * 0.10).cos() * (rows as f32 * 0.03);
            *height = h.clamp(2.0, rows as f32 * 0.45) as usize;
        }

        for (x, &m_h) in mountain_heights.iter().enumerate().take(cols) {
            let peak_y = rows.saturating_sub(m_h + 3);
            
            for y in peak_y..rows.saturating_sub(2) {
                let ch = if y == peak_y {
                    if rng.next_bool(0.5) { '^' } else { '/' }
                } else if y == peak_y + 1 {
                    '.'
                } else {
                    ' '
                };
                
                let col = if y == peak_y { snow_color } else { mountain_color };
                if ch != ' ' {
                    bg.push((x, y, ch, col));
                }
            }
        }

        let mut bx = 4;
        while bx < cols - 4 {
            let base_y = rows.saturating_sub(3);
            let bg_h = rng.next_range(1.0, 3.0) as usize;
            let bg_tree_color = (rng.next_range(16.0, 24.0) as u8, rng.next_range(24.0, 32.0) as u8, rng.next_range(20.0, 28.0) as u8);
            let bg_trunk_color = (24, 20, 16);
            
            bg.push((bx, base_y, '|', bg_trunk_color));
            if bg_h > 1 {
                bg.push((bx, base_y - 1, '|', bg_trunk_color));
            }
            
            let foliage_top = base_y.saturating_sub(bg_h);
            bg.push((bx, foliage_top, '▲', bg_tree_color));
            if bg_h > 1 {
                bg.push((bx - 1, foliage_top + 1, '▲', bg_tree_color));
                bg.push((bx + 1, foliage_top + 1, '▲', bg_tree_color));
            }
            bx += rng.next_range(6.0, 14.0) as usize;
        }

        let mid_tree_color = (25, 38, 28);
        let mid_trunk_color = (32, 28, 24);
        
        let mut mx = 12;
        while mx < cols - 8 {
            let tree_h = rng.next_range(2.0, 3.5) as usize;
            let base_y = rows.saturating_sub(3);
            
            mid.push((mx, base_y, '║', mid_trunk_color));
            for h_offset in 1..=tree_h {
                let foliage_y = base_y.saturating_sub(h_offset);
                mid.push((mx, foliage_y, '▲', mid_tree_color));
                if h_offset > 1 {
                    if mx > 0 { mid.push((mx - 1, foliage_y, '▲', mid_tree_color)); }
                    if mx < cols - 1 { mid.push((mx + 1, foliage_y, '▲', mid_tree_color)); }
                }
            }
            mx += rng.next_range(8.0, 15.0) as usize;
        }

        let fg_tree_color = (35, 55, 40);
        let trunk_color = (48, 42, 36);
        
        let mut fx = cols.saturating_sub(22);
        while fx < cols - 3 {
            let tree_h = rng.next_range(2.0, 4.0) as usize;
            let base_y = rows.saturating_sub(3);
            
            fg.push((fx, base_y, '║', trunk_color));
            
            for h_offset in 1..=tree_h {
                let foliage_y = base_y.saturating_sub(h_offset);
                fg.push((fx, foliage_y, '▲', fg_tree_color));
                if h_offset > 1 {
                    if fx > 0 { fg.push((fx - 1, foliage_y, '▲', fg_tree_color)); }
                    if fx < cols - 1 { fg.push((fx + 1, foliage_y, '▲', fg_tree_color)); }
                }
            }
            fx += rng.next_range(7.0, 12.0) as usize;
        }

        let tree_x = 8;
        if cols > 20 {
            let base_y = rows.saturating_sub(3);
            let trunk_top = base_y.saturating_sub(4);
            for y in trunk_top..=base_y {
                fg.push((tree_x, y, '║', trunk_color));
            }
            let branch_y = base_y.saturating_sub(2);
            fg.push((tree_x + 1, branch_y, '═', trunk_color));
            fg.push((tree_x + 2, branch_y, '═', trunk_color));
            
            let foliage_base = base_y.saturating_sub(4);
            fg.push((tree_x, foliage_base - 2, '▲', fg_tree_color));
            for dx in -1..=1 {
                fg.push(((tree_x as i32 + dx) as usize, foliage_base - 1, '▲', fg_tree_color));
            }
            for dx in -2..=2 {
                fg.push(((tree_x as i32 + dx) as usize, foliage_base, '▲', fg_tree_color));
            }
        }

        (bg, mid, fg)
    }

    pub fn update_bird(&mut self, delta: f32, cols: usize, rows: usize) {
        match self.bird_state {
            BirdState::Sitting => {
                self.bird_x = self.bird_perch_x;
                self.bird_y = self.bird_perch_y;
                self.bird_timer -= delta;
                if self.bird_timer <= 0.0 {
                    self.bird_state = BirdState::Flying;
                    self.bird_timer = self.rng.next_range(6.0, 12.0);
                    self.bird_vx = self.rng.next_range(4.0, 8.0);
                    self.bird_vy = self.rng.next_range(-4.0, -1.5);
                    if self.rng.next_bool(0.15) && self.subtitle_timer <= 0.0 {
                        self.subtitle = "[Bird took off]".to_string();
                        self.subtitle_timer = 1.5;
                    }
                }
            }
            BirdState::Flying => {
                self.bird_timer -= delta;
                self.bird_x += self.bird_vx * delta;
                self.bird_y += self.bird_vy * delta;

                if self.rng.next_bool(0.25) {
                    self.bird_wing_flap = !self.bird_wing_flap;
                }

                if self.bird_x >= cols as f32 || self.bird_y < 0.0 || self.bird_timer <= 0.0 {
                    self.bird_state = BirdState::Dead;
                    self.bird_timer = self.rng.next_range(8.0, 18.0);
                }
            }
            BirdState::Scared => {
                self.bird_timer -= delta;
                if self.bird_timer <= 0.0 {
                    self.bird_state = BirdState::Flying;
                    self.bird_timer = 4.0;
                    
                    let current_x = self.bird_x;
                    let current_y = self.bird_y;
                    self.bird_vx = self.rng.next_range(9.0, 15.0);
                    self.bird_vy = self.rng.next_range(-6.0, -3.0);
                    
                    if (current_x - self.bird_perch_x).abs() < 1.0 && (current_y - self.bird_perch_y).abs() < 1.0 {
                        self.bird_x = self.bird_perch_x + 1.0;
                    }
                }
            }
            BirdState::Explores => {
                self.bird_timer -= delta;
                self.bird_x += self.bird_vx * delta;
                self.bird_y += self.bird_vy * delta;

                if self.rng.next_bool(0.18) {
                    self.bird_vx = self.rng.next_range(-25.0, 25.0);
                    self.bird_vy = self.rng.next_range(-20.0, 20.0);
                }

                if self.bird_x < 1.0 {
                    self.bird_x = 1.0;
                    self.bird_vx = -self.bird_vx * 0.8;
                } else if self.bird_x >= (cols as f32 - 1.0) {
                    self.bird_x = cols as f32 - 2.0;
                    self.bird_vx = -self.bird_vx * 0.8;
                }

                if self.bird_y < 1.0 {
                    self.bird_y = 1.0;
                    self.bird_vy = -self.bird_vy * 0.8;
                } else if self.bird_y >= (rows as f32 - 1.0) {
                    self.bird_y = rows as f32 - 2.0;
                    self.bird_vy = -self.bird_vy * 0.8;
                }

                if self.rng.next_bool(0.25) {
                    self.bird_wing_flap = !self.bird_wing_flap;
                }

                if self.rng.next_bool(0.6) {
                    self.splashes.push(Splash {
                        x: self.bird_x,
                        y: self.bird_y,
                        vx: self.rng.next_range(-4.0, 4.0),
                        vy: self.rng.next_range(-4.0, 4.0),
                        life: self.rng.next_range(0.3, 0.6),
                        color: if self.rng.next_bool(0.5) { (255, 235, 140) } else { (100, 220, 255) },
                        is_background: false,
                    });
                }

                if self.bird_timer <= 0.0 {
                    self.bird_state = BirdState::Dead;
                    self.bird_timer = 8.0;
                    self.subtitle = "[Subtitles: Bird Vaporized!]".to_string();
                    self.subtitle_timer = 2.0;
                    
                    for _ in 0..20 {
                        self.splashes.push(Splash {
                            x: self.bird_x,
                            y: self.bird_y,
                            vx: self.rng.next_range(-15.0, 15.0),
                            vy: self.rng.next_range(-12.0, -1.0),
                            life: self.rng.next_range(0.5, 1.2),
                            color: (50, 50, 55),
                            is_background: false,
                        });
                        self.splashes.push(Splash {
                            x: self.bird_x,
                            y: self.bird_y,
                            vx: self.rng.next_range(-18.0, 18.0),
                            vy: self.rng.next_range(-14.0, 2.0),
                            life: self.rng.next_range(0.4, 0.9),
                            color: (255, 235, 140),
                            is_background: false,
                        });
                    }
                }
            }
            BirdState::Dead => {
                self.bird_timer -= delta;
                if self.bird_timer <= 0.0 {
                    if !self.perch_points.is_empty() {
                        let p_idx = self.rng.next_usize(self.perch_points.len());
                        self.bird_perch_x = self.perch_points[p_idx].0 as f32;
                        self.bird_perch_y = self.perch_points[p_idx].1 as f32;
                    }
                    self.bird_state = BirdState::Sitting;
                    self.bird_timer = self.rng.next_range(5.0, 15.0);
                    self.bird_x = self.bird_perch_x;
                    self.bird_y = self.bird_perch_y;
                }
            }
        }
    }

    pub fn update_scenery_and_animals(&mut self, delta: f32, cols: usize, rows: usize) {
        self.animal_spawn_timer -= delta;
        if self.animal_spawn_timer <= 0.0 && self.active_animal.is_none() {
            self.animal_spawn_timer = self.rng.next_range(25.0, 50.0);
            let roll = self.rng.next_range(0.0, 1.0);
            let animal_type = if roll < 0.50 {
                AnimalType::Deer
            } else if roll < 0.85 {
                AnimalType::Bear
            } else {
                AnimalType::Bigfoot
            };

            let spawn_left = self.rng.next_bool(0.5);
            let base_speed = match animal_type {
                AnimalType::Deer => 4.5f32,
                AnimalType::Bear => 1.8f32,
                AnimalType::Bigfoot => 1.2f32,
            };

            let ay = if animal_type == AnimalType::Bigfoot {
                rows.saturating_sub(4) as f32
            } else {
                rows.saturating_sub(3) as f32
            };

            self.active_animal = Some(Animal {
                x: if spawn_left { -3.0 } else { cols as f32 + 3.0 },
                y: ay,
                vx: if spawn_left { base_speed } else { -base_speed },
                animal_type,
                state: AnimalState::Walking,
                timer: self.rng.next_range(5.0, 9.0),
                frame_toggle: false,
            });

            self.subtitle = match animal_type {
                AnimalType::Deer => "[A deer wanders out of the forest]".to_string(),
                AnimalType::Bear => "[A heavy brown bear walks out of the woods]".to_string(),
                AnimalType::Bigfoot => "[Unidentified creature rustling in the midground trees...]".to_string(),
            };
            self.subtitle_timer = 3.0;
        }

        if let Some(ref mut animal) = self.active_animal {
            animal.timer -= delta;
            
            if self.rng.next_bool(0.08) {
                animal.frame_toggle = !animal.frame_toggle;
            }

            match animal.state {
                AnimalState::Walking => {
                    animal.x += animal.vx * delta;
                    if animal.timer <= 0.0 {
                        if self.rng.next_bool(0.40) && animal.animal_type != AnimalType::Bigfoot {
                            animal.state = AnimalState::Idle;
                            animal.timer = self.rng.next_range(3.0, 6.0);
                            self.subtitle = match animal.animal_type {
                                AnimalType::Deer => "[Deer grazing on mossy ground]".to_string(),
                                AnimalType::Bear => "[Bear sitting down to rest]".to_string(),
                                _ => "".to_string(),
                            };
                            self.subtitle_timer = 2.0;
                        } else {
                            animal.timer = self.rng.next_range(4.0, 8.0);
                        }
                    }
                }
                AnimalState::Idle => {
                    if animal.timer <= 0.0 {
                        animal.state = AnimalState::Walking;
                        animal.timer = self.rng.next_range(4.0, 8.0);
                        self.subtitle = match animal.animal_type {
                            AnimalType::Deer => "[Deer walks on]".to_string(),
                            AnimalType::Bear => "[Bear lumbering forward]".to_string(),
                            _ => "".to_string(),
                        };
                        self.subtitle_timer = 1.8;
                    }
                }
                AnimalState::Startled => {
                    if animal.timer <= 0.0 {
                        animal.state = AnimalState::WalkingOff;
                        let run_speed = match animal.animal_type {
                            AnimalType::Deer => 11.0f32,
                            AnimalType::Bear => 6.0f32,
                            _ => 3.0f32,
                        };
                        animal.vx = animal.vx.signum() * run_speed;
                    }
                }
                AnimalState::WalkingOff => {
                    animal.x += animal.vx * delta;
                }
            }
        }

        if let Some(ref animal) = self.active_animal {
            if animal.x < -6.0 || animal.x > cols as f32 + 6.0 {
                self.active_animal = None;
            }
        }

        if self.subtitle_timer > 0.0 {
            self.subtitle_timer -= delta;
            if self.subtitle_timer <= 0.0 {
                self.subtitle.clear();
            }
        }

        if self.wind.abs() > 8.5 && self.rng.next_bool(0.01) && self.subtitle_timer <= 0.0 {
            self.subtitle = "Warning: Severe gale force wind gusts".to_string();
            self.subtitle_timer = 2.5;
        }
    }
}
