use chalk_rs::Chalk;

fn main() {
  let mut chalk = Chalk::new();

  let info = String::from_utf8_lossy(&[
    27, 91, 51, 52, 109, 27, 91, 49, 109, 73, 78, 70, 79, 58, 32, 27, 91, 109,
  ]);
  let warn = String::from_utf8_lossy(&[
    27, 91, 51, 51, 109, 27, 91, 49, 109, 87, 65, 82, 78, 58, 32, 27, 91, 109,
  ]);
  let err = String::from_utf8_lossy(&[
    27, 91, 51, 49, 109, 27, 91, 49, 109, 69, 82, 82, 58, 32, 27, 91, 109,
  ]);

  println!("{info}{warn}{err}");
}
