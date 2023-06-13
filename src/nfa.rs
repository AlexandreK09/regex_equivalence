use super::*;
use std::collections::{BTreeSet, HashMap, HashSet};

#[derive(Debug, Clone)]
/// struct representing a nondeterministic finite automaton
pub struct NFA {
    initial: usize, //we only need one initial state
    finals: HashSet<usize>,
    transitions: Vec<HashMap<char, HashSet<usize>>>,
}

impl NFA {
    ///generate a nondterministic finite automaton from a local langage
    pub fn from_local_language(ll: LocalLanguage, chars: Vec<char>) -> NFA {
        let states_count = chars.len() + 1;
        let mut finals: HashSet<usize> = ll.suffixes.into_iter().collect();
        let mut transitions = vec![HashMap::new(); states_count];

        let initial = chars.len();

        for prefixe in ll.prefixes {
            //add transitions from initial state to prefixes
            transitions[initial]
                .entry(chars[prefixe])
                .or_insert(HashSet::new())
                .insert(prefixe);
        }

        if ll.accept_empty {
            finals.insert(initial);
        }

        for (a, b) in ll.factors {
            let c = chars[b];
            transitions[a].entry(c).or_insert(HashSet::new()).insert(b);
        }
        NFA {
            initial,
            finals,
            transitions,
        }
    }

    fn next_states(&self, state: usize, c: char) -> HashSet<usize> {
        self.transitions[state]
            .get(&c)
            .cloned()
            .unwrap_or(HashSet::new())
    }

    fn next_states_mult(&self, states: &BTreeSet<usize>, c: char) -> BTreeSet<usize> {
        let mut res = BTreeSet::new();
        for &state in states {
            res.extend(self.next_states(state, c));
        }
        res
    }

    /// generate a determinstic finite automaton that is equivalent to the nondeterminstic finite automaton
    pub fn determinize(self) -> DFA {
        let initial = 0;
        let mut finals = HashSet::new();
        let mut transitions = Vec::new();

        let mut indexes = HashMap::new();
        let mut stack = Vec::new();

        let initial_states = {
            let mut res = BTreeSet::new();
            res.insert(self.initial);
            res
        };

        transitions.push([0; 26]);
        indexes.insert(initial_states.clone(), 0);

        if self.finals.contains(&self.initial) {
            finals.insert(0);
        }

        let mut states_count = 1;
        stack.push(initial_states);

        while let Some(states) = stack.pop() {
            let actual_id = *indexes.get(&states).unwrap();
            for i in 0..26 {
                let c = (b'a' + i) as char;
                let next_states = self.next_states_mult(&states, c);

                if !indexes.contains_key(&next_states) {
                    transitions.push([0; 26]);
                    indexes.insert(next_states.clone(), states_count);

                    for state in &next_states {
                        if self.finals.contains(state) {
                            finals.insert(states_count);
                        }
                    }

                    states_count += 1;
                    stack.push(next_states.clone());
                }

                let next_id = *indexes.get(&next_states).unwrap();
                transitions[actual_id][i as usize] = next_id;
            }
        }

        DFA {
            states_count,
            initial,
            finals,
            transitions,
        }
    }
}
