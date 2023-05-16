/*
f:[i,j] -> R
losowe a0
np f(a0) > 0
if f(a0 + epsilon) > f(a0) to f jest rosnąca w przedziale
    else if < f(a0) jest malejąca
jeśli miejsce zerowe powinno być później iej to: a1 = a0 + delta
jeśli miejsce zerowe powinno być wcześniej to: a1 = a0 - delta

jeśli przeskoczyliśmy miejsce zerowe, to: delta /= 2
*/
fn g(x: f64) -> f64{
    x*x + 4.0*x + 4.0
}

fn f_is_rising(x: f64, epsilon: f64, f: fn(f64) -> f64) -> bool{
    f(x) < f(x+epsilon)
}

fn met_newt(x0: f64, epsilon: f64, N: u128, f: fn(f64) -> f64) -> f64{
    let mut counter: u128 = 0;
    let mut a0: f64 = x0;
    let mut delta: f64 = 1.0;
    let mut prev_right: bool = true;
    loop{
        let mut rising = f_is_rising(a0, epsilon, f);
        if counter == N {
            return a0;
        }
        if rising {
            if f(a0) > 0.0 {
                //need to go back
                a0 -= delta;
                if counter != 0 && prev_right {
                    delta /= 2.0;
                }
                prev_right = false;
            } else {
                //need to go forward
                a0 += delta;
                if counter != 0 && !prev_right {
                    delta /= 2.0;
                }
                prev_right = true;
            }
        } else{
            //f is falling
            if f(a0) > 0.0 {
                //need to go forward
                a0 += delta;
                if counter != 0 && !prev_right {
                    delta /= 2.0;
                }
                prev_right = true;
            } else {
                //need to go back
                a0 -= delta;
                if counter != 0 && prev_right {
                    delta /= 2.0;
                }
                prev_right = false;
            }
        }
        counter += 1;
    }
}

fn main() {
    println!("{}", met_newt(0.8, 0.000000001, 100, g));
}
