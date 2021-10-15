/**
 * Forward declaration of guess API
 * @param  num   your gues
 * @return 	     -1 if num is lower than the guess numbe
 *    		      1 if num is higher than the guess numbe
 *               otherwise return
 * unsafe fn guess(num: i32) -> i32 {
 */
struct Solution {}

impl Solution {
    unsafe fn guessNumber(mut right: i32) -> i32 {
        let mut left = 1;
        let mut rel: i32;
        let mut tmp: i32;

        while left != right {
            tmp = (left + right) / 2;
            rel = guess(tmp);
            if rel < 0 {
                left = tmp + 1;
            } else {
                if rel > 0 {
                    right = tmp - 1;
                } else {
                    return tmp;
                }
            }
        }

        left
    }
}
//
//     // this version is not working on leetcode :(
//     unsafe fn guess_number(mut right: i32) -> i32 {
//         let mut left = 0;
//         let mut pick = right / 2 + 1;
//         let mut tmp = guess(pick);
//
//         while tmp != 0 {
//             if tmp < 0 {
//                 left = pick + 1;
//             } else {
//                 right = pick - 1;
//             }
//             pick = (right + left) / 2;
//             tmp = guess(pick);
//         }
//
//         pick
//     }
// }

static mut PICK: i32 = 0;

unsafe fn guess(num: i32) -> i32 {
    if PICK == num {
        return 0;
    }
    if num < PICK {
        -1
    } else {
        1
    }
}

fn test(n: i32, pick: i32) {
    unsafe { PICK = pick; }
    println!("Max {}; Pick {}", n, unsafe { PICK });
    assert_eq!(pick, unsafe { Solution::guessNumber(n) });
}

fn main() {
    // test(1, 1);
    // test(2, 1);
    // test(2, 2);
    // test(3, 2);
    // test(4, 3);
    test(10, 6);
    // test(10, 4);
    // test(100, 51);
    // test(100, 49);
    // test(100, 99);
    // test(100, 1);
    // test(100, 80);
    // test(7, 5);
    // test(5, 5);
}
