fn f_to_c(temp_f: f32) -> f32 {
    return (temp_f - 32.0) * (5.0 / 9.0);
}

fn c_to_f(temp_c: f32) -> f32 {
    return (9.0 / 5.0) * temp_c + 32.0;
}

fn main() {
    println!("Hello, world!");
    println!("{} C", f_to_c(75.0)); // 75 F to C
    println!("{} F", c_to_f(75.0)); // 75 C to F

    // These are inverse functions of each other, so running them in succession should return the original temperature.
    println!("{}", f_to_c(c_to_f(75.0))); // Test 1
    println!("{}", c_to_f(f_to_c(32.0))); // Test 2
}
