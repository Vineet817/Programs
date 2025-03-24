impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();

        for ch in s.chars() {
            if ch == '(' || ch == '{' || ch == '[' {
                stack.push(ch);  // Push opening brackets
            } else {
                if let Some(top) = stack.pop() {  // Ensure stack is not empty before popping
                    if (ch == ')' && top != '(') ||
                        (ch == '}' && top != '{') ||
                        (ch == ']' && top != '[') {
                        return false;  // Mismatched closing bracket
                    }
                } else {
                    return false;  // Closing bracket without a corresponding opening bracket
                }
            }
        }

        stack.is_empty()  // If stack is empty, the brackets are balanced
    }



}
struct Solution;
fn main() {
    let stng:String = String::from("()");
    Solution::is_valid(stng);
}

