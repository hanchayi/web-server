fn main() {
    let r;
    let a = String::from("a");
    let b = String::from("b");

    {
        // let b = String::from("b");
        r = longest(&a, &b);
        println!("r, {}", r);
    }

    println!("r, {}", r);
}

fn longest<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        return str1
    } else {
        return str2
    }
}
