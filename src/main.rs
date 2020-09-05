use om_hj::na;
use na::Vector2;
use std::fs::File;
use std::io::Write;

fn main() {
    let j0 = |x: Vector2<f64>| x[0].powi(2) + x[1].powi(2) - 20.0 * x[0] - 30.0 * x[1];
    let j1 = |x: Vector2<f64>| 2.0 * x[0] + 3.0 * x[1] - 13.0;
    let j2 = |x: Vector2<f64>| 2.0 * x[0] + x[1] - 10.0;
    let j  = |x: Vector2<f64>, coef: f64| j0(x) + coef * (j1(x).max(0.0).powi(2) + j2(x).max(0.0).powi(2));
    let coef = |count: usize| (count as f64).exp();
    let init_x = Vector2::new(0.0, 0.0);
    let init_per = 1.0;
    let eps = 1e-5;
    let x = om_hj::search_with_penalty_2d(init_x, init_per, eps, j, coef);
    println!("Hooke-Jeeves search");
    println!("x : {{{}, {}}}", x[0], x[1]);
    println!("J0: {}", j0(x));
    println!("J1: {}", j1(x));
    println!("J2: {}", j2(x));

    let etalon_x_norm = Vector2::new(2.0f64, 3.0).norm();
    let coef = |count: usize| (count as f64).exp();
    let init_x = Vector2::new(0.0, 0.0);
    let init_per = 1e-5;
    let mut file = File::create("hj.txt").unwrap();
    for n in 1..=800 {
        let x = om_hj::search_with_penalty_and_n_2d(init_x, init_per, n, j, coef);
        writeln!(&mut file, "{}", (x.norm() - etalon_x_norm).abs()).unwrap();
    }
}
