#[derive(Debug, Clone, PartialEq, Eq)]
/// Type representing a regular expression.
/// A regular expression is a way to represent a regular language (i.e. any language that can be recognized by a finite automaton)
pub enum Regex {
    /// Terminal(c) is the langage containing only the word composed of the unique character c
    Terminal(char),
    /// Epsilon is the langage containing only the empty word
    Epsilon,
    /// Represents the concatenation of two langages
    Concatenation(Box<Regex>, Box<Regex>),
    /// Represents the union of two langage
    Alternative(Box<Regex>, Box<Regex>),
    /// Represents the langage accepting any word from the langage provided but also the empty word
    Optional(Box<Regex>),
    /// Represents the Kleene star applied to a langage.
    /// (i.e. the langage accepting any finite concatenation of words from the provided langage (including the empty word).)
    Star(Box<Regex>),
}

impl Regex {
    ///Give a representation of a regex as a String
    pub fn to_string(&self) -> String {
        match self {
            Regex::Terminal(c) => c.to_string(),
            Regex::Concatenation(a, b) => format!("({}{})", a.to_string(), b.to_string()),
            Regex::Alternative(a, b) => format!("{}|{}", a.to_string(), b.to_string()),
            Regex::Epsilon => 'ε'.to_string(),
            Regex::Optional(a) => format!("{}?", a.to_string()),
            Regex::Star(a) => format!("{}*", a.to_string()),
        }
    }
}
