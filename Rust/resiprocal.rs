fn reciprocal(x:f64) ->f64{
    let mut guess:f64 = 1.0;
    for _ in 0..100{
        guess=guess*(2.0-x*guess );
    }
    guess
}
fn main(){
    println!("resiprocal:{}" , reciprocal(2.0));
}