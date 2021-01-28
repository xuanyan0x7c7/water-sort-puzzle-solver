use crate::solver::base::{is_solved as is_game_solved, SolutionStep, Solver, Tube};
use std::collections::HashSet;

struct TubeStats {
    simple: bool,
    color: Option<u8>,
    color_height: usize,
}

#[derive(Clone)]
struct State {
    from: usize,
    to: usize,
    transform: Vec<usize>,
}

pub struct DFSSolver {
    height: usize,
    tubes: usize,
    initial_tubes: Vec<Tube>,
    states: HashSet<Vec<Tube>>,
    stack: Vec<State>,
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
    #[inline]
    fn is_solved(&self, state: &Vec<Tube>) -> bool {
        is_game_solved(state, self.height)
    }

    fn inner_search(&mut self, state: &Vec<Tube>, from: usize, to: usize) -> bool {
        let mut transform: Vec<usize> = (0..self.tubes).collect();
        transform.sort_unstable_by_key(|index| &state[*index]);
        let sorted_tubes: Vec<Tube> = transform
            .iter()
            .map(|index| state[*index].clone())
            .collect();
        if self.states.get(&sorted_tubes).is_some() {
            return false;
        }
        self.states.insert(sorted_tubes.clone());
        self.stack.push(State {
            from,
            to,
            transform,
        });
        if self.is_solved(&sorted_tubes) {
            return true;
        }
        let tube_stats: Vec<TubeStats> = sorted_tubes
            .iter()
            .map(|tube| match tube.last() {
                Some(&color) => {
                    let mut color_height = 1usize;
                    while tube.len() > color_height && tube[tube.len() - color_height - 1] == color
                    {
                        color_height += 1;
                    }
                    TubeStats {
                        simple: color_height == tube.len(),
                        color: Some(color),
                        color_height,
                    }
                }
                None => TubeStats {
                    simple: false,
                    color: None,
                    color_height: 0,
                },
            })
            .collect();
        for i in 0..(self.tubes - 1) {
            if !tube_stats[i].simple || tube_stats[i].color_height == self.height {
                continue;
            }
            for j in (i + 1)..self.tubes {
                if tube_stats[j].simple && tube_stats[i].color == tube_stats[j].color {
                    let mut tubes = sorted_tubes.clone();
                    tubes[i].clear();
                    tubes[j].resize(
                        tube_stats[i].color_height + tube_stats[j].color_height,
                        tube_stats[j].color.unwrap(),
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
                    tubes[i].resize(self.height, tube_stats[i].color.unwrap());
                    tubes[j].truncate(sorted_tubes[j].len() - amount);
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
                    tubes[i].resize(amount, tube_stats[j].color.unwrap());
                    tubes[j].truncate(sorted_tubes[j].len() - amount);
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
                        if sorted_tubes[j].len() < self.height {
                            indexes.push((i, j));
                        }
                        if sorted_tubes[i].len() < self.height {
                            indexes.push((j, i));
                        }
                        for (x, y) in indexes {
                            let mut tubes = sorted_tubes.clone();
                            let amount = usize::min(
                                tube_stats[x].color_height,
                                self.height - tubes[y].len(),
                            );
                            tubes[x].truncate(sorted_tubes[x].len() - amount);
                            tubes[y].resize(
                                sorted_tubes[y].len() + amount,
                                tube_stats[x].color.unwrap(),
                            );
                            if self.inner_search(&tubes, i, j) {
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
