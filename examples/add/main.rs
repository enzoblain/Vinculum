use vinculum::functions::add;

fn main() {
    vinculum::runtime::init();

    let result = add(2, 3);
    println!("Result: {}", result);

    vinculum::runtime::shutdown();
}
