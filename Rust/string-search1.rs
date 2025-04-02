pub fn str_str(haystack: String, needle: String) -> i32 {
    let n=needle.len();
    for i in 0..haystack.len()-1 {
        if i+n>haystack.len(){
            return -1 as i32;
        }
        if haystack[i..=(i+n)] == needle {
           return i as i32;
        }
    }
    -1
}
fn main() {
    let haystack:String=String::from("helloworld");
    let neddle:String  =String::from("owo");
    println!("{}", str_str(haystack, neddle));
}