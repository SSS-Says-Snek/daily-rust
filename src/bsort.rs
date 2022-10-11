use crate::utils::get_input;

fn bubble_sort<T: PartialOrd + Copy> (vecs: &mut [T]) {
    for i in (0..vecs.len()).rev() {
        for j in 0..i {
            if vecs[j] > vecs[j + 1] {
                (vecs[j + 1], vecs[j]) = (vecs[j], vecs[j + 1]);
            }
        }
    }
}

pub fn bsort() {
    println!();

    let mut e: [i128; 5] = [3, 5376, 136, 34, -15];
    bubble_sort(&mut e);
    println!("{e:?}");

    loop {
        let (input, break_out) = get_input("Enter list separated by spaces");
        if break_out {break};

        let mut nums: Vec<i32> = input.trim()
            .split_whitespace()
            .map(|c| c.parse().expect("Not a number!"))
            .collect();

        bubble_sort(&mut nums);
        println!("{nums:?}");
    }
}