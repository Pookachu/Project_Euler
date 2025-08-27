mod grid;

fn main() {
    let grid: &[[u32; 20]; 20] = &grid::GRID;
    let mut max_product: u32 = 0;

    for row in 0..20 {
        for col in 0..20 {
            if col + 3 < 20 {
                max_product = max_product.max(grid[row][col] * grid[row][col+1] * grid[row][col+2] * grid[row][col+3]);
            }

            if row + 3 < 20 {
                max_product = max_product.max(grid[row][col] * grid[row+1][col] * grid[row+2][col] * grid[row+3][col]);
            }

            if row + 3 < 20 && col + 3 < 20 {
                max_product = max_product.max(grid[row][col] * grid[row+1][col+1] * grid[row+2][col+2] * grid[row+3][col+3]);
            }
            
            if row +3 < 20 && col >= 3 {
                max_product = max_product.max(grid[row][col] * grid[row+1][col-1] * grid[row+2][col-2] * grid[row+3][col-3]);
            }
        }
    }
    println!("Max Product: {}", max_product);
}
