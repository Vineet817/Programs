pub fn str_str(haystack: String, needle: String) -> i32 {
    haystack.find(&needle).map_or(-1, |idx| idx as i32)
}
fn main() {
    let haystack:String=String::from("helloworld");
    let neddle:String  =String::from("owo");
    println!("{}", str_str(haystack, neddle));
}