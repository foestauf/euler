const GRID: &str = include_str!("grid.txt");

pub fn solve() {
    let grid = parse_grid(GRID);
    let largest_product = find_largest_product(&grid);
    println!("The largest product is {}", largest_product);

}

fn parse_grid(grid: &str) -> [[u32; 20]; 20] {
    let mut result = [[0; 20]; 20];
    for (i, line) in grid.lines().enumerate() {
        for (j, number) in line.split_whitespace().enumerate() {
            result[i][j] = number.parse().unwrap();
        }
    }
    result
}

fn find_largest_product(grid: &[[u32; 20]; 20]) -> u32 {
    let mut max_product = 0;

    for i in 0..20 {
        for j in 0..20 {
            let horizontal = if j < 17 {
                grid[i][j] * grid[i][j + 1] * grid[i][j + 2] * grid[i][j + 3]
            } else {
                0
            };

            let vertical = if i < 17 {
                grid[i][j] * grid[i + 1][j] * grid[i + 2][j] * grid[i + 3][j]
            } else {
                0
            };

            let diagonal_right = if i < 17 && j < 17 {
                grid[i][j] * grid[i + 1][j + 1] * grid[i + 2][j + 2] * grid[i + 3][j + 3]
            } else {
                0
            };

            let diagonal_left = if i < 17 && j >= 3 {
                grid[i][j] * grid[i + 1][j - 1] * grid[i + 2][j - 2] * grid[i + 3][j - 3]
            } else {
                0
            };

            let products = [horizontal, vertical, diagonal_right, diagonal_left];

            let current_max = products.iter().max().unwrap();
            max_product = max_product.max(*current_max);
        }
    }

    max_product
}