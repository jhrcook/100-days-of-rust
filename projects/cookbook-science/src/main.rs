extern crate approx;
extern crate ndarray;

use approx::assert_abs_diff_eq;
use ndarray::{arr1, arr2, Array, Array1};

fn main() {
    let functions = [
        adding_matrics,
        mult_matrics,
        mult_scalar_vector_matrix,
        vector_comparison,
        vector_normalization,
    ];
    for function in functions {
        function();
        println!("–––––––––––––––––––––––––––––––––––––––––––––––––––");
    }
}

fn adding_matrics() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);
    let sum_ab = &a + &b;
    println!("{}", a);
    println!("+");
    println!("{}", b);
    println!("=");
    println!("{}", sum_ab);
}

fn mult_matrics() {
    let a = arr2(&[[1, 2, 3], [4, 5, 6]]);
    let b = arr2(&[[6, 5, 4], [3, 2, 1]]);
    let b = b.t();
    println!("{}", a.dot(&b));
}

fn mult_scalar_vector_matrix() {
    let scalar = 4;
    let vector = arr1(&[1, 2, 3]);
    let matrix = arr2(&[[4, 5, 6], [7, 8, 9]]);
    let new_vector: Array1<_> = scalar * vector;
    let new_matrix = matrix.dot(&new_vector);
    println!("{}", new_matrix);
}

fn vector_comparison() {
    // Another way of constructing Arrays.
    let a = Array::from(vec![1., 2., 3., 4., 5.]);
    let b = Array::from(vec![5., 4., 3., 2., 1.]);
    let mut c = Array::from(vec![1., 2., 3., 4., 5.]);
    let d = Array::from(vec![5., 4., 3., 2., 1.]);

    // `z` consumes `a` and `b`, but `w` does not consume `c` nor `d`.
    let z = a + b;
    let w = &c + &d;

    // Use the `approx` crate for accurately comparing floats.
    // Make sure to use either feature "approx-0_5" or "approx" in 'ndarray' with the correct version of "approx" crate
    assert_abs_diff_eq!(z, Array::from(vec![6., 6., 6., 6., 6.]));

    println!("c = {}", c);
    c[0] = 10.;
    c[1] = 10.;

    assert_abs_diff_eq!(w, Array::from(vec![6., 6., 6., 6., 6.]));
}

fn vector_normalization() {


}
