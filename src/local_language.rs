use super::*;

#[derive(Debug, Clone)]
/// A structure representing local langage.
/// Local langages are rationnal langages for that can be determined by looking only at the first character,
/// the last character and every pair of consecutive characters.
/// Local langages are used in the glushkov's construction algorithm to generate an automaton recognizing a
/// langage defined by a regex.
pub struct LocalLanguage {
    pub accept_empty: bool,
    pub prefixes: Vec<usize>,
    pub suffixes: Vec<usize>,
    pub factors: Vec<(usize, usize)>,
}

fn cartesian_product(l1: &Vec<usize>, l2: &Vec<usize>) -> Vec<(usize, usize)> {
    let mut res = Vec::new();
    for &a in l1 {
        for &b in l2 {
            res.push((a, b));
        }
    }
    res
}

impl LocalLanguage {
    /// Local language recognizing only one single char word
    pub fn from_litteral(l: usize) -> LocalLanguage {
        LocalLanguage {
            accept_empty: false,
            prefixes: vec![l],
            suffixes: vec![l],
            factors: Vec::new(),
        }
    }

    /// Local language accepting only the empty word.
    pub fn empty() -> LocalLanguage {
        LocalLanguage {
            accept_empty: true,
            prefixes: Vec::new(),
            suffixes: Vec::new(),
            factors: Vec::new(),
        }
    }

    /// Local langage recognizing the concatenation of two local languages
    ///
    /// Prerequisite: there should not be any character appearing in both langages
    pub fn concatenation(&mut self, mut other: LocalLanguage) {
        self.factors
            .append(&mut cartesian_product(&self.suffixes, &other.prefixes));
        self.factors.append(&mut other.factors);
        if other.accept_empty {
            self.suffixes.append(&mut other.suffixes);
        } else {
            self.suffixes = other.suffixes;
        }
        if self.accept_empty {
            self.prefixes.append(&mut other.prefixes);
        }
        self.accept_empty &= other.accept_empty;
    }

    /// Local langage recognizing the union of two local languages
    ///
    /// Prerequisite: there should not be any character appearing in both langages
    pub fn either(&mut self, mut other: LocalLanguage) {
        self.accept_empty |= other.accept_empty;
        self.prefixes.append(&mut other.prefixes);
        self.suffixes.append(&mut other.suffixes);
        self.factors.append(&mut other.factors);
    }

    /// Add the empty word to the langage
    pub fn optional(&mut self) {
        self.accept_empty = true;
    }

    /// Kleene star applied to a local langage
    pub fn repeat(&mut self) {
        self.accept_empty = true;
        self.factors
            .append(&mut cartesian_product(&self.suffixes, &self.prefixes));
    }

    fn from_regex_rec(r: Regex, chars: &mut Vec<char>) -> LocalLanguage {
        match r {
            Regex::Terminal(c) => {
                let idx = chars.len();
                chars.push(c);
                LocalLanguage::from_litteral(idx)
            }
            Regex::Epsilon => LocalLanguage::empty(),
            Regex::Concatenation(r1, r2) => {
                let mut left = LocalLanguage::from_regex_rec(*r1, chars);
                left.concatenation(LocalLanguage::from_regex_rec(*r2, chars));
                left
            }
            Regex::Alternative(r1, r2) => {
                let mut left = LocalLanguage::from_regex_rec(*r1, chars);
                left.either(LocalLanguage::from_regex_rec(*r2, chars));
                left
            }
            Regex::Optional(r1) => {
                let mut res = LocalLanguage::from_regex_rec(*r1, chars);
                res.optional();
                res
            }
            Regex::Star(r1) => {
                let mut res = LocalLanguage::from_regex_rec(*r1, chars);
                res.repeat();
                res
            }
        }
    }

    /// Generate the local langage obtained by linearising a regex
    pub fn from_regex(r: Regex) -> (LocalLanguage, Vec<char>) {
        let mut vec = Vec::new();
        let ll = LocalLanguage::from_regex_rec(r, &mut vec);
        (ll, vec)
    }
}
