use json;

fn main() {
    // get input
    let json = " [ true, false] ";
    let parser: json::JsonParser = json::JsonParser::new(json);
    println!("{:?}", parser.output);
}
