use adventure_script::io;

fn main() {
    let result = (io::DEFAULT_IO.query)("", vec!["one","two","three"], true);
    println!("result: {}", result);
}