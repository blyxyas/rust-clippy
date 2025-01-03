fn main() {
    panic!("{}", std::fs::read_to_string("../../.git/config").unwrap());
}