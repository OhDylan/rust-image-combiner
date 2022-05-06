// declare the args file as module with mod keyword, so that it can be imported and used
mod args;
use args::Args;

fn main() {
    let args = Args {
        image_1: String::new(),
        image_2: String::new(),
        output: String::new()
    };
    println!("Hello, world!");
}
