use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn num_squares(mut n: i32) -> i32 {
        let mut tmp1 = isqrt(n);

        if tmp1 == 1 {
            return n;
        }

        if tmp1 * tmp1 == n {
            return 1;
        }


        let mut tmp2;
        for i in 1..=tmp1 {
            tmp1 = n - i * i;
            tmp2 = isqrt(tmp1);
            if tmp2 * tmp2 == tmp1 {
                return 2;
            }
        }

        while n % 4 == 0 {
            n /= 4;
        }

        if n % 8 == 7 {
            return 4;
        }

        3
    }

    //todo: maybe complete
    pub fn _num_squares(n: i32) -> i32 {
        let mut max = isqrt(n);
        if max * max == n {
            return 1;
        }
        let mut tmp = max;
        let mut sum = max * max;
        let mut count = 1;
        let mut used = HashSet::new();
        used.insert(max);
        while sum != n {
            tmp = isqrt(n - sum);
            used.insert(tmp);
            sum += tmp * tmp;
            count += 1;
        }
        if count > 2 {
            let mut candidate: i32;
            max = max - 1;
            while !used.contains(&max) && count > 2 {
                sum = max * max;
                candidate = 1;
                used.insert(tmp);
                while sum != n {
                    tmp = isqrt(n - sum);
                    sum += tmp * tmp;
                    used.insert(tmp);
                    candidate += 1;
                }
                if candidate < count {
                    count = candidate;
                }
                max = max - 1;
            }
        }

        count
    }
}

fn isqrt(n: i32) -> i32 {
    let mut shift = 2;
    let mut n_shifted = n >> shift;
    while n_shifted != 0 && n_shifted != n {
        shift += 2;
        n_shifted >>= 2;
    }
    shift -= 2;

    let mut result = 0;
    let mut tmp;
    while shift >= 0 {
        result <<= 1;
        tmp = result + 1;
        if tmp * tmp <= (n >> shift) {
            result = tmp
        }
        shift -= 2
    }

    result
}

fn main() {
    assert_eq!(2, Solution::num_squares(2));
    assert_eq!(3, Solution::num_squares(3));
    assert_eq!(2, Solution::num_squares(8));
    assert_eq!(3, Solution::num_squares(12));
    assert_eq!(2, Solution::num_squares(13));
    assert_eq!(2, Solution::num_squares(18));
    assert_eq!(1, Solution::num_squares(49));
    assert_eq!(2, Solution::num_squares(50));
    assert_eq!(3, Solution::num_squares(51));
}
