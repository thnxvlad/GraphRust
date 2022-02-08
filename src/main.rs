mod lib;
use lib::Graph;

fn main() {
    let file_path = String::from("input.tgf"); // put your file path
    let mut g: Graph<String, String> = Graph::read_from_file(file_path);
    g.write_to_file(String::from("output.tgf"));
}