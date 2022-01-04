use json_parser::lex;

fn main() {
    let input = "24".to_string();
    dbg!(lex(input));
}
