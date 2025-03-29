pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
let mut idx:usize= 0;
    for i in 0..nums.len(){
        if nums[i]!=val  {
            nums[idx] = nums[i];
            idx+=1;
        }

    }
    nums.truncate(idx);
    nums.len() as i32
}
fn main(){
    let mut num1 = vec![1, 2, 3,3, 4];
    println!("val = {}", remove_element(&mut num1, 3));
}