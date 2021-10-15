use std::collections::{HashSet, LinkedList};

fn main() {
    let sol = length_of_longest_substring;
    let a = " ";
    print!("{} -> {}", &a, sol(String::from(a)));
}

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut set: HashSet<char> = HashSet::new();
    let mut max = 0;
    let mut queue: LinkedList<char> = LinkedList::new();
    let mut last = 0;

    for c in s.chars() {
        queue.push_back(c);
        if max < last {
            max = last;
        }
        last += 1;
        match set.insert(c) {
            true => {}
            false => {
                let mut poped = queue.pop_front();
                while poped != Some(c) {
                    set.remove(&poped.unwrap());
                    poped = queue.pop_front();
                }
                last = set.len();
            }
        }
    }

    (if last > max { last } else { max }) as i32
}
