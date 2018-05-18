pub trait RuleSet {
    type Cell : Default + Copy + PartialEq + Send + Sync;
    fn step(neighborhood: [[<Self as RuleSet>::Cell; 3]; 3]) -> <Self as RuleSet>::Cell; 
}

pub trait CellWorld<R : RuleSet> {
    /// Leg ein neues Gitter mit der angegebenen Höhe und Breite an.
    fn new(width: usize, height: usize) -> Self;

    /// Gib die Breite des Gitters aus
    fn width(&self) -> usize;
    /// Gib die Höhe des Gitters aus
    fn height(&self) -> usize;

    /// Setz den Wert der Zelle an der angegebenen Position auf `value`
    /// Bei Koordinaten außerhalb des Gitters: beliebiges, safes Verhalten (z.b. panic, no-op)
    fn set_cell(&mut self, x: usize, y: usize, value: R::Cell);

    /// Gib der Wert der Zelle an der angegebenen Position aus.
    /// Bei Koordinaten außerhalb des Gitters: beliebiges, safes Verhalten (z.b. panic, beliebiger return value)
    fn get_cell(&self, x: usize, y: usize) -> R::Cell;

    /// Wende das Ruleset einmal auf das ganze Gitter an.
    fn step(&mut self);

    /// Wende das Ruleset `n`-mal auf das ganze Gitter an.
    /// Falls dir keine tollen Optimisationen einfallen, gibt es eine simple default-Implementation
    fn step_many(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }
}