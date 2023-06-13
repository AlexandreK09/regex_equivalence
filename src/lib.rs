mod dfa;
mod local_language;
mod nfa;
mod regex;
pub use dfa::DFA;
pub use local_language::LocalLanguage;
pub use nfa::NFA;
pub use regex::Regex;

fn dfa_from_regex(r: Regex) -> DFA {
    let (ll, chars) = LocalLanguage::from_regex(r);
    let nfa = NFA::from_local_language(ll, chars);
    let dfa = nfa.determinize();
    dfa
}

/// A fonction to determine if two regexes recognize the same langage
pub fn is_equivalent(reg1: &Regex, reg2: &Regex) -> bool {
    DFA::equals(dfa_from_regex(reg1.clone()), dfa_from_regex(reg2.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use Regex::*;

    #[test]
    fn test_equivalent() {
        assert!(is_equivalent(
            &Star(Box::new(Terminal('a'))),
            &Concatenation(
                Box::new(Optional(Box::new(Terminal('a')))),
                Box::new(Star(Box::new(Terminal('a'))))
            )
        ));
        assert!(is_equivalent(
            &Alternative(Box::new(Terminal('a')), Box::new(Terminal('b'))),
            &Alternative(Box::new(Terminal('b')), Box::new(Terminal('a')))
        ));
    }

    #[test]
    fn test_not_equivalent() {
        assert!(!is_equivalent(
            &Star(Box::new(Terminal('a'))),
            &Concatenation(
                Box::new(Terminal('a')),
                Box::new(Star(Box::new(Terminal('a'))))
            )
        ));
        assert!(!is_equivalent(
            &Star(Box::new(Alternative(
                Box::new(Terminal('a')),
                Box::new(Terminal('b'))
            ))),
            &Alternative(
                Box::new(Star(Box::new(Terminal('a')))),
                Box::new(Star(Box::new(Terminal('a'))))
            )
        ));
    }
}
