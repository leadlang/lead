use interpreter::{error, function, get_as, get_mut, methods, module, parse, pkg_name, types::{BufKeyVal, BufValue}};

module!(
  Array,
  pkg_name! { "📦 Core / Array" }
  methods! {
    function!(
      "array::malloc", 
      r#"Allocate an array in memory
## Format:
```
$var: array::alloc
```
      "#,
      |_, _, _, opt| {
      opt.set_return_val(BufValue::Array(vec![]));
      }
    ),
    function!("array::push", r#"Push a value to an array
## Format:
```
array::push ->&$array ->$val
```

## Example:
```
$var: array::malloc
$to_push: malloc string Hello

array::push ->&$var ->$to_push
```
"#, |args, mut heap, file, _| {
      parse!(file + heap + args: str arr, -> value);

      get_mut!(file + heap: Array arr);

      if arr.len() as i64 == i64::MAX {
        error("Array length reached 9223372036854775807 (number overflow)", file);
      }

      arr.push(value);
    }),
    function!("array::push_if_cap_available", r#"Push if capacity available
## Format:
```
array::push_if_cap_available ->&array ->$val
```

⚠️ It may reject silent if size isn't within capacity, not recommended"#, |args, mut heap, file, _| {
      parse!(file + heap + args: str arr, -> value);

      get_mut!(file + heap: Array arr);

      let _ = arr.push_within_capacity(value);
    }),
    function!("array::pop", r"Removes the last element from an array & returns it
## Format:
```
# If you want to discard the result
array::pop ->&$array 

# If you want to collect it
$var: array::pop ->&$array
```    ", |args, mut heap, file, opt| {
      parse!(file + heap + args: str arr);

      get_mut!(file + heap: Array arr);

      if let Some(x) = arr.pop() {
        opt.set_return_val(x)
      }
    }),
    function!("array::len", r"Returns the length of the array
## Format:
```
$len: array::len ->&$array
```", |args, heap, file, opt| {
      parse!(file + heap + args: & arr);

      get_as!(file + heap: Array arr);

      opt.set_return_val(BufValue::U_Int(arr.len() as u64))
    }),
    function!("array::cap", r"Returns the capacity of the array
## Format:
```
$len: array::cap ->&$array
```", |args, heap, file, opt| {
      parse!(file + heap + args: & arr);

      get_as!(file + heap: Array arr);

      opt.set_return_val(BufValue::U_Int(arr.capacity() as u64))
    }),
    function!("array::clear", r"Completely clears the array
## Format:
```
array::clear ->&array
```

## Note:
It is advisable to drop() the array if you want to clear it from memory", |args, mut heap, file, _| {
      parse!(file + heap + args: str arr);

      get_mut!(file + heap: Array arr);

      arr.clear();
    }),
    function!("array::get", r"Gets a pointer reference to an element of an array
## Format:
```
*val: array::get ->&array 1
*val: array::get ->&array 32

-- OR --
*val: array::get ->&array $index
```", |args, heap, file, opt| {
      parse!(file + heap + args: str arr, str index);

      let arr_parsed = heap.get(&arr).unwrap_or_else(|| {
        error("Unable to get array", file);
      });
      get_as!(file + heap: Array arr_parsed);

      let index = match heap.get(&index) {
        Some(x) => match &x {
          &BufValue::U_Int(x) => *x as usize,
          &BufValue::Int(x) => *x as usize,
          _ => {
            opt.set_return_ptr("".into(), BufKeyVal::None);
            return;
          }
        }
        _ => {
          match index.parse::<usize>() {
            Ok(x) => x,
            Err(_) => {
              opt.set_return_ptr("".into(), BufKeyVal::None);
              return;
            }
          }
        }
      };

      if arr_parsed.len() >= index {
        opt.set_return_ptr("".into(), BufKeyVal::None);
      } else {
        opt.set_return_ptr(arr.into(), BufKeyVal::Array(index));
      }
    })
  }
);
