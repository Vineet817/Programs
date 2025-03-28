pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut idx = 0;
    while idx < nums.len() -1 {
        if nums[idx] == nums[idx + 1] {
            nums.remove(idx);
        }else {
            idx+=1;
        }

    }
    fn valid(num: &Vec<i32>) -> bool {
        for i in 0..num.len() {

            for j in i+1..num.len() {
                if num[j]==num[i] {return false}
            }
        }return true
    }
    if valid(nums) {
        return nums.len() as i32;
    }else{
       return  remove_duplicates( nums);
    }

}
fn main(){
    let mut nums1 = vec![1, 1, 2];
    println!("{}", remove_duplicates(&mut nums1));
}