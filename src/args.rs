fn get_nth_arg(n: usize) -> String {
    print!("{:?}", std::env::args());
    std::env::args().nth(n).unwrap()
}

// Remember to put pub access modifier so that others can use this struct
#[derive(Debug)]
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}

impl Args {
    pub fn new() -> Self {
        Args {
            // index 0 for the iterator is the path to the binary
            image_1: get_nth_arg(1),
            image_2: get_nth_arg(2),
            output: get_nth_arg(3)
        }
    }
}