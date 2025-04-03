pub fn add_binary(a: String, b: String) -> String {
    let num1 = i32::from_str_radix(&a, 2).unwrap();
    let num2 = i32::from_str_radix(&b, 2).unwrap();
    format!("{:b}", num1 + num2)
}
fn main() {
    println!("{}",add_binary("1".to_string(), "1".to_string()));
}