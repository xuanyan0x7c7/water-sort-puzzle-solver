use crate::solver::base::{is_solved as is_game_solved, SolutionStep, Solver, Tube};
use std::collections::HashMap;
use std::rc::Rc;

struct TubeStats {
    simple: bool,
    color: Option<u8>,
    color_height: usize,
}

#[derive(Clone)]
struct State {
    tubes: Rc<Vec<Tube>>,
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
    states: HashMap<Rc<Vec<Tube>>, Rc<State>>,
}

impl Solver for BFSSolver {
    fn new(height: usize, colors: usize, initial_tubes: &Vec<Tube>) -> Self {
        Self {
            height,
            colors,
            tubes: initial_tubes.len(),
            initial_tubes: initial_tubes.clone(),
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
            for state in current_states {
                if self.inner_search(&state, &mut next_states) {
                    return true;
                }
            }
            current_states = next_states;
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
            steps.push(step);
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
    #[inline]
    fn is_solved(&self, state: &Vec<Tube>) -> bool {
        is_game_solved(state, self.height)
    }

    fn push_state(
        &mut self,
        next_states: &mut Vec<Rc<State>>,
        tubes: &Vec<Tube>,
        depth: usize,
        from: usize,
        to: usize,
        amount: usize,
    ) -> bool {
        let mut transform: Vec<usize> = (0..self.tubes).collect();
        transform.sort_unstable_by_key(|index| &tubes[*index]);
        let sorted_tubes: Rc<Vec<Tube>> = Rc::new(
            transform
                .iter()
                .map(|index| tubes[*index].clone())
                .collect(),
        );
        if self.states.get(&sorted_tubes).is_some() {
            return false;
        }
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
        self.is_solved(&sorted_tubes)
    }

    fn inner_search(&mut self, state: &State, next_states: &mut Vec<Rc<State>>) -> bool {
        let tube_stats: Vec<TubeStats> = state
            .tubes
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
                if tube_stats[j].simple
                    && tube_stats[i].color == tube_stats[j].color
                    && tube_stats[i].color_height + tube_stats[j].color_height >= self.height - 1
                {
                    let mut tubes = (*state.tubes).clone();
                    tubes[i].clear();
                    tubes[j].resize(
                        tube_stats[i].color_height + tube_stats[j].color_height,
                        tube_stats[i].color.unwrap(),
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
                    tubes[i].resize(self.height, tube_stats[i].color.unwrap());
                    tubes[j].truncate(state.tubes[j].len() - amount);
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
                    tubes[i].resize(amount, tube_stats[j].color.unwrap());
                    tubes[j].truncate(state.tubes[j].len() - amount);
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
                        if state.tubes[j].len() < self.height {
                            indexes.push((i, j));
                        }
                        if state.tubes[i].len() < self.height {
                            indexes.push((j, i));
                        }
                        for (x, y) in indexes {
                            let mut tubes = (*state.tubes).clone();
                            let amount = usize::min(
                                tube_stats[x].color_height,
                                self.height - tubes[y].len(),
                            );
                            tubes[x].truncate(state.tubes[x].len() - amount);
                            tubes[y].resize(
                                state.tubes[y].len() + amount,
                                tube_stats[x].color.unwrap(),
                            );
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
