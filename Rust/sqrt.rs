pub fn my_sqrt(x: i32) -> i32 {
    x.isqrt() as i32
}
fn main() {
    let number:i32=12;
    println!("{}",my_sqrt_using_NRM(number));
}

// newton  raphson method
pub fn my_sqrt_using_NRM(x: i32) -> i32 {
    let mut guess=x;
    while (guess*guess - x) as f32 > 1e-2{
        guess= (guess+(x/guess))/2

    }
    guess as i32
}