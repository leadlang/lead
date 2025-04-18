#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use interpreter::{module, pkg_name, types::BufValue};
use lead_lang_macros::{define, define_prototypes};
fn main() {}
pub struct MyModule;
impl interpreter::Package for MyModule {
    fn name(&self) -> &'static [u8] {
        "📦 MyModule".as_bytes()
    }
    fn prototype_docs(&self) -> interpreter::PrototypeDocs {
        let mut proto = interpreter::PrototypeDocs::default();
        proto.int = {
            let mut map = std::collections::HashMap::new();
            map.insert("print", _inner_callable_print_doc);
            map
        };
        proto
    }
    fn prototype(&self) -> interpreter::Extends {
        interpreter::Extends {
            int: &[("print", print)],
            ..std::default::Default::default()
        }
    }
}
#[allow(non_upper_case_globals)]
static _inner_callable_print_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+",
    "*",
    "Print two variables\n\n\n## Format:\n\n\n### Print two variables\n\n```\n$data: print $a $b\n```\n\n",
];
#[allow(unused)]
fn print(
    me: *mut i64,
    args: *const [*const str],
    mut heap: interpreter::types::HeapWrapper,
    file: &String,
    opt: &mut interpreter::types::Options,
) {
    let _option_code_result = _call_print(me, args, heap, file, opt);
    opt.set_return_val(_option_code_result)
}
#[allow(unused)]
fn _call_print(
    me: *mut i64,
    args: *const [*const str],
    mut heap: interpreter::types::HeapWrapper,
    file: &String,
    opt: &mut interpreter::types::Options,
) -> BufValue {
    BufValue::Bool(true)
}
