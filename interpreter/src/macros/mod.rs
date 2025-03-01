#[macro_export]
macro_rules! create_state {
  ($t:ty) => {
    static mut STATE: Option<std::collections::HashMap<String, $t>> = None;
  };
}

#[macro_export]
macro_rules! get_state {
  () => {
    unsafe {
      if let None = STATE {
        STATE = Some(std::collections::HashMap::new());
      }
      STATE.as_mut().unwrap()
    }
  };
}

#[macro_export]
macro_rules! module {
  ($struct:ident, $($x:tt)*) => {
    pub struct $struct;
    impl interpreter::Package for $struct {
      $($x)*
    }
  };
}

#[macro_export]
macro_rules! pkg_name {
  ($($x:tt)*) => {
    fn name(&self) -> &'static [u8] {
      $($x)*.as_bytes()
    }
  };
}

// #[macro_export]
// macro_rules! methods {
//   ($($x:ident),*) => {
//     fn doc(&self) -> std::collections::HashMap<&'static str, &'static str> {
//       interpreter::hashmap! {
//         $(stringify!($x) => _call_$x_doc),*
//       }
//     }

//     fn methods(&self) -> interpreter::types::MethodRes {
//       &[
//         $({
//           (stringify!($x), $x)
//         }),*
//       ]
//     }
//   };
// }

#[macro_export]
macro_rules! document {
  ($($x:tt)*) => {
    stringify!($($x)*)
  }
}

#[macro_export]
macro_rules! function {
  ($name:literal, $($x:tt)*) => {
    {
      ($name, $($x)*
      )
    }
  };
}

#[macro_export]
macro_rules! hashmap {
  ($($key:expr => $value:expr),* $(,)?) => {{
    let mut map = std::collections::HashMap::new();
    $( map.insert($key, $value); )*
    map
  }};
}

#[macro_export]
macro_rules! parse {
  ($file:ident + $heap:ident + $args:ident:) => {};

  ($file:ident + $heap:ident + $args:ident: $($x:tt $y:ident),*) => {
    #[allow(unused_variables)]
    let [_, $($y),*] = &$args[..] else {
      interpreter::error("Invalid Format!", $file);
    };

    $(
      let $y = unsafe { &**$y };
    )*

    $(interpreter::modify!($file + $heap: $x $y);)*;
  };
}

#[macro_export]
macro_rules! modify {
  ($file:ident + $heap:ident: -> $y:ident) => {
    let Some(Some($y)) = $heap.remove($y) else {
      interpreter::error("Could not obtain Varible!", $file);
    };
  };

  ($file:ident + $heap:ident: & $y:ident) => {
    let Some($y) = $heap.get($y) else {
      interpreter::error("Varible not found!", $file);
    };
  };

  ($file:ident + $heap:ident: mut $y:ident) => {
    let Some($y) = $heap.get_mut($y) else {
      interpreter::error("Varible not found!", $file);
    };
    let $y = $y as *mut _;
    let $y = unsafe { &mut *$y };
  };

  ($file:ident + $heap:ident: > $y:ident) => {
    interpreter::warn("WARN: Deprecated library feature `>`, migrate to the $: syntax instead!");
    if !$y.starts_with("$") && !$y.starts_with("*") {
      interpreter::error("Invalid Variable provided!\nNote: Mutable / Moved values may not be provided to what expects piped (`>`) values", $file);
    }
  };
  ($file:ident + $heap:ident: str $y:ident) => {};

  ($file:ident + $heap:ident: drop $y:ident) => {
    let $y: Vec<Option<String>> = vec![];
    drop($y);
  };
}

#[macro_export]
macro_rules! get_as {
  ($file:ident + $heap:ident: $ty:ident $y:ident) => {
    let interpreter::types::BufValue::$ty($y) = $y else {
      interpreter::error("Variable not using the expected type!", $file);
    };
  };
}

#[macro_export]
macro_rules! get_mut {
  ($file:ident + $heap:ident: mut $y:ident) => {
    let Some($y) = $heap.get_mut($y) else {
      interpreter::error("get_mut!: Varible not found!", $file);
    };
  };

  ($file:ident + $heap:ident: $ty:ident $y:ident) => {
    let Some(interpreter::types::BufValue::$ty($y)) = $heap.get_mut($y) else {
      interpreter::error("get_mut!: Varible not found for the given type!", $file);
    };
  };
}
