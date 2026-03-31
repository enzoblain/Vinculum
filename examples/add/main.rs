use vinculum_hs::functions::math::add;

#[vinculum_hs::main(haskell_directory = "examples/add")]
fn main() {
    let result = add(2, 3);
    println!("Result: {}", result);
}
