use problems::water_jug::*;
use std::hash::Hash;
use std::collections::HashSet;
use std::fmt::Display;

pub fn dfs<T: PartialEq + Hash + Eq + Clone + Display> (start: &T, goal_function: &Fn(&T) -> bool, _functions: Vec<&Fn(&T) -> T>) -> bool {

    let mut open: Vec<T> = Vec::new();
    let mut visited: HashSet<T> = HashSet::new();

    open.push((*start).clone());
    let mut flag = false;

    while !open.is_empty() {
        let to_visit = open.pop();
        match to_visit {
            Some(val) => {
                print!("{} ", val);
                if visited.contains(&val) {
                    continue;
                }
                if goal_function(&val) {
                    flag = true;
                    break;
                }
                visited.insert(val.clone());

                let mut nodes: Vec<T> = (&_functions).into_iter().map(|func| func(&val)).rev().collect();
                open.append(&mut nodes);
            }
            None => break
        }
    }

    return flag;
}


// Test for dfs

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dfs_test() {
        let start_jug = Jugs::new(0, 0, 3, 4);
        fn goal_check(state: &Jugs) -> bool {
            return state.jug_b == 2 || state.jug_a == 2;
        }
        assert_eq!(dfs(&start_jug, &goal_check, vec![&fill_a, &fill_b, &empty_a, &empty_b, &trn_a_to_b, &trn_b_to_a]), true);

    }
}