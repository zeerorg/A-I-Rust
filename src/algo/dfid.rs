use std::hash::Hash;
use std::collections::HashSet;
use std::fmt::Display;
use helper::node::*;

pub fn dfid<T: PartialEq + Hash + Eq + Clone + Display> (start: &T, goal_function: &Fn(&T) -> bool, _functions: &Vec<&Fn(&T) -> T>) -> bool {

    let mut prev_node_count = 0;
    let mut depth_allowed = 1;
    loop {
        let (found, searched) = dfs_depth(start, goal_function, _functions, depth_allowed);
        println!("{} {}", found, searched);
        depth_allowed += 1;
        if found {
            return true;
        }
        if searched == prev_node_count {
            return false;
        }
        prev_node_count = searched;
    }
}

fn dfs_depth<T: PartialEq + Hash + Eq + Clone + Display> (start: &T, goal_function: &Fn(&T) -> bool, _functions: &Vec<&Fn(&T) -> T>, max_depth: i32) -> (bool, i32) {
        let mut open: Vec<NodeDepth<T>> = Vec::new();
        let mut visited: HashSet<T> = HashSet::new();

        open.push(NodeDepth::new(start.clone(), 0));

        while !open.is_empty() {
            let to_visit = open.pop();
            match to_visit {
                Some(val) => {
                    let curr_depth = val.depth;
                    let to_visit = val.node;
                    print!("{} ", curr_depth);
                    if visited.contains(&to_visit) {
                        continue;
                    }
                    if goal_function(&to_visit) {
                        return (true, visited.len() as i32);
                    }

                    if curr_depth < max_depth {
                        visited.insert(to_visit.clone());
                        let mut nodes: Vec<NodeDepth<T>> = (&_functions).into_iter().map(|func| func(&to_visit)).map(|node_val| NodeDepth::new(node_val, curr_depth+1)).rev().collect();
                        open.append(&mut nodes);
                    }
                }

                None => break
            }
        }
    return (false, visited.len() as i32);
}

#[cfg(test)]
mod tests {
    use super::*;
    use problems::water_jug::*;
    use problems::eight_puzz::*;

    #[test]
    fn dfid_test() {
        let start_jug = Jugs::new(0, 0, 3, 4);
        fn goal_check(state: &Jugs) -> bool {
            return state.jug_b == 2 || state.jug_a == 2;
        }
        let (flag, _) = dfs_depth(&start_jug, &goal_check, &vec![&fill_a, &fill_b, &empty_a, &empty_b, &trn_a_to_b, &trn_b_to_a], 1000);
        assert_eq!(dfid(&start_jug, &goal_check, &vec![&fill_a, &fill_b, &empty_a, &empty_b, &trn_a_to_b, &trn_b_to_a]), true);
        assert_eq!(flag, true);
    }

    #[test]
    fn dfid_neq_test() {
        fn test_gt_2(x: &i32) -> bool {
            return *x > 2;
        }
        assert_eq!(dfid(&1, &test_gt_2, &vec![]), false);
    }

    #[test]
    fn dfid_8puzz_test() {
        let start_puzzle = Puzzle::new([
            [1, 2, 3],
            [4, 5, 6],
            [7, 8, 0]
        ]);
        fn is_final(to_check: &Puzzle) -> bool {
            return (*to_check) == Puzzle::new([
                [5, 2, 3],
                [8, 1, 6],
                [7, 0, 4]
            ])
        }
        assert_eq!(dfid(&start_puzzle, &is_final, &vec![&from_down, &from_up, &from_right, &from_left]), true);
    }
}