fn main() {
    find_nemo("I am finding Nemo !");
    find_nemo("Nemo is me");
    find_nemo("I Nemo am");
    find_nemo("a simple phrase");
}

fn find_nemo(words: &str) {
    match words.find("Nemo") {
        Some(index) => println!("I found Nemo at {index}"),
        None => println!("I can't find Nemo :(")
    }
}