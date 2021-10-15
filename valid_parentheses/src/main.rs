use std::collections::LinkedList;

pub fn is_valid(string: String) -> bool {
    let mut stack :LinkedList<char> = LinkedList::new();
    for c in string.chars() {
        match c {
            '(' => stack.push_back(c),
            '{' => stack.push_back(c),
            '[' => stack.push_back(c),
            ')' => {
                match stack.pop_back() {
                    Some('(') => {},
                    _ => return false
                }
            }
            '}' => {
                match stack.pop_back() {
                    Some('{') => {},
                    _ => return false
                }
            }
            ']' => {
                match stack.pop_back() {
                    Some('[') => {},
                    _ => return false
                }
            }
            _ => {}
        }
    }
    if stack.is_empty() {
        return true;
    }
    return false;
}

fn main() {
    assert_eq!(true, is_valid(String::from("()")));
    assert_eq!(true, is_valid(String::from("[]")));
    assert_eq!(true, is_valid(String::from("{}")));
    assert_eq!(true, is_valid(String::from("{[()]}")));
    assert_ne!(true, is_valid(String::from("(}")));
    assert_ne!(true, is_valid(String::from("(){[}]")));
    assert_ne!(true, is_valid(String::from("[hello")));
}
