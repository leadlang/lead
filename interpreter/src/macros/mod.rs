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

#[macro_export]
macro_rules! rtval_name {
  ($x:literal) => {
    fn name(&self) -> &'static str {
      $x
    }
  };
}

#[macro_export]
macro_rules! runtime_value {
  (
    $struct:ident, 
    { $(pub $x:ident: $y:ty),* },
    $($t:tt)*
  ) => {
    pub struct $struct {
      $(
        pub $x: Option<$y>
      ),*
    }

    impl $struct {
      const fn new_const() -> Self {
        Self {
          $(
            $x: None
          ),*
        }
      }

      fn new(
        $(
          $x: $y
        ),*
      ) -> Self {
        Self {
          $(
            $x: Some($x)
          ),*
        }
      }
    }

    impl interpreter::RuntimeValue for $struct {
      $(
        $t
      )*
    }
  };
}

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
      (
        $name, 
        $($x)*
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
    let [_, $($y),*] = &(unsafe { &*$args })[..] else {
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
    let $y = {
      let $heap: &mut interpreter::types::HeapWrapper = unsafe { &mut *(&mut $heap as *mut _) };
      let Some(Some($y)) = $heap.remove($y) else {
        interpreter::error("Could not obtain Varible!", $file);
      };

      $y as interpreter::types::BufValue
    };
  };

  ($file:ident + $heap:ident: & $y:ident) => {
    let $y = {
      let $heap: &mut interpreter::types::HeapWrapper = unsafe { &mut *(&mut $heap as *mut _) };
      let Some($y) = $heap.get($y) else {
        interpreter::error("Varible not found!", $file);
      };

      $y as &interpreter::types::BufValue
    };
  };

  ($file:ident + $heap:ident: mut $y:ident) => {
    let $y = {
      let $heap: &mut interpreter::types::HeapWrapper = unsafe { &mut *(&mut $heap as *mut _) };
      let Some($y) = $heap.get_mut($y) else {
        interpreter::error("Varible not found!", $file);
      };
      let $y = $y as *mut interpreter::types::BufValue;
      let $y = unsafe { &mut *$y };

      $y
    };
  };
  ($file:ident + $heap:ident: str $y:ident) => {};
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
