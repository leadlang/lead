use interpreter::{error, function, get_as, get_mut, methods, module, parse, pkg_name, types::BufValue};

module!(
  Array,
  pkg_name! { "📦 Core / Array" }
  methods! {
    function!("array::malloc", |_, _, _, opt| {
      opt.set_return_val(BufValue::Array(vec![]));
    }),
    function!("array::push", |args, heap, file, _| {
      parse!(file + heap + args: str arr, -> value);

      get_mut!(file + heap: Array arr);

      if arr.len() as i64 == i64::MAX {
        error("Array length reached 9223372036854775807 (number overflow)", file);
      }

      arr.push(value);
    }),
    function!("array::push_if_cap_available", |args, heap, file, _| {
      parse!(file + heap + args: str arr, -> value);

      get_mut!(file + heap: Array arr);

      let _ = arr.push_within_capacity(value);
    }),
    function!("array::pop", |args, heap, file, opt| {
      parse!(file + heap + args: str arr);

      get_mut!(file + heap: Array arr);

      if let Some(x) = arr.pop() {
        opt.set_return_val(x)
      }
    }),
    function!("array::len", |args, heap, file, opt| {
      parse!(file + heap + args: & arr);

      get_as!(file + heap: Array arr);

      opt.set_return_val(BufValue::Int(arr.len() as i64))
    }),
    function!("array::capacity", |args, heap, file, opt| {
      parse!(file + heap + args: & arr);

      get_as!(file + heap: Array arr);

      opt.set_return_val(BufValue::Int(arr.capacity() as i64))
    }),
    function!("array::clear", |args, heap, file, _| {
      parse!(file + heap + args: str arr);

      get_mut!(file + heap: Array arr);

      arr.clear();
    }),
  }
);
