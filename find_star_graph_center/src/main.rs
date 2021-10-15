use std::collections::HashSet;

struct Solution {}

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let first = edges[0][0];
        let second = edges[0][1];
        if (first == edges[1][0]) || (first == edges[1][1]) {
            return first;
        }
        return second;
    }

    pub fn _find_center(edges: Vec<Vec<i32>>) -> i32 {
        let mut nodes: HashSet<i32> = HashSet::new();
        for edge in edges {
            if !nodes.insert(edge[0]) {
                return edge[0];
            }
            if !nodes.insert(edge[1]) {
                return edge[1];
            }
        }
        return 0;
    }
}

fn main() {
    assert_eq!(2, Solution::find_center(vec![vec![1,2],vec![2,3],vec![4,2]]))
}
