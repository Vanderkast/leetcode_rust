struct Solution {}

impl Solution {
    pub fn island_perimeter(grid: Vec<Vec<i32>>) -> i32 {
        if grid.len() * grid.get(0).unwrap().len() == 1 {
            return 4;
        }

        let water = Some(&0);
        let earth = Some(&1);
        let mut row: &Vec<i32>;
        let mut prev: i32;
        let mut perimeter = 0;

        for i in 0..grid.len() {
            row = grid.get(i).unwrap();
            prev = 0;
            for j in 0..row.len() {
                if row.get(j) == water {
                    prev = 0;
                    continue;
                }
                perimeter += 4 - prev; // remove left cell edge
                prev = 1;
                // right
                if j + 1 < row.len() {
                    if row.get(j + 1) == earth {
                        perimeter -= 1;
                    }
                }
                // up
                if i > 0 {
                    if let Some(up_row) = grid.get(i.checked_sub(1).unwrap()) {
                        if up_row.get(j) == earth {
                            perimeter -= 1;
                        }
                    }
                }
                // down
                if i + 1 < grid.len() {
                    if let Some(down_row) = grid.get(i + 1) {
                        if down_row.get(j) == earth {
                            perimeter -= 1;
                        }
                    }
                }
            }
        }

        perimeter
    }
}

fn main() {
    let test = vec![
        vec![0, 1, 0, 0],
        vec![1, 1, 1, 0],
        vec![0, 1, 0, 0],
        vec![1, 1, 0, 0],
    ];
    // println!("{}", Solution::island_perimeter(test));

    let test = vec![
        vec![0, 1, 1]
    ];
    // println!("{}", Solution::island_perimeter(test));

    let test = vec![
        vec![0],
        vec![1],
        vec![1],
    ];
    println!("{}", Solution::island_perimeter(test));
}
