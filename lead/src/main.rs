use core::Core;

use interpreter::{package, Application, ImplPackage};

static mut DATA: String = String::new();

fn main() {
  let mut app = Application::new("app.pb".into());
  app.add_pkg(Core);

  app.add_pkg(
    ImplPackage::new()
      .add_method(":coll", |_, _, _| unsafe {
        DATA.push_str("test");
      })
      .add_method(":print", |_, _, _| {
        println!("{}", unsafe { DATA.as_str() });
      }),
  );

  app.add_pkg(package!(":test", |_, _, _| {
    println!("Test");
  }));

  #[cfg(debug_assertions)]
  app.list_cmds();

  app.run();
}
