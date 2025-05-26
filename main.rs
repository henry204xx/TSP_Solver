use std::io::{self, BufRead};

const INF: i32 = i32::MAX;

fn tsp(matrix: &Vec<Vec<i32>>, dp: &mut Vec<Vec<i32>>, parent: &mut Vec<Vec<Option<usize>>>, current_city: usize, visited: usize) -> i32 {
    let n = matrix.len();

    if visited == (1 << n) - 1 { // All cities visited (base case)
        return matrix[current_city][0];
    }

    if dp[current_city][visited] != -1 { //already visited somewhere
        return dp[current_city][visited];
    }

    let mut min_cost = INF;
    let mut best_next_city = None;

    for next_city in 0..n {
        if visited & (1 << next_city) == 0 {
            let next_mask = visited | (1 << next_city);
            let cost = matrix[current_city][next_city] + tsp(matrix, dp, parent, next_city, next_mask); //recursive
            if cost < min_cost {
                min_cost = cost;
                best_next_city = Some(next_city);
            }
        }
    }

    dp[current_city][visited] = min_cost;
    parent[current_city][visited] = best_next_city;
    min_cost
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    println!("Enter number of cities (n):");
    let n: usize = lines
        .next()
        .expect("No input")
        .expect("Failed to read line")
        .trim()
        .parse()
        .expect("Invalid number");

    println!("Enter the cost matrix row by row (space separated):");
    let mut matrix = Vec::with_capacity(n);
    for i in 0..n {
        let line = lines
            .next()
            .expect("Not enough rows")
            .expect("Failed to read line");
        let row: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|x| x.parse().expect("Invalid number"))
            .collect();
        if row.len() != n {
            panic!("Row {} does not have {} elements", i + 1, n);
        }
        matrix.push(row);
    }

    let mut dp = vec![vec![-1; 1 << n]; n];
    let mut parent = vec![vec![None; 1 << n]; n];

    let min_cost = tsp(&matrix, &mut dp, &mut parent, 0, 1);

    println!("Minimum cost for tsp: {}", min_cost);

    // Reconstruct path
    let mut path = vec![1];
    let mut visited = 1;
    let mut current_city = 0;

    while let Some(next_city) = parent[current_city][visited] {
        path.push(next_city+1);
        visited |= 1 << next_city;
        current_city = next_city;
    }

    path.push(1);

    println!("Optimal path: {:?}", path);
}
