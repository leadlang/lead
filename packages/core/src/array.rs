use interpreter::{error, get_as, module, pkg_name, types::{BufKeyVal, BufValue}};
use lead_lang_macros::{methods, define};

module! {
  Array,
  pkg_name! { "📦 Core / Array" }
  methods! {
    array::malloc=malloc,
    array::push=push,
    array::push_within_cap=push_if_cap_available,
    array::pop=pop,
    array::len=len,
    array::cap=cap,
    array::clear=clear,
    array::get=get
  }
}

#[define((
  desc: "Allocate an empty array in memory",
  usage: [
    (
      desc: "Allocating String",
      code: "$val: array::malloc"
    )
  ],
  notes: None
))]
fn malloc() -> BufValue {
  BufValue::Array(vec![])
}

#[define((
  desc: "Push a value to an array",
  usage: [
    (
      desc: "Pushing $val",
      code: "array::push ->&$array ->$val"
    )
  ],
  notes: None
))]
fn push(array: &mut BufValue, value: BufValue) {
  let BufValue::Array(array) = array else {
    error("Expected array", file);
  };

  array.push(value);
}

#[define((
  desc: "Push if capacity available",
  usage: [
    (
      desc: "Pushing a value $val",
      code: "array::push ->&$array ->$val"
    )
  ],
  notes: Some("⚠️ It may reject silently if size isn't within capacity, not recommended")
))]
fn push_if_cap_available(array: &mut BufValue, value: BufValue) {
  let BufValue::Array(array) = array else {
    error("Expected array", file);
  };

  let _ = array.push_within_capacity(value);
}

#[define((
  desc: "Removes the last element from an array & returns it",
  usage: [
    (
      desc: "If you want to discard the result",
      code: "array::pop ->&$array"
    ),
    (
      desc: "If you want to collect it",
      code: "$var: array::pop ->&$array\n$var: unwrap ->$var"
    )
  ],
  notes: None
))]
fn pop(array: &mut BufValue) -> BufValue {
  let BufValue::Array(array) = array else {
    error("Expected array", file);
  };

  BufValue::Faillable(array.pop().map_or_else(|| Err("Empty".into()), |x| Ok(Box::new(x))))
}

#[define((
  desc: "Returns the length of the array",
  usage: [
    (
      desc: "",
      code: "$len: array::len ->&$array"
    )
  ],
  notes: None
))]
fn len(array: &BufValue) -> BufValue {
  let BufValue::Array(array) = array else {
    error("Expected array", file);
  };

  BufValue::U_Int(array.len() as u64)
}

#[define((
  desc: "Returns the capacity of the array",
  usage: [
    (
      desc: "",
      code: "$len: array::cap ->&$array"
    )
  ],
  notes: None
))]
fn cap(array: &BufValue) -> BufValue {
  let BufValue::Array(array) = array else {
    error("Expected array", file);
  };

  BufValue::U_Int(array.capacity() as u64)
}

#[define((
  desc: "Clears the array",
  usage: [
    (
      desc: "",
      code: "array::clear ->&$array"
    )
  ],
  notes: None
))]
fn clear(array: &mut BufValue) {
  let BufValue::Array(array) = array else {
    error("Expected array", file);
  };


  array.clear();
}

#[define((
  desc: "Gets a pointer reference to an element of an array",
  usage: [
    (
      desc: "Directly mentioning index",
      code: "*val: array::get ->&$array 1"
    ),
    (
      desc: "Using an index variable",
      code: "*val: array::get ->&$array $index"
    )
  ],
  notes: None
))]
fn get(arr: &str, index: &str) -> (String, BufKeyVal) {
  let arr_parsed = heap.get(&arr).unwrap_or_else(|| {
    error("Unable to get array", file);
  });
  get_as!(file + heap: Array arr_parsed);

  let index = match heap.get(&index) {
    Some(x) => match &x {
      &BufValue::U_Int(x) => *x as usize,
      &BufValue::Int(x) => *x as usize,
      _ => {
        return ("".into(), BufKeyVal::None);
      }
    }
    _ => {
      match index.parse::<usize>() {
        Ok(x) => x,
        Err(_) => {
          return ("".into(), BufKeyVal::None);
        }
      }
    }
  };

  if arr_parsed.len() >= index {
    return ("".into(), BufKeyVal::None);
  } else {
    return (arr.into(), BufKeyVal::Array(index));
  }
}