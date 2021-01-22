use crate::solver::{all_same, SolutionStep, Solver, Tube, TubeStats};
use std::collections::{HashMap, VecDeque};

struct State {
    tubes: Vec<Tube>,
    depth: usize,
    from: usize,
    to: usize,
    amount: usize,
}

#[derive(Clone)]
struct Step {
    depth: usize,
    from: usize,
    to: usize,
    amount: usize,
    transform: Vec<usize>,
}

pub struct BFSSolver {
    height: usize,
    colors: usize,
    tubes: usize,
    initial_tubes: Vec<Tube>,
    states: HashMap<Vec<Tube>, Step>,
    queue: VecDeque<State>,
}

impl Solver for BFSSolver {
    fn new(height: usize, colors: usize, initial_tubes: &Vec<Tube>) -> Self {
        Self {
            height,
            colors,
            tubes: initial_tubes.len(),
            initial_tubes: initial_tubes.clone(),
            states: HashMap::new(),
            queue: VecDeque::new(),
        }
    }

    fn search(&mut self) -> bool {
        self.queue.push_back(State {
            tubes: self.initial_tubes.clone(),
            depth: 0,
            from: usize::MAX,
            to: usize::MAX,
            amount: 0,
        });
        while !self.queue.is_empty() {
            let state = self.queue.pop_front().unwrap();
            if self.inner_search(&state) {
                return true;
            }
        }
        false
    }

    fn get_solution(&self) -> Vec<SolutionStep> {
        let mut state = vec![vec![]; self.tubes - self.colors];
        for i in 0..self.colors {
            state.push(vec![i as u8; self.height]);
        }
        let mut steps = vec![];
        loop {
            let step = self.states.get(&state).unwrap();
            steps.push(step.clone());
            if step.depth == 0 {
                break;
            }
            let mut inverse_transform = vec![0usize; self.tubes];
            for (i, x) in step.transform.iter().enumerate() {
                inverse_transform[*x] = i;
            }
            state = (0..self.tubes)
                .map(|index| state[inverse_transform[index]].clone())
                .collect();
            let color = *state[step.to].last().unwrap();
            let l1 = state[step.from].len();
            let l2 = state[step.to].len();
            state[step.from].resize(l1 + step.amount, color);
            state[step.to].resize(l2 - step.amount, color);
        }
        steps.reverse();

        let mut solution = vec![];
        let mut transform: Vec<usize> = (0..self.tubes).collect();
        for (index, step) in steps.iter().enumerate() {
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

impl BFSSolver {
    fn is_solved(&self, state: &Vec<Tube>) -> bool {
        state.iter().all(|tube| tube.is_empty() || (tube.len() == self.height && all_same(tube)))
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
        self.states.insert(
            sorted_tubes.clone(),
            Step {
                depth: state.depth,
                from: state.from,
                to: state.to,
                amount: state.amount,
                transform,
            },
        );
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
                if tube_stats[j].simple
                    && sorted_tubes[i][0] == sorted_tubes[j][0]
                    && sorted_tubes[i].len() + sorted_tubes[j].len() >= self.height - 1
                {
                    let mut tubes = sorted_tubes.clone();
                    tubes[i].clear();
                    tubes[j].resize(
                        sorted_tubes[i].len() + sorted_tubes[j].len(),
                        sorted_tubes[j][0],
                    );
                    self.queue.push_back(State {
                        tubes,
                        depth: state.depth + 1,
                        from: i,
                        to: j,
                        amount: sorted_tubes[i].len(),
                    });
                    return false;
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
                    self.queue.push_back(State {
                        tubes,
                        depth: state.depth + 1,
                        from: j,
                        to: i,
                        amount,
                    });
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
                        let mut amount = 1usize;
                        let mut offset_i = tubes[i].len() - 1;
                        let mut offset_j = tubes[j].len() + 1;
                        while offset_i > 0
                            && tubes[i][offset_i - 1] == color
                            && offset_j < self.height
                        {
                            amount += 1;
                            offset_i -= 1;
                            offset_j += 1;
                        }
                        tubes[i].truncate(offset_i);
                        tubes[j].resize(offset_j, color);
                        self.queue.push_back(State {
                            tubes,
                            depth: state.depth + 1,
                            from: i,
                            to: j,
                            amount,
                        });
                    }
                }
            }
        }
        false
    }
}
