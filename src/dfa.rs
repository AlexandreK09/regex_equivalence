use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone)]
/// A deterministic finite automaton
pub struct DFA {
    pub states_count: usize,
    pub initial: usize,
    pub finals: HashSet<usize>,
    pub transitions: Vec<[usize; 26]>,
}

impl DFA {
    /// transform the automaton to an automaton recognizing the complement of the langage recognized by the original automaton
    pub fn to_complement(&mut self) {
        let mut new_finals = HashSet::new();
        for i in 0..self.states_count {
            if !self.finals.contains(&i) {
                new_finals.insert(i);
            }
        }
        self.finals = new_finals;
    }

    /// generate an automaton recognizing the instersection of the langages recognized by two automatons
    pub fn intersection(&self, other: &DFA) -> DFA {
        let mut finals = HashSet::new();
        let mut transitions = Vec::new();

        let mut stack = Vec::new();
        let mut indexes = HashMap::new();

        indexes.insert((self.initial, other.initial), 0);

        if self.finals.contains(&self.initial) && other.finals.contains(&other.initial) {
            finals.insert(0);
        }

        let initial = 0;
        let mut states_count = 1;
        stack.push((self.initial, other.initial));
        transitions.push([0; 26]);

        while let Some((x, y)) = stack.pop() {
            let actual_index = *indexes.get(&(x, y)).unwrap();
            for i in 0..26 {
                let next_x = self.transitions[x][i];
                let next_y = other.transitions[y][i];

                if !indexes.contains_key(&(next_x, next_y)) {
                    indexes.insert((next_x, next_y), states_count);
                    stack.push((next_x, next_y));
                    if self.finals.contains(&next_x) && other.finals.contains(&next_y) {
                        finals.insert(states_count);
                    }
                    states_count += 1;
                    transitions.push([0; 26]);
                }

                let next_index = *indexes.get(&(next_x, next_y)).unwrap();

                transitions[actual_index][i] = next_index;
            }
        }

        DFA {
            states_count,
            initial,
            finals,
            transitions,
        }
    }

    /// determine if the langage recognized by an automaton is included in the langage recognized by an other automaton
    pub fn is_included_in(&self, mut other: DFA) -> bool {
        other.to_complement();
        let inter = self.intersection(&other);
        inter.finals.len() == 0
    }

    /// determine if two automatons are recognizing the same langage
    pub fn equals(a: DFA, b: DFA) -> bool {
        a.is_included_in(b.clone()) && b.is_included_in(a)
    }
}
