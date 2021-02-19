use super::utils::{get_transform, get_tube_stat, is_solved, pour, TubeStats};
use super::{SolutionStep, Solver};
use std::collections::HashSet;

#[derive(Clone)]
struct State {
    from: usize,
    to: usize,
    transform: Vec<usize>,
}

pub struct DFSSolver {
    height: usize,
    tubes: usize,
    initial_tubes: Vec<u8>,
    states: HashSet<Vec<u8>>,
    stack: Vec<State>,
}

impl Solver for DFSSolver {
    fn new(height: usize, initial_tubes: Vec<u8>) -> Self {
        Self {
            height,
            tubes: initial_tubes.len() / height,
            initial_tubes,
            states: HashSet::new(),
            stack: vec![],
        }
    }

    fn search(&mut self) -> bool {
        self.inner_search(&self.initial_tubes.clone(), usize::MAX, usize::MAX)
    }

    fn get_solution(&self) -> Vec<SolutionStep> {
        let mut solution = vec![];
        let mut transform: Vec<usize> = (0..self.tubes).collect();
        for (index, step) in self.stack.iter().enumerate() {
            if index > 0 {
                solution.push(SolutionStep {
                    from: transform[step.from],
                    to: transform[step.to],
                });
            }
            transform = (0..self.tubes)
                .map(|index| transform[step.transform[index]])
                .collect();
        }
        solution
    }
}

impl DFSSolver {
    fn inner_search(&mut self, state: &Vec<u8>, from: usize, to: usize) -> bool {
        let (transform, sorted_tubes) = get_transform(state, self.height, self.tubes);
        if self.states.contains(&sorted_tubes) {
            return false;
        }
        self.states.insert(sorted_tubes.clone());
        self.stack.push(State {
            from,
            to,
            transform,
        });
        if is_solved(&sorted_tubes, self.height) {
            return true;
        }
        let tube_stats: Vec<TubeStats> = sorted_tubes
            .chunks_exact(self.height)
            .map(|tube| get_tube_stat(tube, self.height))
            .collect();
        for i in 0..(self.tubes - 1) {
            if !tube_stats[i].simple || tube_stats[i].color_height == self.height {
                continue;
            }
            for j in (i + 1)..self.tubes {
                if tube_stats[j].simple && tube_stats[i].color == tube_stats[j].color {
                    let mut tubes = sorted_tubes.clone();
                    pour(
                        &mut tubes,
                        self.height,
                        &tube_stats,
                        i,
                        j,
                        tube_stats[i].size,
                    );
                    if self.inner_search(&tubes, i, j) {
                        return true;
                    } else {
                        self.stack.pop();
                        return false;
                    }
                }
            }
        }
        for i in 0..self.tubes {
            if !tube_stats[i].simple || tube_stats[i].color_height == self.height {
                continue;
            }
            for j in 0..self.tubes {
                if j == i || tube_stats[j].simple || tube_stats[i].color != tube_stats[j].color {
                    continue;
                }
                let amount = tube_stats[j].color_height;
                if tube_stats[i].color_height + amount == self.height {
                    let mut tubes = sorted_tubes.clone();
                    pour(&mut tubes, self.height, &tube_stats, j, i, amount);
                    if self.inner_search(&tubes, j, i) {
                        return true;
                    } else {
                        self.stack.pop();
                        return false;
                    }
                }
            }
        }
        for i in 0..(self.tubes - 1) {
            if tube_stats[i].color_height == self.height {
                continue;
            } else if tube_stats[i].color_height == 0 {
                if i > 0 {
                    continue;
                }
                for j in (i + 1)..self.tubes {
                    if tube_stats[j].simple || tube_stats[j].color_height == 0 {
                        continue;
                    }
                    let mut tubes = sorted_tubes.clone();
                    let amount = tube_stats[j].color_height;
                    pour(&mut tubes, self.height, &tube_stats, j, i, amount);
                    if self.inner_search(&tubes, j, i) {
                        return true;
                    }
                }
            } else {
                for j in (i + 1)..self.tubes {
                    if tube_stats[j].color_height < self.height
                        && tube_stats[i].color == tube_stats[j].color
                    {
                        let mut indexes = vec![];
                        if tube_stats[j].size < self.height {
                            indexes.push((i, j));
                        }
                        if tube_stats[i].size < self.height {
                            indexes.push((j, i));
                        }
                        for (x, y) in indexes {
                            let mut tubes = sorted_tubes.clone();
                            let amount = usize::min(
                                tube_stats[x].color_height,
                                self.height - tube_stats[y].size,
                            );
                            pour(&mut tubes, self.height, &tube_stats, x, y, amount);
                            if self.inner_search(&tubes, x, y) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        self.stack.pop();
        false
    }
}
