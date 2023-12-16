fn main() {
    // Input
    let mut p_faulty = 1e-2;
    let iters = 10;

    // Validation
    assert!(0.0 <= p_faulty);
    assert!(p_faulty <= 1.0);

    enum S {
        W01,
        F01,
        W13,
        F13,
    }
    let mut p = [0.0; 4];

    trait A {}

    // Algo
    for m in 0..iters {
        println!("p_faulty @ m({m}) = {p_faulty}");
        // Given, uniform measurement dist cond working sensor
        let p_01_cond_working = (1.0 - 0.0) / 3.0;
        let p_13_cond_working = (3.0 - 1.0) / 3.0;
        let p_working = 1.0 - p_faulty;
        // P(A, B) = P(A | B) P(B)
        p[S::W01 as usize] = p_working * p_01_cond_working;
        p[S::W13 as usize] = p_working * p_13_cond_working;

        // P([0, 1) and faulty) + P([1, 3] and faulty) = P(faulty)
        // P([0, 1) and faulty) + 0 {given} = 0.01 {given}
        // P([0, 1) and faulty) = 0.01
        p[S::F01 as usize] = p_faulty;
        p[S::F13 as usize] = 0.0;

        // P(faulty | [0, 1)) = P(faulty and [0, 1)) / P([0, 1))
        // P(faulty | [0, 1)) = P(faulty and [0, 1)) / {P([0, 1) and faulty) + P([0, 1) and working)}
        let p_faulty_cond_01 = p[S::F01 as usize] / (p[S::F01 as usize] + p[S::W01 as usize]);
        p_faulty = p_faulty_cond_01;
    }
    println!("p_faulty final  = {p_faulty}");
}
