fn get_nth_Arg(n: usize) -> String {
    std::env::args().nth(n).unwrap();
}

// Remember to put pub access modifier so that others can use this struct
pub struct Args {
    pub image_1: String,
    pub image_2: String,
    pub output: String
}