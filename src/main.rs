use json;

fn main() {
    // get input
    let json = "true and more";
    let out = json::JsonParser::parse(json);
    println!("{:?}", out);
}
