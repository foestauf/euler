pub fn solve() {
    let grid_size = 20;
    let mut grid = vec![vec![1u64; grid_size + 1]; grid_size + 1];

    for row in 1..=grid_size {
        for col in 1..=grid_size {
            grid[row][col] = grid[row - 1][col] + grid[row][col - 1];
        }
    }

    let num_routes = grid[grid_size][grid_size];
    println!("The number of routes in a {}x{} grid is: {}", grid_size, grid_size, num_routes);
}
