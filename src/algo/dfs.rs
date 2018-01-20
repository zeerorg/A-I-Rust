use problems::water_jug;

pub fn dfs<T> (_start: T, _goal: T, _functions: Vec<&Fn(T) -> T>) -> Vec<T> {
    return Vec::new();
}


// Test for dfs

#[cfg(test)]
mod tests {
    use super::*;
    // Water Jug Problem
    

    #[test]
    fn dfs_test() {
        let start_jug = water_jug::Jugs::new(0, 0, 3, 4);
        let end_jug = water_jug::Jugs::new(2, 2, 3, 4);
        assert_eq!(dfs(start_jug.clone(), end_jug.clone(), Vec::new()), Vec::new());
    }
}