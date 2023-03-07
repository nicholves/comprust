pub mod compressor;


fn main() {
    let output = compressor::compress_contents_from_string("Hello my name is Oleg Ivanovich Veselskiy");

    println!("{}", output);

    /*let mut decompress = false;
    let mut compress = false;
    let mut output_file = String::new();

    let args: Vec<_> = env::args().collect();

    if !(args.iter().find(|e| **e == "-compress".to_string())).is_none() {
                compress = true;
                println!("Compressing");
    }

    for (index, arg) in enumerate(&args) {
        if *arg == "-o".to_string() {
            output_file = args[index + 1];
            println!("setting output file");
        }
        
    }*/
}