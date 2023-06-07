use super::utils::{get_transform, get_tube_stat, is_solved, pour, pour_back};
use super::{SolutionStep, Solver};
use itertools::Itertools;
use rustc_hash::FxHashMap;
use std::collections::VecDeque;
use std::iter;
use std::rc::Rc;

#[derive(Clone)]
struct State {
    tubes: Rc<Vec<u8>>,
    depth: usize,
    from: usize,
    to: usize,
    amount: usize,
    transform: Vec<usize>,
}

pub struct BFSSolver {
    height: usize,
    tubes: usize,
    initial_tubes: Vec<u8>,
    states: FxHashMap<Rc<Vec<u8>>, Rc<State>>,
    queue: VecDeque<Rc<State>>,
}

impl Solver for BFSSolver {
    fn new(height: usize, initial_tubes: Vec<u8>) -> Self {
        Self {
            height,
            tubes: initial_tubes.len() / height,
            initial_tubes,
            states: FxHashMap::default(),
            queue: VecDeque::new(),
        }
    }

    fn search(&mut self) -> bool {
        if self.push_state(&self.initial_tubes.clone(), 0, usize::MAX, usize::MAX, 0) {
            return true;
        }
        while let Some(state) = self.queue.pop_front() {
            if self.inner_search(state.as_ref()) {
                return true;
            }
        }
        false
    }

    fn get_solution(&self) -> Vec<SolutionStep> {
        let colors = self.initial_tubes.iter().filter(|&&x| x > 0).count() / self.height;
        let mut state = vec![0; (self.tubes - colors) * self.height];
        for i in 1..=colors {
            state.extend(iter::repeat(i as u8).take(self.height));
        }
        let mut steps = vec![];
        let mut inverse_transform: Vec<usize> = vec![0; self.tubes];
        loop {
            let step = self.states.get(&state).unwrap();
            steps.push(step);
            if step.depth == 0 {
                break;
            }
            for (i, &x) in step.transform.iter().enumerate() {
                inverse_transform[x] = i;
            }
            let mut new_state = vec![0; self.tubes * self.height];
            for i in 0..self.tubes {
                for j in 0..self.height {
                    new_state[i * self.height + j] = state[inverse_transform[i] * self.height + j];
                }
            }
            state = new_state;
            pour_back(&mut state, self.height, step.from, step.to, step.amount);
        }
        steps.reverse();

        let mut solution = vec![];
        let mut transform = (0..self.tubes).collect_vec();
        let mut new_transform = vec![0; self.tubes];
        for (index, step) in steps.iter().enumerate() {
            if index > 0 {
                solution.push(SolutionStep {
                    from: transform[step.from],
                    to: transform[step.to],
                });
            }
            for i in 0..self.tubes {
                new_transform[i] = transform[step.transform[i]];
            }
            std::mem::swap(&mut transform, &mut new_transform);
        }
        solution
    }
}

impl BFSSolver {
    fn push_state(
        &mut self,
        tubes: &[u8],
        depth: usize,
        from: usize,
        to: usize,
        amount: usize,
    ) -> bool {
        let (transform, sorted_tubes) = get_transform(tubes, self.height, self.tubes);
        if self.states.contains_key(&sorted_tubes) {
            return false;
        }
        let solved = is_solved(&sorted_tubes, self.height);
        let sorted_tubes = Rc::new(sorted_tubes);
        let state = Rc::new(State {
            tubes: sorted_tubes.clone(),
            depth,
            from,
            to,
            amount,
            transform,
        });
        self.queue.push_back(state.clone());
        self.states.insert(sorted_tubes, state);
        solved
    }

    fn inner_search(&mut self, state: &State) -> bool {
        let tube_stats = state
            .tubes
            .chunks_exact(self.height)
            .map(|tube| get_tube_stat(tube, self.height))
            .collect_vec();
        for i in 0..(self.tubes - 1) {
            if !tube_stats[i].simple || tube_stats[i].color_height == self.height {
                continue;
            }
            for j in (i + 1)..self.tubes {
                if tube_stats[j].simple
                    && tube_stats[i].color == tube_stats[j].color
                    && tube_stats[i].color_height + tube_stats[j].color_height >= self.height - 1
                {
                    let mut tubes = (*state.tubes).clone();
                    pour(
                        &mut tubes,
                        self.height,
                        &tube_stats,
                        i,
                        j,
                        tube_stats[i].size,
                    );
                    return self.push_state(
                        &tubes,
                        state.depth + 1,
                        i,
                        j,
                        tube_stats[i].color_height,
                    );
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
                    let mut tubes = (*state.tubes).clone();
                    pour(&mut tubes, self.height, &tube_stats, j, i, amount);
                    return self.push_state(&tubes, state.depth + 1, j, i, amount);
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
                    let mut tubes = (*state.tubes).clone();
                    let amount = tube_stats[j].color_height;
                    pour(&mut tubes, self.height, &tube_stats, j, i, amount);
                    if self.push_state(&tubes, state.depth + 1, j, i, amount) {
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
                            let mut tubes = (*state.tubes).clone();
                            let amount = usize::min(
                                tube_stats[x].color_height,
                                self.height - tube_stats[y].size,
                            );
                            pour(&mut tubes, self.height, &tube_stats, x, y, amount);
                            if self.push_state(&tubes, state.depth + 1, x, y, amount) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }
}
