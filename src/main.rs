use json;

fn main() {
    // get input
    let json = " [ true, false] ";
    let out = json::JsonParser::parse(json);
    println!("{:?}", out);
}
