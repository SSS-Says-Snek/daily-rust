use crate::utils::get_input;

const DELTA: f64 = 1e-8;

// It's just an approximation heheheha
fn derive<F: Fn(f64) -> f64>(f: F, x: f64) -> f64 {
    let x1 = x - DELTA;
    let x2 = x + DELTA;
    let y1 = f(x1);
    let y2 = f(x2);
    
    (y2 - y1) / (x2 - x1)
}

pub fn main() {
    loop {
        let (input, break_out) = get_input("Enter func (sin, cos, sqr)");
        if break_out {
            break;
        };

        let input = input.trim();
        let deriv_func = match input {
            "sin" => f64::sin,
            "cos" => f64::cos,
            "sqr" => |x: f64| x.powf(2.0),
            _ => continue
        };

        let (input, break_out) = get_input("Enter func value");
        if break_out {
            break;
        };

        let deriv_x: f64 = input.trim().parse().expect("That's not a number! >:(");
        let result = derive(deriv_func, deriv_x);
        println!("The result is: {result}");
    }
}