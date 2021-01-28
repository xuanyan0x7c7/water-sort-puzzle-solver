use crate::solver::base::{
    all_same, is_solved as is_game_solved, SolutionStep, Solver, Tube, TubeStats,
};
use std::collections::HashSet;

struct State {
    tubes: Vec<Tube>,
    from: usize,
    to: usize,
}

#[derive(Clone)]
struct Step {
    from: usize,
    to: usize,
    transform: Vec<usize>,
}

pub struct DFSSolver {
    height: usize,
    tubes: usize,
    initial_tubes: Vec<Tube>,
    states: HashSet<Vec<Tube>>,
    stack: Vec<Step>,
}

impl Solver for DFSSolver {
    fn new(height: usize, _colors: usize, initial_tubes: &Vec<Tube>) -> Self {
        Self {
            height,
            tubes: initial_tubes.len(),
            initial_tubes: initial_tubes.clone(),
            states: HashSet::new(),
            stack: vec![],
        }
    }

    fn search(&mut self) -> bool {
        self.inner_search(&State {
            tubes: self.initial_tubes.clone(),
            from: usize::MAX,
            to: usize::MAX,
        })
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
    #[inline]
    fn is_solved(&self, state: &Vec<Tube>) -> bool {
        is_game_solved(state, self.height)
    }

    fn inner_search(&mut self, state: &State) -> bool {
        let mut transform: Vec<usize> = (0..self.tubes).collect();
        transform.sort_unstable_by_key(|index| &state.tubes[*index]);
        let sorted_tubes: Vec<Tube> = transform
            .iter()
            .map(|index| state.tubes[*index].clone())
            .collect();
        if self.states.get(&sorted_tubes).is_some() {
            return false;
        }
        self.states.insert(sorted_tubes.clone());
        self.stack.push(Step {
            from: state.from,
            to: state.to,
            transform: transform,
        });
        if self.is_solved(&sorted_tubes) {
            return true;
        }
        let tube_stats: Vec<TubeStats> = sorted_tubes
            .iter()
            .map(|tube| {
                let is_simple = all_same(tube);
                TubeStats {
                    empty: tube.is_empty(),
                    simple: is_simple,
                    finished: is_simple && tube.len() == self.height,
                }
            })
            .collect();
        for i in 0..(self.tubes - 1) {
            if tube_stats[i].empty || !tube_stats[i].simple || tube_stats[i].finished {
                continue;
            }
            for j in (i + 1)..self.tubes {
                if tube_stats[j].simple && sorted_tubes[i][0] == sorted_tubes[j][0] {
                    let mut tubes = sorted_tubes.clone();
                    tubes[i].clear();
                    tubes[j].resize(
                        sorted_tubes[i].len() + sorted_tubes[j].len(),
                        sorted_tubes[j][0],
                    );
                    if self.inner_search(&State {
                        tubes,
                        from: i,
                        to: j,
                    }) {
                        return true;
                    } else {
                        self.stack.pop();
                        return false;
                    }
                }
            }
        }
        for i in 0..(self.tubes - 1) {
            if tube_stats[i].finished {
                continue;
            }
            for j in (i + 1)..self.tubes {
                if tube_stats[j].finished {
                    continue;
                }
                if tube_stats[i].empty {
                    if tube_stats[j].simple {
                        continue;
                    }
                    let mut tubes = sorted_tubes.clone();
                    let color = *sorted_tubes[j].last().unwrap();
                    let mut amount = 1usize;
                    let mut offset = tubes[j].len() - amount;
                    while offset > 0 && tubes[j][offset - 1] == color {
                        amount += 1;
                        offset -= 1;
                    }
                    tubes[i].resize(amount, color);
                    tubes[j].truncate(offset);
                    if self.inner_search(&State {
                        tubes: tubes,
                        from: j,
                        to: i,
                    }) {
                        return true;
                    }
                } else if sorted_tubes[i].last() == sorted_tubes[j].last() {
                    let color = *sorted_tubes[i].last().unwrap();
                    let mut indexes = vec![];
                    if sorted_tubes[j].len() < self.height {
                        indexes.push((i, j));
                    }
                    if sorted_tubes[i].len() < self.height {
                        indexes.push((j, i));
                    }
                    for (i, j) in indexes {
                        let mut tubes = sorted_tubes.clone();
                        let mut offset_i = tubes[i].len() - 1;
                        let mut offset_j = tubes[j].len() + 1;
                        while offset_i > 0
                            && tubes[i][offset_i - 1] == color
                            && offset_j < self.height
                        {
                            offset_i -= 1;
                            offset_j += 1;
                        }
                        tubes[i].truncate(offset_i);
                        tubes[j].resize(offset_j, color);
                        if self.inner_search(&State {
                            tubes,
                            from: i,
                            to: j,
                        }) {
                            return true;
                        }
                    }
                }
            }
        }
        self.stack.pop();
        false
    }
}
