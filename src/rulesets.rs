use RuleSet;

/// eine sehr einfache Diffusion
pub struct Diffusion {}

impl RuleSet for Diffusion {
    type Cell = f32;
    
    fn step([[tl, t, tr],
             [l,  m, r ],
             [bl, b, br]]: [[f32; 3]; 3]) -> f32 {
        tl * 0.01 + t * 0.02 + tr * 0.01 +
         l * 0.02 + m * 0.88 +  r * 0.02 +
        bl * 0.01 + b * 0.02 + br * 0.01
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BinaryCell {
    Live,
    Dead,
}

impl Default for BinaryCell {
    fn default() -> Self { BinaryCell::Dead }
}

// das altbekannte Conway's Game of Life
pub struct GameOfLife {}

impl RuleSet for GameOfLife {
    type Cell = self::BinaryCell;
    
    fn step([[tl, t, tr],
             [l,  m, r ],
             [bl, b, br]]: [[BinaryCell; 3]; 3]) -> BinaryCell {
        use self::BinaryCell::*;
        let live_neighbors = [tl, t, tr, l, r, bl, b, br].iter().filter(|&&x| x == Live).count();
        match (m, live_neighbors) {
            (_, 3)    => Live,
            (Live, 2) => Live,
            _         => Dead,
        }
    }
}