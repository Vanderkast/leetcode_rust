struct Solution;

impl Solution {
    pub fn range_bitwise_and(left: i32, right: i32) -> i32 {
        if left == right {
            return left;
        }

        let temp = left.leading_zeros();

        if temp != right.leading_zeros() {
            return 0;
        }

        let mut temp = 1 << (31 - temp);
        let mut result = temp;
        temp >>= 1;

        while left & temp == right & temp {
            result += left & temp;
            temp >>= 1;
        }

        result
    }
}

fn main() {
    assert_eq!(0, Solution::range_bitwise_and(0, 0));
    assert_eq!(4, Solution::range_bitwise_and(5, 6));
    assert_eq!(4, Solution::range_bitwise_and(5, 7));
    assert_eq!(6, Solution::range_bitwise_and(6, 7));
    assert_eq!(0, Solution::range_bitwise_and(7, 21));
    assert_eq!(10, Solution::range_bitwise_and(10, 11));
    assert_eq!(12, Solution::range_bitwise_and(12, 14));
    assert_eq!(20, Solution::range_bitwise_and(20, 22));
}
