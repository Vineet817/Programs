pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut merged = Vec::with_capacity(nums1.len() + nums2.len());
    let (mut i, mut j) = (0, 0);

    // Merge the two sorted arrays
    while i < nums1.len() && j < nums2.len() {
        if nums1[i] < nums2[j] {
            merged.push(nums1[i]);
            i += 1;
        } else {
            merged.push(nums2[j]);
            j += 1;
        }
    }

    // Add remaining elements from nums1
    while i < nums1.len() {
        merged.push(nums1[i]);
        i += 1;
    }

    // Add remaining elements from nums2
    while j < nums2.len() {
        merged.push(nums2[j]);
        j += 1;
    }

    // Find the median
    let len = merged.len();
    if len % 2 == 1 {
        merged[len / 2] as f64
    } else {
        (merged[len / 2 - 1] as f64 + merged[len / 2] as f64) / 2.0
    }
}
