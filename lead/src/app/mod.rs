use chalk_rs::Chalk;


struct Options {
  
}

mod logo;

pub fn run(args: &[String], _chalk: &mut Chalk) {
  logo::render_lead_logo();
}