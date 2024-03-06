use matrix::Matrix;

fn main() {
    let a = Matrix::new([
        [1, 2, 3],
        [1, 2, 3],
    ]);

    let b = Matrix::new([
        [1, 2],
        [3, 4],
        [5, 6],
    ]);

    let c = a * b;
    let d = b * a;

    println!("a * b = {}", c.to_string());
    println!("b * a = {}", d.to_string());
}