#![feature(prelude_import)]
#![feature(vec_push_within_capacity)]
#![feature(concat_idents)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use core::str;
use std::env;
use interpreter::{
    phf, error, generate, module, pkg_name, types::{BufValue, HeapWrapper, Options},
};
use lead_lang_macros::{define, gendoc, methods};
mod array {
    use std::ptr;
    use interpreter::{error, get_as, module, pkg_name, types::BufValue};
    use lead_lang_macros::{define, methods};
    pub struct Array;
    impl interpreter::Package for Array {
        fn name(&self) -> &'static [u8] {
            "📦 Core / Array".as_bytes()
        }
        fn doc(
            &self,
        ) -> std::collections::HashMap<&'static str, &'static [&'static str; 3]> {
            {
                let mut map = std::collections::HashMap::new();
                map.insert("array::malloc", _inner_callable_malloc_doc);
                map.insert("array::push", _inner_callable_push_doc);
                map.insert(
                    "array::push_within_cap",
                    _inner_callable_push_if_cap_available_doc,
                );
                map.insert("array::pop", _inner_callable_pop_doc);
                map.insert("array::len", _inner_callable_len_doc);
                map.insert("array::cap", _inner_callable_cap_doc);
                map.insert("array::clear", _inner_callable_clear_doc);
                map.insert("array::get", _inner_callable_get_doc);
                map
            }
        }
        fn methods(&self) -> interpreter::types::MethodRes {
            &[
                ("array::malloc", malloc),
                ("array::push", push),
                ("array::push_within_cap", push_if_cap_available),
                ("array::pop", pop),
                ("array::len", len),
                ("array::cap", cap),
                ("array::clear", clear),
                ("array::get", get),
            ]
        }
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_malloc_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+",
        "*",
        "Allocate an empty array in memory\n\n\n## Format:\n\n\n### Allocating String\n\n```\n$val: array::malloc\n```\n\n",
    ];
    #[allow(unused)]
    fn malloc(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        let _option_code_result = _call_malloc(args, heap, file, opt);
        opt.set_return_val(_option_code_result)
    }
    #[allow(unused)]
    fn _call_malloc(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) -> BufValue {
        BufValue::Array(::alloc::vec::Vec::new())
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_push_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ ->&\\$[a-z0-9_]+ ->\\$[a-z0-9_]+$",
        "",
        "Push a value to an array\n# Function Params\n\n```\n->&$array ->$value \n```\n\n## Format:\n\n\n### Pushing $val\n\n```\narray::push ->&$array ->$val\n```\n\n",
    ];
    #[allow(unused)]
    fn push(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, array, value] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let array = unsafe { &**array };
        let value = unsafe { &**value };
        let array = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(array) = heap.get_mut(array) else {
                interpreter::error("Varible not found!", file);
            };
            let array = array as *mut interpreter::types::BufValue;
            let array = unsafe { &mut *array };
            array
        };
        let value = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(Some(value)) = heap.remove(value) else {
                interpreter::error("Could not obtain Varible!", file);
            };
            value as interpreter::types::BufValue
        };
        let _option_code_result = _call_push(array, value, file, opt);
    }
    #[allow(unused)]
    fn _call_push(
        array: &mut BufValue,
        value: BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        let BufValue::Array(array) = array else {
            error("Expected array", file);
        };
        array.push(value);
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_push_if_cap_available_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ ->&\\$[a-z0-9_]+ ->\\$[a-z0-9_]+$",
        "",
        "Push if capacity available\n# Function Params\n\n```\n->&$array ->$value \n```\n\n## Format:\n\n\n### Pushing a value $val\n\n```\narray::push ->&$array ->$val\n```\n\n## Notes:\n⚠\u{fe0f} It may reject silently if size isn't within capacity, not recommended",
    ];
    #[allow(unused)]
    fn push_if_cap_available(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, array, value] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let array = unsafe { &**array };
        let value = unsafe { &**value };
        let array = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(array) = heap.get_mut(array) else {
                interpreter::error("Varible not found!", file);
            };
            let array = array as *mut interpreter::types::BufValue;
            let array = unsafe { &mut *array };
            array
        };
        let value = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(Some(value)) = heap.remove(value) else {
                interpreter::error("Could not obtain Varible!", file);
            };
            value as interpreter::types::BufValue
        };
        let _option_code_result = _call_push_if_cap_available(array, value, file, opt);
    }
    #[allow(unused)]
    fn _call_push_if_cap_available(
        array: &mut BufValue,
        value: BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        let BufValue::Array(array) = array else {
            error("Expected array", file);
        };
        let _ = array.push_within_capacity(value);
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_pop_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ ->&\\$[a-z0-9_]+$",
        "*",
        "Removes the last element from an array & returns it\n# Function Params\n\n```\n->&$array \n```\n\n## Format:\n\n\n### If you want to discard the result\n\n```\narray::pop ->&$array\n```\n\n### If you want to collect it\n\n```\n$var: array::pop ->&$array\n$var: unwrap ->$var\n```\n\n",
    ];
    #[allow(unused)]
    fn pop(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, array] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let array = unsafe { &**array };
        let array = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(array) = heap.get_mut(array) else {
                interpreter::error("Varible not found!", file);
            };
            let array = array as *mut interpreter::types::BufValue;
            let array = unsafe { &mut *array };
            array
        };
        let _option_code_result = _call_pop(array, file, opt);
        opt.set_return_val(_option_code_result)
    }
    #[allow(unused)]
    fn _call_pop(
        array: &mut BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) -> BufValue {
        let BufValue::Array(array) = array else {
            error("Expected array", file);
        };
        BufValue::Faillable(
            array.pop().map_or_else(|| Err("Empty".into()), |x| Ok(Box::new(x))),
        )
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_len_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ \\$[a-z0-9_]+$",
        "*",
        "Returns the length of the array\n# Function Params\n\n```\n$array \n```\n\n## Format:\n\n\n### \n\n```\n$len: array::len ->&$array\n```\n\n",
    ];
    #[allow(unused)]
    fn len(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, array] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let array = unsafe { &**array };
        let array = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(array) = heap.get(array) else {
                interpreter::error("Varible not found!", file);
            };
            array as &interpreter::types::BufValue
        };
        let _option_code_result = _call_len(array, file, opt);
        opt.set_return_val(_option_code_result)
    }
    #[allow(unused)]
    fn _call_len(
        array: &BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) -> BufValue {
        let BufValue::Array(array) = array else {
            error("Expected array", file);
        };
        BufValue::U_Int(array.len() as u64)
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_cap_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ \\$[a-z0-9_]+$",
        "*",
        "Returns the capacity of the array\n# Function Params\n\n```\n$array \n```\n\n## Format:\n\n\n### \n\n```\n$len: array::cap ->&$array\n```\n\n",
    ];
    #[allow(unused)]
    fn cap(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, array] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let array = unsafe { &**array };
        let array = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(array) = heap.get(array) else {
                interpreter::error("Varible not found!", file);
            };
            array as &interpreter::types::BufValue
        };
        let _option_code_result = _call_cap(array, file, opt);
        opt.set_return_val(_option_code_result)
    }
    #[allow(unused)]
    fn _call_cap(
        array: &BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) -> BufValue {
        let BufValue::Array(array) = array else {
            error("Expected array", file);
        };
        BufValue::U_Int(array.capacity() as u64)
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_clear_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ ->&\\$[a-z0-9_]+$",
        "",
        "Clears the array\n# Function Params\n\n```\n->&$array \n```\n\n## Format:\n\n\n### \n\n```\narray::clear ->&$array\n```\n\n",
    ];
    #[allow(unused)]
    fn clear(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, array] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let array = unsafe { &**array };
        let array = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(array) = heap.get_mut(array) else {
                interpreter::error("Varible not found!", file);
            };
            let array = array as *mut interpreter::types::BufValue;
            let array = unsafe { &mut *array };
            array
        };
        let _option_code_result = _call_clear(array, file, opt);
    }
    #[allow(unused)]
    fn _call_clear(
        array: &mut BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        let BufValue::Array(array) = array else {
            error("Expected array", file);
        };
        array.clear();
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_get_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ \\$[a-zA-Z0-9_]* (\\$[a-zA-Z0-9_]*|\"[a-zA-Z0-9]*\"|[0-9]*)$",
        "*",
        "Gets a pointer reference to an element of an array\n# Function Params\n\n```\n<arr> <index> \n```\n\n## Format:\n\n\n### Directly mentioning index\n\n```\n$val: array::get $array 1\n```\n\n### Using an index variable\n\n```\n$val: array::get $array $index\n```\n\n",
    ];
    #[allow(unused)]
    fn get(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, arr, index] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let arr = unsafe { &**arr };
        let index = unsafe { &**index };
        let _option_code_result = _call_get(arr, index, file, heap, opt);
        opt.set_return_val(_option_code_result)
    }
    #[allow(unused)]
    fn _call_get(
        arr: &str,
        index: &str,
        file: &str,
        mut heap: interpreter::types::HeapWrapper,
        opt: &mut interpreter::types::Options,
    ) -> BufValue {
        let arr_parsed = heap
            .get(&arr)
            .unwrap_or_else(|| {
                error("Unable to get array", file);
            });
        let interpreter::types::BufValue::Array(arr_parsed) = arr_parsed else {
            interpreter::error("Variable not using the expected type!", file);
        };
        let index = match heap.get(&index) {
            Some(x) => {
                match &x {
                    &BufValue::U_Int(x) => *x as usize,
                    &BufValue::Int(x) => *x as usize,
                    _ => {
                        return BufValue::Pointer(ptr::null());
                    }
                }
            }
            _ => {
                match index.parse::<usize>() {
                    Ok(x) => x,
                    Err(_) => {
                        return BufValue::Pointer(ptr::null());
                    }
                }
            }
        };
        if arr_parsed.len() >= index {
            return BufValue::Pointer(ptr::null());
        } else {
            return BufValue::Pointer(&arr_parsed[index]);
        }
    }
}
mod type_conv {
    use interpreter::{module, pkg_name, types::BufValue};
    use lead_lang_macros::{define, methods};
    pub struct Types;
    impl interpreter::Package for Types {
        fn name(&self) -> &'static [u8] {
            "📦 Core / Types Conversion".as_bytes()
        }
        fn doc(
            &self,
        ) -> std::collections::HashMap<&'static str, &'static [&'static str; 3]> {
            {
                let mut map = std::collections::HashMap::new();
                map.insert("str::to_int", _inner_callable_to_int_doc);
                map
            }
        }
        fn methods(&self) -> interpreter::types::MethodRes {
            &[("str::to_int", to_int)]
        }
    }
    #[allow(non_upper_case_globals)]
    static _inner_callable_to_int_doc: &'static [&'static str; 3] = &[
        "^(\\$?)[a-z0-9_:]+ ->\\$[a-z0-9_]+$",
        "*",
        "Parse string into int\n# Function Params\n\n```\n->$val \n```\n\n## Format:\n\n\n### \n\n```\n$val: str::to_int ->$val\n$val: unwrap ->$val\n```\n\n## Notes:\nThe result is a Faillable as the parsing might fail as well",
    ];
    #[allow(unused)]
    fn to_int(
        args: *const [&'static str],
        mut heap: interpreter::types::HeapWrapper,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) {
        #[allow(unused_variables)]
        let [_, val] = &(unsafe { &*args })[..] else {
            interpreter::error("Invalid Format!", file);
        };
        let val = unsafe { &**val };
        let val = {
            let heap: &mut interpreter::types::HeapWrapper = unsafe {
                &mut *(&mut heap as *mut _)
            };
            let Some(Some(val)) = heap.remove(val) else {
                interpreter::error("Could not obtain Varible!", file);
            };
            val as interpreter::types::BufValue
        };
        let _option_code_result = _call_to_int(val, file, opt);
        opt.set_return_val(_option_code_result)
    }
    #[allow(unused)]
    fn _call_to_int(
        val: BufValue,
        file: &str,
        opt: &mut interpreter::types::Options,
    ) -> BufValue {
        BufValue::Faillable({
            if let BufValue::Str(s) = val {
                s.parse::<i64>()
                    .map_or_else(
                        |e| Err(e.to_string()),
                        |x| Ok(Box::new(BufValue::Int(x))),
                    )
            } else {
                Err(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Expected string, found {0}", val.type_of()),
                        );
                        res
                    }),
                )
            }
        })
    }
}
use array::Array;
use type_conv::Types;
pub struct Core;
impl interpreter::Package for Core {
    fn name(&self) -> &'static [u8] {
        "📦 Core / Memory".as_bytes()
    }
    fn doc(
        &self,
    ) -> std::collections::HashMap<&'static str, &'static [&'static str; 3]> {
        {
            let mut map = std::collections::HashMap::new();
            map.insert("fmt", _inner_callable_fmt_optimized_doc);
            map.insert("malloc", _inner_callable_malloc_doc);
            map.insert("unwrap", _inner_callable_unwrap_doc);
            map.insert("drop", _inner_callable_memclear_doc);
            map.insert("comp", _inner_callable_comp_doc);
            map.insert("typeof", _inner_callable_kindof_doc);
            map.insert("env", _inner_callable_env_doc);
            map
        }
    }
    fn methods(&self) -> interpreter::types::MethodRes {
        &[
            ("fmt", fmt_optimized),
            ("malloc", malloc),
            ("unwrap", unwrap),
            ("drop", memclear),
            ("comp", comp),
            ("typeof", kindof),
            ("env", env),
        ]
    }
}
#[allow(non_upper_case_globals)]
static _inner_callable_env_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+ \".*\"$",
    "*",
    "Get environment variable\n# Function Params\n\n```\n<val> \n```\n\n## Format:\n\n\n### Example\n\n```\n$secret: env NAME\n```\n\n",
];
#[allow(unused)]
fn env(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) {
    #[allow(unused_variables)]
    let [_, val] = &(unsafe { &*args })[..] else {
        interpreter::error("Invalid Format!", file);
    };
    let val = unsafe { &**val };
    let _option_code_result = _call_env(val, file, heap, opt);
    opt.set_return_val(_option_code_result)
}
#[allow(unused)]
fn _call_env(
    val: &str,
    file: &str,
    mut heap: interpreter::types::HeapWrapper,
    opt: &mut interpreter::types::Options,
) -> BufValue {
    BufValue::Faillable(
        env::var(val)
            .map_or_else(
                |x| Err(
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0}", x));
                        res
                    }),
                ),
                |x| Ok(Box::new(BufValue::Str(x))),
            ),
    )
}
#[allow(non_upper_case_globals)]
static _inner_callable_fmt_optimized_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+",
    "*",
    "Format String\n\n\n## Format:\n\n\n### Example\n\n```\n$val: fmt \"This is a format string $var\"\n```\n\n### Retaining \\\n\n```\n$val: fmt \"This will retain \\\\\"\n```\n\n### Retaining $\n\n```\n$val: fmt \"This will retain \\$\"\n```\n\n",
];
#[allow(unused)]
fn fmt_optimized(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) {
    let _option_code_result = _call_fmt_optimized(args, heap, file, opt);
    opt.set_return_val(_option_code_result)
}
#[allow(unused)]
fn _call_fmt_optimized(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) -> BufValue {
    let mut chars = unsafe { &(&*args)[1..] }
        .iter()
        .map(|s| unsafe { &**s }.chars())
        .enumerate()
        .map(|(i, c)| { if i > 0 { " ".chars().chain(c) } else { "".chars().chain(c) } })
        .flatten();
    let mut output = String::with_capacity(chars.size_hint().1.unwrap_or(32));
    let heap = heap.upgrade();
    if Some('"') != chars.next() {
        error("The formatter must start with `\"`", file)
    }
    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next() {
                    match next {
                        '\\' | '$' | '\"' => output.push(next),
                        'n' => output.push('\n'),
                        'r' => output.push('\r'),
                        't' => output.push('\t'),
                        '0' => output.push('\0'),
                        e => {
                            error(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Expected n or \\ or $ after \\, found {0}", e),
                                    );
                                    res
                                }),
                                file,
                            )
                        }
                    }
                } else {
                    error(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Expected n or \\ or $ after \\, found EOF"),
                            );
                            res
                        }),
                        file,
                    )
                }
            }
            '$' => {
                let mut data = String::with_capacity(16);
                data.push('$');
                let mut begun = false;
                while let Some(next) = chars.next() {
                    match next {
                        '{' => begun = true,
                        '}' => break,
                        e => {
                            if begun {
                                data.push(e);
                            } else {
                                error("The variable name must be contained in \"{\"", file);
                            }
                        }
                    }
                }
                let Some(value) = heap.get(&data) else {
                    {
                        ::core::panicking::panic_fmt(
                            format_args!("Variable {0} not found", data),
                        );
                    };
                };
                output.push_str(&value.display());
            }
            '"' => {
                if let Some(x) = chars.next() {
                    error(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Expected EOL, found {0}", x),
                            );
                            res
                        }),
                        file,
                    );
                }
            }
            c => output.push(c),
        }
    }
    BufValue::Str(output)
}
#[allow(non_upper_case_globals)]
static _inner_callable_malloc_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+ (bool|u_int|int|float|string) (\".*\"|true|false|-?[0-9]*.?[0-9]*)$",
    "*",
    "Allocate Memory\n\n\n## Format:\n\n\n### Allocating String\n\n```\n$val: malloc string \"Hello World\"\n```\n\n### Allocating Number\n\n```\n$val: malloc string \"Hello World\"\n```\n\n## Notes:\nAvailable Types ---\n- bool Boolean (eg. malloc bool true)\n- int 64-Bit Signed Integer (eg. -3, 3)\n- u_int 64-Bit Unsigned Integer (eg. 0, 3)\n- float Floating point number (eg. 1.04, 10.6)\n- string String (eg. \"Hello World\")",
];
fn malloc(args: *const [*const str], _: HeapWrapper, file: &String, opt: &mut Options) {
    let [_, typ, ..] = &(unsafe { &*args })[..] else {
        error(
            r#"Invalid arguments in :malloc
Format ---
- malloc type data

Types ---
- bool Boolean (eg. true)
- int Integer (eg. -3, 3)
- u_int Unsigned Integer (eg. 3)
- float Floating point number (eg. 1.04)
- string String (eg. "Hello World")
"#,
            file,
        );
    };
    let typ = unsafe { &**typ };
    let data = unsafe { &*args }[2..]
        .iter()
        .map(|x| unsafe { &**x })
        .collect::<Vec<_>>()
        .join(" ");
    opt.set_return_val(
        match typ {
            "bool" => BufValue::Bool(&data == "true"),
            "int" => {
                BufValue::Int(
                    data
                        .parse()
                        .map_or_else(
                            |_| error("Unable to convert to INTEGER", file),
                            |x| x,
                        ),
                )
            }
            "u_int" => {
                BufValue::U_Int(
                    data
                        .parse()
                        .map_or_else(
                            |_| error("Unable to convert to UNSIGNED INTEGER", file),
                            |x| x,
                        ),
                )
            }
            "float" => {
                BufValue::Float(
                    data
                        .parse()
                        .map_or_else(
                            |_| error("Unable to convert to FLOAT", file),
                            |x| x,
                        ),
                )
            }
            "string" => {
                BufValue::Str(
                    serde_json::from_str(&data)
                        .map_or_else(
                            |_| error("Unable to convert to STRING", file),
                            |x| x,
                        ),
                )
            }
            e => {
                error(
                    &::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Invalid type, {0}", e),
                        );
                        res
                    }),
                    file,
                )
            }
        },
    );
}
#[allow(non_upper_case_globals)]
static _inner_callable_unwrap_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+ ->\\$[a-z0-9_]+$",
    "*",
    "Unwrap value\n# Function Params\n\n```\n->$val \n```\n\n## Format:\n\n\n### \n\n```\n$val: unwrap ->$val\n```\n\n",
];
#[allow(unused)]
fn unwrap(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) {
    #[allow(unused_variables)]
    let [_, val] = &(unsafe { &*args })[..] else {
        interpreter::error("Invalid Format!", file);
    };
    let val = unsafe { &**val };
    let val = {
        let heap: &mut interpreter::types::HeapWrapper = unsafe {
            &mut *(&mut heap as *mut _)
        };
        let Some(Some(val)) = heap.remove(val) else {
            interpreter::error("Could not obtain Varible!", file);
        };
        val as interpreter::types::BufValue
    };
    let _option_code_result = _call_unwrap(val, file, opt);
    opt.set_return_val(_option_code_result)
}
#[allow(unused)]
fn _call_unwrap(
    val: BufValue,
    file: &str,
    opt: &mut interpreter::types::Options,
) -> BufValue {
    match val {
        BufValue::Faillable(val) => {
            match val {
                Ok(val) => *val,
                Err(err) => {
                    error(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(format_args!("{0}", err));
                            res
                        }),
                        file,
                    );
                }
            }
        }
        _ => error("Expected Faillable(Result<T, E>) in `-> val`", file),
    }
}
#[allow(non_upper_case_globals)]
static _inner_callable_memclear_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+ ->\\$[a-z0-9_]+$",
    "",
    "Removes the value from memory\n# Function Params\n\n```\n->$_val \n```\n\n## Format:\n\n\n### Example\n\n```\nmemclear ->$val\n```\n\n",
];
#[allow(unused)]
fn memclear(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) {
    #[allow(unused_variables)]
    let [_, _val] = &(unsafe { &*args })[..] else {
        interpreter::error("Invalid Format!", file);
    };
    let _val = unsafe { &**_val };
    let _val = {
        let heap: &mut interpreter::types::HeapWrapper = unsafe {
            &mut *(&mut heap as *mut _)
        };
        let Some(Some(_val)) = heap.remove(_val) else {
            interpreter::error("Could not obtain Varible!", file);
        };
        _val as interpreter::types::BufValue
    };
    let _option_code_result = _call_memclear(_val, file, opt);
}
#[allow(unused)]
fn _call_memclear(_val: BufValue, file: &str, opt: &mut interpreter::types::Options) {}
#[allow(non_upper_case_globals)]
static _inner_callable_kindof_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+ \\$[a-z0-9_]+$",
    "*",
    "Finds the type of a variable\n# Function Params\n\n```\n$val \n```\n\n## Format:\n\n\n### typeof String\n\n```\n$val: malloc string \"Hello World\"\n$typeof: typeof $val\n```\n\n",
];
#[allow(unused)]
fn kindof(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) {
    #[allow(unused_variables)]
    let [_, val] = &(unsafe { &*args })[..] else {
        interpreter::error("Invalid Format!", file);
    };
    let val = unsafe { &**val };
    let val = {
        let heap: &mut interpreter::types::HeapWrapper = unsafe {
            &mut *(&mut heap as *mut _)
        };
        let Some(val) = heap.get(val) else {
            interpreter::error("Varible not found!", file);
        };
        val as &interpreter::types::BufValue
    };
    let _option_code_result = _call_kindof(val, file, opt);
    opt.set_return_val(_option_code_result)
}
#[allow(unused)]
fn _call_kindof(
    val: &BufValue,
    file: &str,
    opt: &mut interpreter::types::Options,
) -> BufValue {
    BufValue::Str(val.type_of())
}
#[allow(non_upper_case_globals)]
static _inner_callable_comp_doc: &'static [&'static str; 3] = &[
    "^(\\$?)[a-z0-9_:]+ \\$[a-zA-Z0-9_]* (==|!=|<=|<|>=|>) \\$[a-zA-Z0-9_]*$",
    "*",
    "Compare\n# Function Params\n\n```\n$a <f> $b \n```\n\n\n\n\n## Notes:\nImportant examples\n- comp $1 == $2\n- comp $1 != $2\n- comp $1 < $2 (only if $1 $2 = number)\n- comp $1 <= $2 (only if $1 $2 = number)        \n- comp $1 > $2 (only if $1 $2 = number)\n- comp $1 >= $2 (only if $1 $2 = number)",
];
#[allow(unused)]
fn comp(
    args: *const [&'static str],
    mut heap: interpreter::types::HeapWrapper,
    file: &str,
    opt: &mut interpreter::types::Options,
) {
    #[allow(unused_variables)]
    let [_, a, f, b] = &(unsafe { &*args })[..] else {
        interpreter::error("Invalid Format!", file);
    };
    let a = unsafe { &**a };
    let f = unsafe { &**f };
    let b = unsafe { &**b };
    let a = {
        let heap: &mut interpreter::types::HeapWrapper = unsafe {
            &mut *(&mut heap as *mut _)
        };
        let Some(a) = heap.get(a) else {
            interpreter::error("Varible not found!", file);
        };
        a as &interpreter::types::BufValue
    };
    let b = {
        let heap: &mut interpreter::types::HeapWrapper = unsafe {
            &mut *(&mut heap as *mut _)
        };
        let Some(b) = heap.get(b) else {
            interpreter::error("Varible not found!", file);
        };
        b as &interpreter::types::BufValue
    };
    let _option_code_result = _call_comp(a, f, b, file, opt);
    opt.set_return_val(_option_code_result)
}
#[allow(unused)]
fn _call_comp(
    a: &BufValue,
    f: &str,
    b: &BufValue,
    file: &str,
    opt: &mut interpreter::types::Options,
) -> BufValue {
    BufValue::Bool(
        match f {
            "==" => a.eq(b),
            "!=" => !a.eq(b),
            "<" => a.lt(&b),
            "<=" => a.lt(&b) || a.eq(b),
            ">" => a.gt(&b),
            ">=" => a.gt(&b) || a.eq(b),
            e => {
                error(
                    &::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("Invalid operator {0} in :comp", e),
                        );
                        res
                    }),
                    file,
                )
            }
        },
    )
}
#[no_mangle]
pub fn ver() -> u16 {
    interpreter::VERSION_INT
}
static MODULES: &[&dyn interpreter::Package] = &[&Core, &Array, &Types];
static RUNTIMES: interpreter::phf::Map<
    &'static str,
    &'static dyn interpreter::runtime::RuntimeValue,
> = phf::Map {
    key: 12913932095322966823u64,
    disps: &[],
    entries: &[],
};
#[no_mangle]
pub fn modules() -> &'static [&'static dyn interpreter::Package] {
    MODULES
}
#[no_mangle]
pub fn runtimes() -> interpreter::phf::map::Entries<
    'static,
    &'static str,
    &'static dyn interpreter::runtime::RuntimeValue,
> {
    RUNTIMES.entries()
}
#[no_mangle]
pub fn runtime(id: &str) -> Option<&'static dyn interpreter::runtime::RuntimeValue> {
    let Some(rt) = RUNTIMES.get(id) else {
        return None;
    };
    Some(*rt)
}
