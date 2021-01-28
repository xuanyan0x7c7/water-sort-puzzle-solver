pub type Tube = Vec<u8>;

pub struct TubeStats {
    pub empty: bool,
    pub simple: bool,
    pub finished: bool,
}

pub struct SolutionStep {
    pub from: usize,
    pub to: usize,
}

pub trait Solver {
    fn new(height: usize, colors: usize, initial_tubes: &Vec<Tube>) -> Self;
    fn search(&mut self) -> bool;
    fn get_solution(&self) -> Vec<SolutionStep>;
}

pub fn all_same(tube: &Tube) -> bool {
    if tube.is_empty() {
        return true;
    }
    let first = tube[0];
    for c in tube[1..].iter() {
        if *c != first {
            return false;
        }
    }
    true
}

pub fn is_solved(state: &Vec<Tube>, height: usize) -> bool {
    state
        .iter()
        .all(|tube| tube.is_empty() || (tube.len() == height && all_same(tube)))
}
