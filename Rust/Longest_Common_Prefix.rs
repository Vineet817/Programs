pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    if strs.is_empty() {
        return String::new();
    }
    let shortstr= strs.iter().max();
    println!("{:?}",shortstr);
    shortstr.as_ref().unwrap().to_string()
}
fn main() {
    let strs = vec!["from", "to", "is", "ino"];
    println!("{:?}",longest_common_prefix(strs));
}

