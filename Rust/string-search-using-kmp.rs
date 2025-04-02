
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack: Vec<char> = haystack.chars().collect();
        let needle: Vec<char> = needle.chars().collect();
        let m = haystack.len();
        let n = needle.len();

        if n == 0 {
            return 0;
        }

        // Step 1: Compute the LPS array
        let mut lps = vec![0; n];
        let mut len = 0;
        let mut i = 1;

        while i < n {
            if needle[i] == needle[len] {
                len += 1;
                lps[i] = len;
                i += 1;
            } else {
                if len != 0 {
                    len = lps[len - 1];
                } else {
                    lps[i] = 0;
                    i += 1;
                }
            }
        }

        // Step 2: Perform KMP search
        let mut i = 0; // haystack index
        let mut j = 0; // needle index

        while i < m {
            if haystack[i] == needle[j] {
                i += 1;
                j += 1;
            }

            if j == n {
                return (i - j) as i32;
            } else if i < m && haystack[i] != needle[j] {
                if j != 0 {
                    j = lps[j - 1];
                } else {
                    i += 1;
                }
            }
        }

        -1
    }