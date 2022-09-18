use compare_archive::file;

#[tokio::main]
async fn main() {
    file::read_zip_file().await;
}

// use std::env;
//
// fn main() {
//     let args: Vec<String> = env::args().collect();
//
//     let query = &args[1];
//     let filename = &args[2];
//
//     // {}を探しています
//     println!("Searching for {}", query);
//     // {}というファイルの中
//     println!("In file {}", filename);
// /$ cargo run test sample.txt
//    Compiling minigrep v0.1.0 (file:///projects/minigrep)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
//      Running `target/debug/minigrep test sample.txt`
// Searching for test
// In file sample.txt
// / }
