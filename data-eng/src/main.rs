
fn string_tests() {
    let a = "string a";
    let i = a.chars().nth(0);
    let last = a.chars().last();
    println!("0: {i:?}");
    println!("last: {last:?}");
}


fn exec() {
    string_tests()
}

fn main() {
    exec()
}
