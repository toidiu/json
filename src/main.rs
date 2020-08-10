use json;

fn main() {
    // get input
    let json = "truefalse";
    let out = json::JsonParser::parse(json);
    println!("{:?}", out);
}
