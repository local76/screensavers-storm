use crate::runner::core::TerminalCell;
use crate::storm::Storm;

impl Storm {
    pub(crate) fn draw_bg_cells(&self, grid: &mut [TerminalCell], cols: usize, rows: usize, bg_color: (u8, u8, u8)) {
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
    }

    pub(crate) fn draw_midground_scenery(&self, grid: &mut [TerminalCell], cols: usize, rows: usize, bg_color: (u8, u8, u8)) {
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
    }

    pub(crate) fn draw_foreground_scenery(&self, grid: &mut [TerminalCell], cols: usize, rows: usize, bg_color: (u8, u8, u8)) {
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
    }

    pub(crate) fn draw_puddles(&self, grid: &mut [TerminalCell], cols: usize, rows: usize, bg_color: (u8, u8, u8)) {
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
    }

    pub(crate) fn draw_subtitles(&self, grid: &mut [TerminalCell], cols: usize, rows: usize, bg_color: (u8, u8, u8)) {
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
