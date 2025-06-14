// given a table with n rows and m columns. Each cell is colored with white or black.
// Considering the shapes created by black cells, what is the maximum border of these shapes?
// Border of a shape means the maximum number of consecutive black cells in any row or column without any white cell in between.
// A shape is a set of connected cells. Two cells are connected if they share an edge. Note that no shape has a hole in it.

use std::{
    collections::VecDeque,
    io::{self, Read},
};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    let t: usize = lines.next().unwrap().trim().parse().unwrap();

    for _ in 0..t {
        let nm = lines.next().unwrap();
        let mut nm_iter = nm.split_whitespace();
        let n: usize = nm_iter.next().unwrap().parse().unwrap();
        let m: usize = nm_iter.next().unwrap().parse().unwrap();

        let mut grid = Vec::with_capacity(n);
        for _ in 0..n {
            let row = lines.next().unwrap().trim().chars().collect::<Vec<char>>();
            grid.push(row);
        }
        let mut visited = vec![vec![false; m]; n];
        let mut max_border = 0;

        let directions = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == '#' && !visited[i][j] {
                    // BFS to find all cells in this shape
                    let mut queue = VecDeque::new();
                    queue.push_back((i, j));
                    visited[i][j] = true;

                    // Store all cells in this shape
                    let mut shape_cells = Vec::new();
                    shape_cells.push((i, j));

                    while let Some((x, y)) = queue.pop_front() {
                        for &(dx, dy) in &directions {
                            let nx = x as isize + dx;
                            let ny = y as isize + dy;
                            if nx >= 0 && nx < n as isize && ny >= 0 && ny < m as isize {
                                let (nx, ny) = (nx as usize, ny as usize);
                                if grid[nx][ny] == '#' && !visited[nx][ny] {
                                    visited[nx][ny] = true;
                                    queue.push_back((nx, ny));
                                    shape_cells.push((nx, ny));
                                }
                            }
                        }
                    }
                    // For this shape, scan rows and columns for max consecutive '#'
                    // Build a set of rows and columns that this shape occupies
                    let mut row_map = vec![vec![false; m]; n];
                    for &(x, y) in &shape_cells {
                        row_map[x][y] = true;
                    }

                    // Row Scan
                    for x in 0..n {
                        let mut count = 0;
                        let mut local_max = 0;
                        for y in 0..m {
                            if row_map[x][y] {
                                count += 1;
                                local_max = local_max.max(count);
                            } else {
                                count = 0;
                            }
                        }
                        max_border = max_border.max(local_max);
                    }
                    // Column Scan
                    for y in 0..m {
                        let mut count = 0;
                        let mut local_max = 0;
                        for x in 0..n {
                            if row_map[x][y] {
                                count += 1;
                                local_max = local_max.max(count);
                            } else {
                                count = 0;
                            }
                        }
                        max_border = max_border.max(local_max);
                    }
                }
            }
        }
        println!("{}", max_border)
    }
}
