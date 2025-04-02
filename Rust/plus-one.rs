pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    for i in (0..digits.len()).rev() {
        if digits[i] < 9 {
            digits[i] += 1;
            return digits;
        }
        digits[i] = 0;
    }
    digits.insert(0, 1); // Insert '1' at the beginning in case of carry overflow
    digits
}
fn main() {
    let v: Vec<i32> = vec![1,2,3,4,5,6,7,8,9,1];
    println!("{:?}", plus_one(v));
}