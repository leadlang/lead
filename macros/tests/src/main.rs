use lead_lang_macros::define;

fn main() {
    println!("Hello, world!");
    _call_call("".into(), "".into())
}

#[define]
fn call(a: String, b: String) {
    println!("{a} {b}");
}