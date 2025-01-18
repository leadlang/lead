use interpreter::{function, methods, module, parse, pkg_name, types::BufValue};

module! {
  Types,
  pkg_name! { "📦 Lead Programming Language / Types" }
  methods! {
    function! {
      "str::to_int",
      
      |args, mut heap, file, _| {
        parse!(file + heap + args: > main, -> second);

        let val = BufValue::Faillable(
          {
            if let BufValue::Str(s) = second {
              s.parse::<i64>().map_or_else(|e| Err(e.to_string()), |x| Ok(Box::new(BufValue::Int(x))))
            } else {
              Err("Expected string".to_string())
            }
          }
        );

        heap.upgrade().set(main.into(), val);
      }
    }
  }
}
