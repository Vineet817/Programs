pub fn is_match(s: String, p: String) -> bool {
    let s = s.as_bytes();
    let p = p.as_bytes();
    let (mut i, mut j) = (0, 0);
    let (mut star_idx, mut match_idx) = (None, 0);

    while i < s.len() {
        if j < p.len() && (p[j] == b'?' || s[i] == p[j]) {
            i += 1;
            j += 1;
        } else if j < p.len() && p[j] == b'*' {
            star_idx = Some(j);
            match_idx = i;
            j += 1;
        } else if let Some(si) = star_idx {
            j = si + 1;
            match_idx += 1;
            i = match_idx;
        } else {
            return false;
        }
    }

    while j < p.len() && p[j] == b'*' {
        j += 1;
    }

    j == p.len()
}
fn main() {
    println!("{}",is_match("aa".to_string(), "aa".to_string()));
    println!("{}",is_match("aa".to_string(), "?c".to_string()));
    println!("{}",is_match("aa".to_string(), "*".to_string()));
    println!("{}",is_match("aa".to_string(), "?a".to_string()));
    println!("{}",is_match("adceb".to_string(), "*a*b".to_string()));


}