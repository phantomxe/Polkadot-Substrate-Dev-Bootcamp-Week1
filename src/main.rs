
fn concatenate_strings(a: &str, b: &str) -> String {
    let mut result = String::new();
    result.push_str(a);
    result.push_str(b);
    result
}

fn main() {
    let string1 = "My String 1";
    let string2 = "My String 2";

    println!("{}", concatenate_strings(string1, string2));
}
