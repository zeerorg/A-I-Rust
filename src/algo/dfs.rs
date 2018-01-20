
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
        assert_eq!(dfs(1, 2, Vec::new()), Vec::new());
    }
}