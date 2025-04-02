impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let vector: Vec<&str> = s.split(' ').collect();
        if vector[vector.len() - 1]==None{
            vector.remove(vector.len() - 1);
        }
        return vector[vector.len() - 1].len() as i32;
    }
}