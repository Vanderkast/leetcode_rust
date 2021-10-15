use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        // todo implement word search (lc_id: 79)
        return false;
    }
}

fn main() {
    assert_eq!(true, Solution::exist(vec![vec!['A', 'B', 'C', 'E'],
                                          vec!['S', 'F', 'C', 'S'],
                                          vec!['A', 'D', 'E', 'E']],
                                     String::from("ABCCED")))
}
