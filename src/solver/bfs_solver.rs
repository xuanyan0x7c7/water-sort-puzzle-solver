use super::utils::{get_transform, get_tube_stat, is_solved, pour, pour_back, TubeStats};
use super::{SolutionStep, Solver};
use std::collections::HashMap;
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
    colors: usize,
    tubes: usize,
    initial_tubes: Vec<u8>,
    states: HashMap<Rc<Vec<u8>>, Rc<State>>,
}

impl Solver for BFSSolver {
    fn new(height: usize, colors: usize, initial_tubes: Vec<u8>) -> Self {
        let tubes = initial_tubes.len() / height;
        Self {
            height,
            colors,
            tubes,
            initial_tubes,
            states: HashMap::new(),
        }
    }

    fn search(&mut self) -> bool {
        let mut current_states = vec![];
        if self.push_state(
            &mut current_states,
            &self.initial_tubes.clone(),
            0,
            usize::MAX,
            usize::MAX,
            0,
        ) {
            return true;
        }
        while !current_states.is_empty() {
            let mut next_states = vec![];
            for state in current_states.iter() {
                if self.inner_search(state, &mut next_states) {
                    return true;
                }
            }
            current_states = next_states;
        }
        false
    }

    fn get_solution(&self) -> Vec<SolutionStep> {
        let mut state = vec![0; (self.tubes - self.colors) * self.height];
        for i in 0..self.colors {
            for _ in 0..self.height {
                state.push((i + 1) as u8);
            }
        }
        let mut steps = vec![];
        loop {
            let step = self.states.get(&state).unwrap();
            steps.push(step);
            if step.depth == 0 {
                break;
            }
            let mut inverse_transform = vec![0usize; self.tubes];
            for (i, x) in step.transform.iter().enumerate() {
                inverse_transform[*x] = i;
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
    fn push_state(
        &mut self,
        next_states: &mut Vec<Rc<State>>,
        tubes: &Vec<u8>,
        depth: usize,
        from: usize,
        to: usize,
        amount: usize,
    ) -> bool {
        let (transform, sorted_tubes) = get_transform(tubes, self.height, self.tubes);
        if self.states.contains_key(&sorted_tubes) {
            return false;
        }
        let sorted_tubes = Rc::new(sorted_tubes);
        let state = Rc::new(State {
            tubes: sorted_tubes.clone(),
            depth,
            from,
            to,
            amount,
            transform,
        });
        next_states.push(state.clone());
        self.states.insert(sorted_tubes.clone(), state);
        is_solved(&sorted_tubes, self.height)
    }

    fn inner_search(&mut self, state: &State, next_states: &mut Vec<Rc<State>>) -> bool {
        let tube_stats: Vec<TubeStats> = state
            .tubes
            .chunks_exact(self.height)
            .map(|tube| get_tube_stat(tube, self.height))
            .collect();
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
                        next_states,
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
                    return self.push_state(next_states, &tubes, state.depth + 1, j, i, amount);
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
                    if self.push_state(next_states, &tubes, state.depth + 1, j, i, amount) {
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
                            if self.push_state(next_states, &tubes, state.depth + 1, x, y, amount) {
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
