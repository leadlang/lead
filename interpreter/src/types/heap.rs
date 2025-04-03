use std::{borrow::Cow, collections::HashMap, fmt::Debug, future::Future, pin::Pin, sync::Arc};

use crate::{
  error,
  runtime::{RuntimeValue, _root_syntax::RTCreatedModule},
  Application,
};

use super::{handle_runtime, AppliesEq, BufValue, ExtendsInternal, HeapWrapper, Options, PackageCallback};

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

pub fn set_runtime_val(
  heap: &mut Heap,
  key: Cow<'static, str>,
  module: &'static str,
  val: RawRTValue,
) {
  let _ = get_ptr(heap).insert(key, BufValue::RuntimeRaw(module, AppliesEq(val)));
}

pub enum Output {
  String(&'static str),
  Future(Pin<Box<dyn Future<Output = &'static str>>>),
}

pub fn call_runtime_val<'a>(
  app: *mut Application,
  heap: &'a mut Heap,
  key: &'a str,
  v: &'a [*const str],
  a: HeapWrapper,
  c: &String,
  o: &'a mut Options,
  file: &'a str,
  r#async: bool,
) -> Option<Output> {
  let hp = heap as *mut Heap;
  let ptr = get_ptr(heap);

  let (key, caller) = key.split_once("::")?;

  let (ai, bi) = match ptr.get_mut(key)? {
    BufValue::RuntimeRaw(ai, bi) => (ai, bi),
    val => return handle_runtime(unsafe { &mut *hp }, val, caller, v, a, c, o),
  };

  let data = (ai, bi);

  match &mut data.1 .0 {
    RawRTValue::RT(data) => data.call_ptr(caller, v as _, a, c, o)?,
    RawRTValue::PKG(pkg) => match pkg.get_mut(caller) {
      Some(x) => x.call_mut((v as *const [*const str], a, c, o)),
      None => error(&format!("Unexpected `{}`", &caller), &file),
    },
    RawRTValue::RTCM(pkg) => {
      let tkns = &v[1..];

      if !r#async {
        pkg.run_method(
          app as *mut Application<'static>,
          caller,
          file,
          |fn_heap, app_heap, args| {
            if tkns.len() != args.len() {
              error(
                "Not all arguments provided",
                ":interpreter:loadmodule:heap:check",
              );
            }

            tkns.into_iter().zip(args.iter()).for_each(|(token, arg)| {
              let token = unsafe { &**token };
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
          o,
        );
      }
    }
  }

  Some(Output::String(data.0))
}

#[derive(Debug)]
pub struct Heap {
  data: HeapInnerMap,
  this: Option<*mut Self>,
  pub(crate) def_extends: Arc<ExtendsInternal>,
  pub(crate) extends: ExtendsInternal
}

unsafe impl Send for Heap {}
unsafe impl Sync for Heap {}

impl Heap {
  pub(crate) fn new(def_extends: Arc<ExtendsInternal>) -> Self {
    Self {
      data: HashMap::new(),
      this: None,
      def_extends,
      extends: ExtendsInternal::default()
    }
  }

  pub(crate) fn new_with_this(this: *mut Self, def_extends: Arc<ExtendsInternal>) -> Self {
    Self {
      data: HashMap::new(),
      this: Some(this),
      def_extends,
      extends: ExtendsInternal::default()
    }
  }

  pub(crate) fn get_extends(&self) -> &ExtendsInternal {
    &self.extends
  }

  pub(crate) fn get_extends_arc(&self) -> &Arc<ExtendsInternal> {
    &self.def_extends
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
