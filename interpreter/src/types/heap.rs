use std::{
  borrow::Cow,
  collections::HashMap,
  fmt::Debug,
  future::Future,
  ops::{Deref, DerefMut},
  sync::Arc,
};

use crate::{
  error,
  parallel_ipreter::AsyncHeapHelper,
  runtime::{RuntimeValue, _root_syntax::RTCreatedModule},
  Application,
};

use super::{
  get_handle_runtime_ptr, handle_runtime, AppliesEq, BufValue, ExtendsInternal, HeapWrapper,
  Options, PackageCallback,
};

pub type HeapInnerMap = HashMap<Cow<'static, str>, BufValue>;

#[allow(private_interfaces)]
pub enum RawRTValue {
  RT(Box<dyn RuntimeValue>),
  PKG(HashMap<String, PackageCallback>),
  RTCM(RTCreatedModule),
}

impl Debug for RawRTValue {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "RawRTValue {{ 0x... }}")
  }
}

fn get_ptr(heap: &mut Heap) -> &mut HashMap<Cow<'static, str>, BufValue> {
  &mut heap.data
}

pub fn set_runtime_val(heap: &mut Heap, key: Cow<'static, str>, val: RawRTValue) {
  let _ = get_ptr(heap).insert(key, BufValue::RuntimeRaw(AppliesEq(val)));
}

pub fn get_runtime_ptr<'a>(
  heap: &'a mut AsyncHeapHelper,
  key: &'a str,
  file: &'a str,
  line: &usize,
) -> Option<*const ()> {
  let hp = heap.deref_mut() as *mut Heap;
  let ptr = get_ptr(heap);

  let (key, caller) = key.split_once("::")?;

  let data = match ptr.get(key)? {
    BufValue::RuntimeRaw(bi) => bi,
    val => return get_handle_runtime_ptr(unsafe { &mut *hp }, val, caller),
  };

  match &data.0 {
    RawRTValue::RT(data) => Some(&*data as *const _ as _),
    RawRTValue::PKG(pkg) => match pkg.get(caller) {
      Some(x) => Some(x as *const _ as *const ()),
      None => error(
        &format!("Unexpected `{}`", &caller),
        format!("{file}:{line}"),
      ),
    },
    RawRTValue::RTCM(pkg) => Some(pkg as *const _ as *const ()),
  }
}

macro_rules! implement {
  ($($struct:ident),*) => {
    $(
      impl<T: ?Sized> Clone for $struct<T> {
        fn clone(&self) -> Self {
          Self(self.0)
        }
      }

      impl<T: ?Sized> Copy for $struct<T> {}
      unsafe impl<T: ?Sized> Send for $struct<T> {}
      unsafe impl<T: ?Sized> Sync for $struct<T> {}
      impl<T: ?Sized> Deref for $struct<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
          unsafe { &*self.0 }
        }
      }
    )*
  };
}

pub(crate) struct SafePtrMut<T: ?Sized>(pub(crate) *mut T);
pub(crate) struct SafePtr<T: ?Sized>(pub(crate) *const T);

implement! {
  SafePtr,
  SafePtrMut
}

pub(crate) fn call_runtime_val(
  app: SafePtrMut<Application<'static>>,
  heap: SafePtrMut<Heap>,
  key: SafePtr<str>,
  v: SafePtr<[&'static str]>,
  a: HeapWrapper<'static>,
  o: SafePtrMut<Options>,
  file: SafePtr<str>,
  line: SafePtr<usize>,
) -> Option<impl Future<Output = ()>> {
  let c = format!("{}:{}", &*file, &*line);

  let hp = heap.0;
  let ptr = get_ptr(unsafe { &mut *hp });

  let (key, caller) = unsafe { &*key.0 }.split_once("::")?;

  let data = match ptr.get_mut(key)? {
    BufValue::RuntimeRaw(bi) => bi,
    val => {
      _ = handle_runtime(unsafe { &mut *hp }, val, caller, v.deref(), a, &c, unsafe {
        &mut *o.0
      });
      return None;
    }
  };

  match &mut data.0 {
    RawRTValue::RT(data) => data.call_ptr(caller, v.0, a, &c, unsafe { &mut *o.0 })?,
    RawRTValue::PKG(pkg) => match pkg.get(caller) {
      Some(x) => x.call((v.0, a, &c, unsafe { &mut *o.0 })),
      None => error(&format!("Unexpected `{}`", &caller), &c),
    },
    RawRTValue::RTCM(pkg) => {
      return Some(async move {
        let tkns = &unsafe { &*v.0 }[1..];

        pkg.run_method(
          app.0,
          caller,
          file.deref(),
          |fn_heap, app_heap, args| {
            if tkns.len() != args.len() {
              error(
                "Not all arguments provided",
                ":interpreter:loadmodule:heap:check",
              );
            }

            tkns.into_iter().zip(args.iter()).for_each(|(token, arg)| {
              let token = *token;
              let from = app_heap
                .remove(token)
                .unwrap_or_else(|| {
                  error(
                    format!("Unable to get {token} from Heap"),
                    ":interpreter:loadmodule",
                  )
                })
                .unwrap_or_else(|| {
                  error(
                    format!("Unable to get {token} from Heap"),
                    ":interpreter:loadmodule",
                  )
                });

              fn_heap
                .set(Cow::Borrowed(unsafe { &*(&arg[2..] as *const str) }), from)
                .unwrap();
            });
          },
          unsafe { &mut *hp },
          unsafe { &mut *o.0 },
        ).await
      });
    }
  }

  None
}

// pub fn call_runtime_val<'a>(
//   app: *mut Application<'static>,
//   heap: &'a mut Heap,
//   key: &'a str,
//   v: &'a [&'static str],
//   a: HeapWrapper<'a>,
//   o: &'a mut Options,
//   file: &'a str,
//   line: &usize,
// ) -> Option<impl Future<Output = ()>> {
//   let c = format!("{file}:{line}");

//   let hp = heap as *mut Heap;
//   let ptr = get_ptr(heap);

//   let (key, caller) = key.split_once("::")?;

//   let data = match ptr.get_mut(key)? {
//     BufValue::RuntimeRaw(bi) => bi,
//     val => return handle_runtime(unsafe { &mut *hp }, val, caller, v, a, &c, o)
//       .map(|x| async { x }),
//   };
//   None
// }

#[derive(Debug)]
pub struct Heap {
  data: HeapInnerMap,
  this: Option<*mut Self>,
  pub(crate) def_extends: Arc<ExtendsInternal>,
  pub(crate) extends: ExtendsInternal,
}

unsafe impl Send for Heap {}
unsafe impl Sync for Heap {}

impl Heap {
  pub(crate) fn new(def_extends: Arc<ExtendsInternal>) -> Self {
    Self {
      data: HashMap::new(),
      this: None,
      def_extends,
      extends: ExtendsInternal::default(),
    }
  }

  pub(crate) fn new_with_this(this: *mut Self, def_extends: Arc<ExtendsInternal>) -> Self {
    Self {
      data: HashMap::new(),
      this: Some(this),
      def_extends,
      extends: ExtendsInternal::default(),
    }
  }

  pub(crate) fn get_extends(&self) -> (&ExtendsInternal, &Arc<ExtendsInternal>) {
    (&self.extends, &self.def_extends)
  }

  pub fn clear(&mut self) {
    *self = Self::new(self.def_extends.clone());
  }

  pub fn inner_heap(&mut self) -> &mut HeapInnerMap {
    &mut self.data
  }

  pub fn set(&mut self, key: Cow<'static, str>, val: BufValue) -> Option<()> {
    if key.starts_with("self.") {
      let key: &'static str = unsafe { &*(&key[5..] as *const str) };
      return unsafe { &mut *self.this? }.set(Cow::Borrowed(key), val);
    }

    if !key.starts_with("$") {
      return None;
    }

    self.data.insert(key, val);
    Some(())
  }

  pub fn get(&self, key: &str) -> Option<&BufValue> {
    if key.starts_with("self.$") {
      return unsafe { &mut *self.this? }.get(&key[5..]);
    }

    let val = self.data.get(key)?;

    match val {
      BufValue::Pointer(ptr) => {
        if ptr.is_null() {
          return None;
        }

        Some(unsafe { &**ptr })
      }
      BufValue::PointerMut(ptr) => {
        if ptr.is_null() {
          return None;
        }

        Some(unsafe { &**ptr })
      }
      e => Some(e),
    }
  }

  pub fn get_mut(&mut self, key: &str) -> Option<&mut BufValue> {
    if key.starts_with("self.->&$") {
      return unsafe { &mut *self.this? }.get_mut(&key[5..]);
    }

    if !key.starts_with("->&") {
      return None;
    }

    let key = key.get(3..)?;

    let val = self.data.get_mut(key)?;

    match val {
      BufValue::Pointer(_) => None,
      BufValue::PointerMut(ptr) => {
        if ptr.is_null() {
          return None;
        }

        Some(unsafe { &mut **ptr })
      }
      e => Some(e),
    }
  }

  pub fn remove(&mut self, key: &str) -> Option<Option<BufValue>> {
    if key.starts_with("self.->$") {
      return unsafe { &mut *self.this? }.remove(&key[5..]);
    }

    if !key.starts_with("->") {
      return None;
    }

    let key = key.get(2..)?;

    return Some(self.data.remove(key));
  }
}
