use std::env;


fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Currently takes argument 1 and compresses it badly");
    println!("{} compressed: {}", args[1], compress(&args[1]));
}


fn compress(data: &String) -> String {
    String::from(data)
}
