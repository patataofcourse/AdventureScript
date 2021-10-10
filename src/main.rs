use adventure_script::io::AdventureIO;

fn main() {
    let result = (AdventureIO::default().query)("", vec!["one", "two", "three"], true);
    println!("result: {}", result);
}
