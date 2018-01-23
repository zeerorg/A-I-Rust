mod algo {
    pub mod dfs;
    pub mod bfs;
    pub mod dfid;
}

mod problems {
    pub mod water_jug;
    pub mod eight_puzz;
}

mod helper {
    pub mod node;
}

fn main() {
    println!("Hello, World");
    print_sum(2, increment(&3));
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn increment(x: &i32) -> i32 {
    return x+1;
}