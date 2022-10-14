use crate::utils::get_input;

fn bubble_sort<T: PartialOrd + Copy>(vecs: &mut [T]) {
    for i in (0..vecs.len()).rev() {
        for j in 0..i {
            if vecs[j] > vecs[j + 1] {
                (vecs[j + 1], vecs[j]) = (vecs[j], vecs[j + 1]);
            }
        }
    }
}

pub fn main() {
    println!();

    // See, it's generic!
    let mut e: [i128; 5] = [3, 5376, 136, 34, -15];
    bubble_sort(&mut e);
    println!("Unsorted: [3, 5376, 136, 34, -15]\nSorted: {e:?}\n");

    loop {
        let (input, break_out) = get_input("Enter list separated by spaces");
        if break_out {
            break;
        };

        let mut nums: Vec<i128> = input
            .split_whitespace()
            .map(|x| x.parse().expect("Not a number!"))
            .collect();

        bubble_sort(&mut nums);
        println!("{nums:?}");
    }
}
