use std::cmp;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Jugs {
    pub jug_a: i32,
    pub jug_b: i32,
    max_a: i32,
    max_b: i32
}

impl Jugs {
    pub fn new(jug_a: i32, jug_b: i32, max_a: i32, max_b: i32) -> Jugs {
        Jugs {
            max_a: max_a,
            max_b: max_b,
            jug_a: jug_a,
            jug_b: jug_b
        }
    }
}

impl fmt::Display for Jugs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.jug_a, self.jug_b)
    }
}

pub fn fill_a(jugs: &Jugs) -> Jugs {
    Jugs {
        jug_a: jugs.max_a,
        jug_b: jugs.jug_b,
        max_a: jugs.max_a,
        max_b: jugs.max_b
    }
}

pub fn fill_b(jugs: &Jugs) -> Jugs {
    Jugs {
        jug_a: jugs.jug_a,
        jug_b: jugs.max_b,
        max_a: jugs.max_a,
        max_b: jugs.max_b
    }
}

pub fn empty_a(jugs: &Jugs) -> Jugs {
    Jugs {
        jug_a: 0,
        jug_b: jugs.jug_b,
        max_a: jugs.max_a,
        max_b: jugs.max_b
    }
}

pub fn empty_b(jugs: &Jugs) -> Jugs {
    Jugs {
        jug_a: jugs.jug_a,
        jug_b: 0,
        max_a: jugs.max_a,
        max_b: jugs.max_b
    }
}

pub fn trn_a_to_b(jugs: &Jugs) -> Jugs {
    Jugs {
        jug_a: cmp::max(jugs.jug_a - (jugs.max_b - jugs.jug_b), 0),
        jug_b: cmp::min(jugs.max_b, jugs.jug_b + jugs.jug_a),
        max_a: jugs.max_a,
        max_b: jugs.max_b
    }
}

pub fn trn_b_to_a(jugs: &Jugs) -> Jugs {
    Jugs {
        jug_b: cmp::max(jugs.jug_b - (jugs.max_a - jugs.jug_a), 0),
        jug_a: cmp::min(jugs.max_a, jugs.jug_b + jugs.jug_a),
        max_a: jugs.max_a,
        max_b: jugs.max_b
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    // Water Jug Problem

    #[test]
    fn fill_test() {
        let start_jug = Jugs::new(2, 0, 3, 4);
        assert_eq!(fill_a(&start_jug), Jugs::new(3, 0, 3, 4));
        assert_eq!(fill_b(&start_jug), Jugs::new(2, 4, 3, 4));
    }

    #[test]
    fn empty_test() {
        let start_jug = Jugs::new(2, 2, 3, 4);
        assert_eq!(empty_b(&start_jug), Jugs::new(2, 0, 3, 4));
        assert_eq!(empty_a(&start_jug), Jugs::new(0, 2, 3, 4));
    }

    #[test]
    fn trn_test() {
        let start_jug1 = Jugs::new(2, 4, 3, 5);
        assert_eq!(trn_a_to_b(&start_jug1), Jugs::new(1, 5, 3, 5));
        assert_eq!(trn_b_to_a(&start_jug1), Jugs::new(3, 3, 3, 5));
    }
}