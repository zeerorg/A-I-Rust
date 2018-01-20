mod algo {
    pub mod dfs;   
}

mod problems {
    pub mod water_jug;
}

fn main() {
    println!("Hello, World");
    print_sum(2, increment(3));
    let mut arr : Vec<&Fn(i32) -> i32> = Vec::new();
    arr.push(&increment);
    algo::dfs::dfs(1, 2, arr);
}

fn print_sum(x: i32, y: i32) {
    println!("sum is: {}", x + y);
}

fn increment(x: i32) -> i32 {
    return x+1;
}