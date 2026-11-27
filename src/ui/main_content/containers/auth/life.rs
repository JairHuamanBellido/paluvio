//! Conway's Game of Life on a wrapping (toroidal) grid.
//!
//! Each cell stores its age: `0` is dead, `n` means alive for `n`
//! generations. The panel maps age to opacity, so fresh cells look faint and
//! settled ones solid. The board starts as small classic patterns (gliders,
//! blinkers, toads...) scattered at random. When it dies out, freezes into
//! still lifes and blinkers, or runs too long, it reseeds.

/// Generations a board may repeat itself before it reseeds.
const STALE_LIMIT: u32 = 24;
/// Hard cap so wandering gliders do not keep one board forever.
const MAX_GENERATIONS: u32 = 900;
/// Board area (in cells) per scattered pattern.
const CELLS_PER_PATTERN: usize = 110;

/// Opacity steps for live cells, youngest first. Older cells stay at the last.
const OPACITY_BY_AGE: [f32; 7] = [0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 1.0];

/// Seed patterns as `(row, col)` offsets, all inside a 4x4 box. Gliders are
/// listed twice so they show up more often; they keep the board moving.
const PATTERNS: [&[(usize, usize)]; 8] = [
    // Glider.
    &[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
    &[(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)],
    // Blinker.
    &[(0, 0), (0, 1), (0, 2)],
    // Toad.
    &[(0, 1), (0, 2), (0, 3), (1, 0), (1, 1), (1, 2)],
    // Beacon.
    &[(0, 0), (0, 1), (1, 0), (2, 3), (3, 2), (3, 3)],
    // Block.
    &[(0, 0), (0, 1), (1, 0), (1, 1)],
    // Beehive.
    &[(0, 1), (0, 2), (1, 0), (1, 3), (2, 1), (2, 2)],
    // R-pentomino: grows chaotic for a while before it settles.
    &[(0, 1), (0, 2), (1, 0), (1, 1), (2, 1)],
];

pub struct Life {
    cols: usize,
    rows: usize,
    cells: Vec<u8>,
    scratch: Vec<u8>,
    rng: u32,
    generation: u32,
    /// Fingerprints of the last two boards, to spot period 1 and 2 loops.
    recent: [u64; 2],
    stale: u32,
}

impl Life {
    pub fn new(cols: usize, rows: usize, seed: u32) -> Self {
        let mut life = Self {
            cols,
            rows,
            cells: vec![0; cols * rows],
            scratch: vec![0; cols * rows],
            rng: seed.wrapping_mul(2_654_435_761) | 1,
            generation: 0,
            recent: [0; 2],
            stale: 0,
        };
        life.reseed();
        life
    }

    pub fn cols(&self) -> usize {
        self.cols
    }

    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn cells(&self) -> &[u8] {
        &self.cells
    }

    fn population(&self) -> usize {
        self.cells.iter().filter(|age| **age > 0).count()
    }

    /// xorshift32: small, fast, good enough for scattering patterns.
    fn next(&mut self) -> usize {
        self.rng ^= self.rng << 13;
        self.rng ^= self.rng >> 17;
        self.rng ^= self.rng << 5;
        self.rng as usize
    }

    /// Clears the board and scatters patterns at random spots and rotations.
    fn reseed(&mut self) {
        self.cells.fill(0);
        self.generation = 0;
        self.recent = [0; 2];
        self.stale = 0;
        if self.cols < 4 || self.rows < 4 {
            return;
        }

        let count = (self.cols * self.rows / CELLS_PER_PATTERN).max(3);
        for _ in 0..count {
            let pattern = PATTERNS[self.next() % PATTERNS.len()];
            let (row, col) = (self.next() % self.rows, self.next() % self.cols);
            // Bits pick transpose / flip rows / flip cols: 8 orientations.
            let orientation = self.next();
            for &(d_row, d_col) in pattern {
                let (mut d_row, mut d_col) = if orientation & 1 == 0 {
                    (d_row, d_col)
                } else {
                    (d_col, d_row)
                };
                if orientation & 2 != 0 {
                    d_row = 3 - d_row;
                }
                if orientation & 4 != 0 {
                    d_col = 3 - d_col;
                }
                let index = (row + d_row) % self.rows * self.cols + (col + d_col) % self.cols;
                self.cells[index] = 1;
            }
        }
    }

    /// Advances one generation (B3/S23), reseeding a board that went stale.
    pub fn step(&mut self) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let index = row * self.cols + col;
                let age = self.cells[index];
                self.scratch[index] = match (age > 0, self.neighbours(row, col)) {
                    (true, 2 | 3) => age.saturating_add(1),
                    (false, 3) => 1,
                    _ => 0,
                };
            }
        }
        std::mem::swap(&mut self.cells, &mut self.scratch);
        self.generation += 1;

        let fingerprint = self.fingerprint();
        if self.recent.contains(&fingerprint) {
            self.stale += 1;
        } else {
            self.stale = 0;
        }
        self.recent = [self.recent[1], fingerprint];

        if self.population() == 0 || self.stale > STALE_LIMIT || self.generation > MAX_GENERATIONS {
            self.reseed();
        }
    }

    fn neighbours(&self, row: usize, col: usize) -> u8 {
        let mut count = 0;
        for d_row in [self.rows - 1, 0, 1] {
            for d_col in [self.cols - 1, 0, 1] {
                if d_row == 0 && d_col == 0 {
                    continue;
                }
                let index = (row + d_row) % self.rows * self.cols + (col + d_col) % self.cols;
                count += u8::from(self.cells[index] > 0);
            }
        }
        count
    }

    /// Hash of which cells are alive (ages ignored).
    fn fingerprint(&self) -> u64 {
        self.cells
            .iter()
            .enumerate()
            .filter(|(_, age)| **age > 0)
            .fold(0xcbf2_9ce4_8422_2325, |hash, (index, _)| {
                (hash ^ index as u64).wrapping_mul(0x0100_0000_01b3)
            })
    }
}

/// Opacity for a cell of `age` (> 0): 40% when born, up to 100% when settled.
pub fn opacity(age: u8) -> f32 {
    let step = usize::from(age.max(1) - 1).min(OPACITY_BY_AGE.len() - 1);
    OPACITY_BY_AGE[step]
}

#[cfg(test)]
mod tests {
    use super::*;

    const COLS: usize = 24;
    const ROWS: usize = 32;

    fn empty() -> Life {
        let mut life = Life::new(COLS, ROWS, 0);
        life.cells.fill(0);
        life
    }

    fn set(life: &mut Life, cells: &[(usize, usize)]) {
        for (row, col) in cells {
            life.cells[row * COLS + col] = 1;
        }
    }

    fn alive(life: &Life) -> Vec<(usize, usize)> {
        (0..ROWS * COLS)
            .filter(|index| life.cells[*index] > 0)
            .map(|index| (index / COLS, index % COLS))
            .collect()
    }

    #[test]
    fn blinker_flips_between_row_and_column() {
        let mut life = empty();
        set(&mut life, &[(5, 4), (5, 5), (5, 6)]);
        life.step();
        assert_eq!(alive(&life), vec![(4, 5), (5, 5), (6, 5)]);
        life.step();
        assert_eq!(alive(&life), vec![(5, 4), (5, 5), (5, 6)]);
    }

    #[test]
    fn block_stays_and_ages() {
        let mut life = empty();
        let block = [(2, 2), (2, 3), (3, 2), (3, 3)];
        set(&mut life, &block);
        life.step();
        life.step();
        assert_eq!(alive(&life), block.to_vec());
        assert!(block.iter().all(|(r, c)| life.cells[r * COLS + c] == 3));
    }

    #[test]
    fn neighbours_wrap_around_edges() {
        let mut life = empty();
        // Blinker split across the left and right edges.
        set(&mut life, &[(5, COLS - 1), (5, 0), (5, 1)]);
        life.step();
        assert_eq!(alive(&life), vec![(4, 0), (5, 0), (6, 0)]);
    }

    #[test]
    fn new_board_has_patterns() {
        assert!(Life::new(COLS, ROWS, 3).population() > 0);
    }

    #[test]
    fn empty_board_reseeds() {
        let mut life = empty();
        life.step();
        assert!(life.population() > 0);
        assert_eq!(life.generation, 0);
    }

    #[test]
    fn tiny_board_stays_empty_without_panicking() {
        let mut life = Life::new(0, 0, 1);
        life.step();
        assert_eq!(life.population(), 0);
    }

    #[test]
    fn opacity_climbs_from_forty_to_full() {
        assert_eq!(opacity(1), 0.4);
        assert_eq!(opacity(4), 0.7);
        assert_eq!(opacity(7), 1.0);
        assert_eq!(opacity(200), 1.0);
    }
}
