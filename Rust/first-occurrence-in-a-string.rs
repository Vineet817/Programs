pub fn str_str(haystack: String, needle: String) -> i32 {
    let length:usize=needle.len();
    let mut idx:i32=-1;
     for i  in 0..(haystack.len()-length+1) {
       if haystack[i..(i+length as usize)] == needle {
           idx=i as i32;
           break;
       }
     }
     idx as i32

}

fn main(){
    let needle=String::from("hello");
    let haystack=String::from("blowhello");
    println!("{}",str_str(haystack, needle));
}