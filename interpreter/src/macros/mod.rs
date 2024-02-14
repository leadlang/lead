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
macro_rules! methods {
  ($($x:tt)*) => {
    fn methods(&self) -> interpreter::types::MethodRes {
      &[
        $($x)*
      ]
    }
  };
}

#[macro_export]
macro_rules! function {
  ($name:literal, $($x:tt)*) => {
    ($name, $($x)*
    )
  };
}

#[macro_export]
macro_rules! parse {
  ($heap:ident + $args:ident: $($x:tt $y:ident),*) => {
    #[allow(unused_variables)]
    let [_, $($y),*] = &$args[..] else {
      interpreter::error("Invalid Format!");
    };

    $(interpreter::modify!($heap: $x $y);)*;
  };
}

#[macro_export]
macro_rules! modify {
  ($heap:ident: -> $y:ident) => {
    let Some(Some($y)) = $heap.remove($y) else {
      interpreter::error("Invalid Format or Varible not found!");
    };
  };

  ($heap:ident: & $y:ident) => {
    let Some($y) = $heap.get($y) else {
      interpreter::error("Invalid Format or Varible not found!");
    };
  };

  ($heap:ident: mut $y:ident) => {
    let Some($y) = $heap.get_mut($y) else {
      interpreter::error("Invalid Format or Varible not found!");
    };
  };

  ($heap:ident: > $y:ident) => {
    if !$y.starts_with("$") && !$y.starts_with("*") {
      interpreter::error("Invalid Variable provided!\nNote: Mutable / Moved values may not be provided to what expects piped (`>`) values");
    }
  };
  ($heap:ident: str $y:ident) => {};

  ($heap:ident: drop $y:ident) => {
    let $y: Vec<Option<String>> = vec![];
    drop($y);
  };
}
