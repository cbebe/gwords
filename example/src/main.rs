use gwords_lib::gword;
use text_io::read;

fn main() {
    let input: String = read!("{}\n");

    println!("{:?}", gword(&input.to_string()));
}
