use problems::water_jug::*;
use std::hash::Hash;
use std::collections::HashSet;
use std::fmt::Display;

pub fn bfs<T: PartialEq + Hash + Eq + Clone + Display> (start: &T, goal_function: &Fn(&T) -> bool, _functions: Vec<&Fn(&T) -> T>) -> bool {
    let mut open: Vec<T> = Vec::new();
    let mut visited: HashSet<T> = HashSet::new();

    open.push((*start).clone());
    let mut flag = false;

    while !open.is_empty() {
        let to_visit = open.remove(0);
        if visited.contains(&to_visit) {
            continue;
        }
        if goal_function(&to_visit) {
            flag = true;
            break;
        }
        visited.insert(to_visit.clone());
        let mut nodes: Vec<T> = (&_functions).into_iter().map(|func| func(&to_visit)).collect();
        open.append(&mut nodes);
    }

    return flag;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bfs_test() {
        let start_jug = Jugs::new(0, 0, 3, 4);
        fn goal_check(state: &Jugs) -> bool {
            return state.jug_b == 2 || state.jug_a == 2;
        }
        assert_eq!(bfs(&start_jug, &goal_check, vec![&fill_a, &fill_b, &empty_a, &empty_b, &trn_a_to_b, &trn_b_to_a]), true);

    }
}