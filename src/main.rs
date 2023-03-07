pub mod compressor;

fn main() {
    let output = compressor::compress_contents_from_string("Hello my name is Oleg Ivanovich Veselskiy");

    println!("{}", output);
}
