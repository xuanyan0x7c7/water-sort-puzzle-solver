use crate::solver::base::{
    all_same, is_solved as is_game_solved, SolutionStep, Solver, Tube, TubeStats,
};
use std::collections::HashMap;
use std::rc::Rc;

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
            self.initial_tubes.clone(),
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
        tubes: Vec<Tube>,
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
            tubes: Rc::clone(&sorted_tubes),
            depth,
            from,
            to,
            amount,
            transform,
        });
        next_states.push(Rc::clone(&state));
        self.states.insert(Rc::clone(&sorted_tubes), state);
        self.is_solved(&sorted_tubes)
    }

    fn inner_search(&mut self, state: &State, next_states: &mut Vec<Rc<State>>) -> bool {
        let tube_stats: Vec<TubeStats> = state
            .tubes
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
                    && state.tubes[i][0] == state.tubes[j][0]
                    && state.tubes[i].len() + state.tubes[j].len() >= self.height - 1
                {
                    let mut tubes = (*state.tubes).clone();
                    tubes[i].clear();
                    tubes[j].resize(
                        state.tubes[i].len() + state.tubes[j].len(),
                        state.tubes[j][0],
                    );
                    return self.push_state(
                        next_states,
                        tubes,
                        state.depth + 1,
                        i,
                        j,
                        state.tubes[i].len(),
                    );
                }
            }
        }
        for i in 0..self.tubes {
            if tube_stats[i].empty || !tube_stats[i].simple || tube_stats[i].finished {
                continue;
            }
            for j in 0..self.tubes {
                if j == i || tube_stats[j].simple || state.tubes[i].last() != state.tubes[j].last()
                {
                    continue;
                }
                let color = *state.tubes[j].last().unwrap();
                let mut amount = 1usize;
                while state.tubes[j].len() > amount
                    && state.tubes[j][state.tubes[j].len() - amount - 1] == color
                {
                    amount += 1;
                }
                if state.tubes[i].len() + amount == self.height {
                    let mut tubes = (*state.tubes).clone();
                    tubes[i].resize(self.height, color);
                    tubes[j].truncate(state.tubes[j].len() - amount);
                    return self.push_state(next_states, tubes, state.depth + 1, j, i, amount);
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
                    let mut tubes = (*state.tubes).clone();
                    let color = *state.tubes[j].last().unwrap();
                    let mut amount = 1usize;
                    let mut offset = tubes[j].len() - amount;
                    while offset > 0 && tubes[j][offset - 1] == color {
                        amount += 1;
                        offset -= 1;
                    }
                    tubes[i].resize(amount, color);
                    tubes[j].truncate(offset);
                    if self.push_state(next_states, tubes, state.depth + 1, j, i, amount) {
                        return true;
                    }
                } else if state.tubes[i].last() == state.tubes[j].last() {
                    let color = *state.tubes[i].last().unwrap();
                    let mut indexes = vec![];
                    if state.tubes[j].len() < self.height {
                        indexes.push((i, j));
                    }
                    if state.tubes[i].len() < self.height {
                        indexes.push((j, i));
                    }
                    for (i, j) in indexes {
                        let mut tubes = (*state.tubes).clone();
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
                        if self.push_state(next_states, tubes, state.depth + 1, i, j, amount) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}
