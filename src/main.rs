use json;

fn main() {
    // get input
    let json = "true";
    let out = json::JsonParser::parse(json);
    println!("{:?}", out);
}
