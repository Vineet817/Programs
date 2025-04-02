pub fn search_insert(nums: &[i32], target: i32) -> i32 {
    let len = nums.len();
    if len == 0 {
        return 0;
    }

    let mid = len / 2;

    if nums[mid] == target {
        return mid as i32;
    } else if nums[mid] < target {
        return (mid as i32 + 1) + search_insert(&nums[mid + 1..], target);
    } else {
        return search_insert(&nums[..mid], target);
    }
}

fn main() {
    let haystack = vec![1,3,5,6];
    println!("{:?}", search_insert(&haystack, 7));
}