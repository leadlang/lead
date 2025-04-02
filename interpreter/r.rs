#![feature(prelude_import)]
#![feature(fn_traits)]
#![feature(trait_alias)]
#![feature(concat_idents)]
#![feature(macro_metavar_expr)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use std::{
    collections::HashMap, process, time::{Duration, Instant},
    sync::LazyLock,
};
pub use paste::paste;
#[macro_use]
pub mod macros {}
pub mod runtime {
    use std::collections::HashMap;
    use crate::types::{Args, Heap, HeapWrapper, Options};
    pub type PackageCallback = fn(
        Args,
        &mut Heap,
        HeapWrapper,
        &String,
        &mut Options,
    ) -> ();
    pub type RuntimeMethodRes = HashMap<&'static str, (&'static str, PackageCallback)>;
    pub mod _root_syntax {
        use tokio::task::spawn_blocking;
        use crate::{
            error, ipreter::{interpret, tok_parse},
            types::{
                make_unsafe_send_future, set_runtime_val, BufValue, Heap, Options,
                RawRTValue,
            },
            Application, RespPackage,
        };
        use std::{borrow::Cow, collections::HashMap, mem::{take, transmute}};
        pub(crate) struct RTCreatedModule {
            pub(crate) code: String,
            pub(crate) lines: Vec<&'static str>,
            pub(crate) name: &'static str,
            pub(crate) heap: Heap,
            pub(crate) methods: HashMap<
                &'static str,
                (Vec<&'static str>, &'static [&'static str]),
            >,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for RTCreatedModule {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field5_finish(
                    f,
                    "RTCreatedModule",
                    "code",
                    &self.code,
                    "lines",
                    &self.lines,
                    "name",
                    &self.name,
                    "heap",
                    &self.heap,
                    "methods",
                    &&self.methods,
                )
            }
        }
        impl RTCreatedModule {
            pub(crate) fn run_method<T: FnOnce(&mut Heap, &mut Heap, &Vec<&str>) -> ()>(
                &mut self,
                app: *mut Application,
                method: &str,
                file: &str,
                into_heap: T,
                heap: &mut Heap,
                opt: &mut Options,
            ) {
                let mut temp_heap = Heap::new_with_this(&mut self.heap);
                let app = unsafe { &mut *app };
                let (args, method_code) = self
                    .methods
                    .get(&method)
                    .unwrap_or_else(|| error("Unable to find :method", file));
                into_heap(&mut temp_heap, heap, args);
                let file_name = ":fn";
                let file = method_code;
                let mut line = 0usize;
                let mut markers = HashMap::new();
                while line < file.len() {
                    let content = file[line];
                    if !content.starts_with("#") {
                        unsafe {
                            tok_parse(
                                ::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}:{1}", &file_name, line),
                                    );
                                    res
                                }),
                                content,
                                app,
                                &mut temp_heap,
                                &mut line,
                                &mut markers,
                                false,
                                Some(opt),
                            );
                        }
                    }
                    line += 1;
                }
                drop(temp_heap);
            }
        }
        #[allow(unused)]
        pub fn insert_into_application(
            app: *mut Application,
            args: &[*const str],
            line: &mut usize,
            to_set: Cow<'static, str>,
            heap: &mut Heap,
            markers: &mut HashMap<Cow<'static, str>, usize>,
        ) {
            let app = unsafe { &mut *app };
            if args.len() == 3 {
                let [a, v, v2] = &args[..] else {
                    {
                        ::core::panicking::panic_fmt(format_args!("Invalid syntax"));
                    };
                };
                unsafe {
                    let a = &**a;
                    match a {
                        "*listen" => {
                            let function = &**v2;
                            let module = &**v;
                            let module = heap
                                .remove(module)
                                .expect("Invalid Format")
                                .expect("Unable to capture Runtime");
                            let BufValue::RuntimeRaw(name, module) = module else {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!("Expected, Lead Module"),
                                    );
                                };
                            };
                            let RawRTValue::RTCM(mut module) = module.0 else {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!("Expected, Lead Module, not {0}", name),
                                    );
                                };
                            };
                            let BufValue::Listener(mut listen) = heap
                                .remove(function)
                                .expect("Unable to capture heaplistener")
                                .expect("Unable to capture heaplistener") else {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!("Invalid! Not HeapListener"),
                                    );
                                }
                            };
                            let app_ptr: &'static mut Application = unsafe {
                                transmute(&mut *app)
                            };
                            let future = async move {
                                let mut dummy_heap = Heap::new();
                                let app = app_ptr as *mut _;
                                let mut opt = Options::new();
                                while let Some(event) = listen.recv().await {
                                    let app = unsafe {
                                        transmute::<
                                            &mut Application,
                                            &'static mut Application<'static>,
                                        >(&mut *app)
                                    };
                                    let opt: &'static mut Options = unsafe {
                                        transmute(&mut opt)
                                    };
                                    let dummy_heap: &'static mut Heap = unsafe {
                                        transmute(&mut dummy_heap)
                                    };
                                    let module: &'static mut RTCreatedModule = unsafe {
                                        transmute(&mut module)
                                    };
                                    spawn_blocking(move || {
                                            module
                                                .run_method(
                                                    app as _,
                                                    "on",
                                                    "",
                                                    move |fn_heap, _, c| {
                                                        if c.len() == 1 {
                                                            let arg0: &'static str = unsafe { transmute(&*c[0]) };
                                                            fn_heap.set(Cow::Borrowed(&arg0[2..]), event);
                                                        } else {
                                                            {
                                                                ::core::panicking::panic_fmt(
                                                                    format_args!("Expected, exactly 1 argument"),
                                                                );
                                                            };
                                                        }
                                                    },
                                                    dummy_heap,
                                                    opt,
                                                );
                                        })
                                        .await;
                                }
                            };
                            let future = make_unsafe_send_future(future);
                            app.runtime.spawn(future);
                        }
                        _ => {
                            ::core::panicking::panic_fmt(format_args!("Invalid syntax"));
                        }
                    }
                }
                return;
            }
            let [a, v] = &args[..] else {
                {
                    ::core::panicking::panic_fmt(format_args!("Invalid syntax"));
                };
            };
            unsafe {
                let v = &&**v;
                match &**a {
                    "*run" => {
                        interpret(&v, app);
                    }
                    "*mark" => {
                        markers.insert(Cow::Borrowed(*v as &str), *line);
                    }
                    "*goto" => {
                        *line = *markers.get(*v as &str).expect("No marker was found!");
                    }
                    "*import" => {
                        let packages = app.pkg_resolver.call_mut((v,));
                        let mut pkg = HashMap::new();
                        for package in packages {
                            let RespPackage { methods } = package;
                            for (sig, call) in methods {
                                pkg.insert(sig.to_string(), *call);
                            }
                        }
                        let val = RawRTValue::PKG(pkg);
                        set_runtime_val(heap, to_set, "loaded", val);
                    }
                    "*mod" => {
                        let code = String::from_utf8(
                                app
                                    .module_resolver
                                    .call_mut((
                                        ::alloc::__export::must_use({
                                                let res = ::alloc::fmt::format(
                                                    format_args!("./{0}.mod.pb", v),
                                                );
                                                res
                                            })
                                            .as_str(),
                                    )),
                            )
                            .unwrap_or_else(|_| {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!("Unable to read {0}.mod.pb", v),
                                    );
                                };
                            });
                        let Some(m) = parse_into_modules(code) else {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("No RTC Module found in the module file"),
                                );
                            };
                        };
                        set_runtime_val(heap, to_set, m.name, RawRTValue::RTCM(m));
                    }
                    a => {
                        ::core::panicking::panic_fmt(format_args!("Unknown {0}", a));
                    }
                };
            }
        }
        pub(crate) fn parse_into_modules(code: String) -> Option<RTCreatedModule> {
            let mut data = RTCreatedModule {
                code,
                lines: ::alloc::vec::Vec::new(),
                heap: Heap::new(),
                methods: HashMap::new(),
                name: "%none",
            };
            let split = data.code.split("\n");
            let split = split
                .map(|x| unsafe { transmute::<&str, &'static str>(x.trim()) })
                .filter(|x| x != &"" && !x.starts_with("#"))
                .collect::<Vec<_>>();
            data.lines = split;
            let mut mod_id: u8 = 0;
            let mut ctx = "";
            let mut tok_arg: Vec<&str> = ::alloc::vec::Vec::new();
            let mut start: usize = 0;
            let mut in_ctx = false;
            for (id, tokens) in data.lines.iter().enumerate() {
                let mut tok = tokens.split(" ").collect::<Vec<_>>();
                if !in_ctx {
                    let caller = tok.remove(0);
                    match caller {
                        "declare" => {
                            if mod_id != 0 {
                                {
                                    ::core::panicking::panic_fmt(
                                        format_args!(
                                            "More than 1 module found in a single lead module file",
                                        ),
                                    );
                                };
                            }
                            mod_id += 1;
                            data.name = tok.remove(0);
                        }
                        "fn" => {
                            ctx = tok.remove(0);
                            in_ctx = true;
                            start = id + 1;
                            for t in &tok {
                                if (!t.starts_with("->")) || (t.starts_with("->&")) {
                                    error(
                                        ::alloc::__export::must_use({
                                            let res = ::alloc::fmt::format(
                                                format_args!(
                                                    "Arguments of module parameters can ONLY be move! {0} is not move!",
                                                    t,
                                                ),
                                            );
                                            res
                                        }),
                                        ":core:parser",
                                    );
                                }
                            }
                            tok_arg = take(&mut tok);
                        }
                        a => {
                            ::core::panicking::panic_fmt(
                                format_args!("Unknown NON-CONTEXT {0}", a),
                            );
                        }
                    };
                } else {
                    if tok[0] == "*end" {
                        in_ctx = false;
                        if start == usize::MAX {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("Something is wrong!"),
                                );
                            };
                        }
                        let lines: &'static [&'static str] = unsafe {
                            transmute(&data.lines[..] as &[&'static str])
                        };
                        let begin = start as usize;
                        let None = data
                            .methods
                            .insert(
                                ctx,
                                (std::mem::take(&mut tok_arg), &lines[begin..id]),
                            ) else {
                            {
                                ::core::panicking::panic_fmt(
                                    format_args!("Method overlap"),
                                );
                            };
                        };
                        start = usize::MAX;
                    }
                }
            }
            if mod_id == 0 { None } else { Some(data) }
        }
    }
    pub trait RuntimeValue: Sync {
        fn doc(&self) -> HashMap<&'static str, &'static [&'static str; 3]>;
        fn name(&self) -> &'static str;
        fn call_ptr(
            &mut self,
            caller: &str,
            v: *const [*const str],
            a: HeapWrapper,
            c: &String,
            o: &mut Options,
        ) -> Option<()>;
    }
}
pub use runtime::RuntimeValue;
#[cfg(feature = "phf")]
pub use phf;
mod ipreter {
    use std::{borrow::Cow, collections::HashMap, future::Future, pin::Pin};
    use crate::{
        error, runtime::_root_syntax::insert_into_application,
        types::{
            call_runtime_val, mkbuf, set_runtime_val, BufValue, Heap, HeapWrapper,
            Options, Output, RawRTValue,
        },
        Application,
    };
    pub fn interpret<'a>(file: &str, mut app: &mut Application<'a>) {
        let file_name = if file == ":entry" { app.entry } else { file };
        let app_ptr = app as *mut Application;
        let file = app
            .code
            .get(file)
            .unwrap_or_else(move || {
                let app = unsafe { &mut *app_ptr };
                let data = app
                    .module_resolver
                    .call_mut((
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("./{0}.pb", file),
                            );
                            res
                        }),
                    ));
                app.code.insert(file.into(), String::from_utf8(data).unwrap());
                app.code.get(file).expect("Impossible")
            });
        let file = file.replace("\r", "");
        let file = file.split("\n").collect::<Vec<_>>();
        let mut line = 0usize;
        let app2: *mut Application<'static> = unsafe {
            std::mem::transmute(app as *mut Application)
        };
        let app2: &'static mut Application<'static> = unsafe { &mut *app2 };
        let mut markers = HashMap::new();
        while line < file.len() {
            let content = &file[line];
            if !content.starts_with("#") {
                unsafe {
                    let f = tok_parse(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("{0}:{1}", &file_name, line + 1),
                            );
                            res
                        }),
                        content,
                        &mut app,
                        &mut app2.heap,
                        &mut line,
                        &mut markers,
                        false,
                        None,
                    );
                    drop(f);
                }
            }
            line += 1;
        }
    }
    pub(crate) unsafe fn tok_parse<'a>(
        file: String,
        piece: &str,
        app: &mut Application<'a>,
        heap: *mut Heap,
        line: &mut usize,
        markers: &mut HashMap<Cow<'static, str>, usize>,
        r#async: bool,
        orig_opt: Option<&mut Options>,
    ) -> Option<Pin<Box<dyn Future<Output = ()> + 'a>>> {
        let heap: &'static mut Heap = unsafe { &mut *heap };
        let tokens: Vec<*const str> = piece
            .split(" ")
            .map(|x| x as *const str)
            .collect();
        let mut caller = unsafe { &*tokens[0] };
        let mut val_type = false;
        let mut to_set = "";
        let mut start = 0;
        if caller.ends_with(":") && caller.starts_with("$") {
            val_type = true;
            let l = unsafe { &*tokens[0] };
            let set = l.split_at(l.len() - 1).0;
            to_set = set;
            start = 1;
            caller = unsafe { &*tokens[1] };
        }
        let mut opt = Options::new();
        if caller.starts_with("*if$") {
            let caller = unsafe { &*tokens[start] };
            let caller = caller.replacen("*if", "", 1);
            let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get the value")
            else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Invalid type, expected boolean in *if"),
                    );
                };
            };
            if *x {
                return tok_parse(
                    file,
                    &piece[unsafe { &*tokens[start] }.len() + 1..],
                    app,
                    heap,
                    line,
                    markers,
                    r#async,
                    orig_opt,
                );
            }
            None
        } else if caller.starts_with("*else$") {
            let caller = unsafe { &*tokens[start] };
            let caller = caller.replacen("*else", "", 1);
            let BufValue::Bool(x) = heap.get(&caller).expect("Unable to get the value")
            else {
                {
                    ::core::panicking::panic_fmt(
                        format_args!("Invalid type, expected boolean in *if"),
                    );
                };
            };
            if !*x {
                return tok_parse(
                    file,
                    &piece[unsafe { &*tokens[start] }.len() + 1..],
                    app,
                    heap,
                    line,
                    markers,
                    r#async,
                    orig_opt,
                );
            }
            None
        } else if caller == "*return" {
            let Some(opt) = orig_opt else {
                error("*return can only be called from a lead module", file);
            };
            let var = unsafe { &*tokens[start + 1] };
            opt.r_val = Some(
                heap
                    .remove(var)
                    .expect("Cannot find variable")
                    .expect("Cannot find variable")
                    .into(),
            );
            None
        } else if caller.starts_with("*") {
            insert_into_application(
                app as *mut _ as _,
                &tokens[start..],
                line,
                Cow::Borrowed(to_set),
                heap,
                markers,
            );
            None
        } else if caller.starts_with("@") {
            if val_type {
                let _ = heap.set(Cow::Borrowed(to_set), mkbuf(&caller, &file));
            }
            None
        } else if caller.starts_with("$") {
            let app_ptr = app as *mut _;
            let app_heap_ptr = heap as *mut _;
            let tokens_ptr = &tokens as &[*const str];
            let caller_ptr = caller as *const _;
            let wrap = HeapWrapper {
                heap: unsafe { &mut *app_heap_ptr },
                args: unsafe { &*(tokens_ptr as *const _) },
                pkg_name: unsafe { &*caller_ptr },
                app: app_ptr,
                allow_full: false,
            };
            match call_runtime_val(
                app as _,
                heap,
                caller,
                &tokens[start..],
                wrap,
                &file,
                &mut opt,
                &file,
                r#async,
            ) {
                None => {
                    if &caller != &"" {
                        error(
                            &::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Unexpected `{0}`", &caller),
                                );
                                res
                            }),
                            &file,
                        );
                    }
                    None
                }
                Some(Output::String(v)) => {
                    let runt = opt.rem_r_runtime();
                    if val_type && opt.r_val.is_some() {
                        let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
                    } else if val_type && runt.is_some() {
                        let _ = set_runtime_val(
                            heap,
                            Cow::Borrowed(to_set),
                            v,
                            RawRTValue::RT(runt.unwrap()),
                        );
                    }
                    None
                }
                Some(Output::Future(v)) => {
                    return Some(
                        Box::pin(async move {
                            let v = v.await;
                            let runt = opt.rem_r_runtime();
                            if val_type && opt.r_val.is_some() {
                                let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
                            } else if val_type && runt.is_some() {
                                let _ = set_runtime_val(
                                    heap,
                                    Cow::Borrowed(to_set),
                                    v,
                                    RawRTValue::RT(runt.unwrap()),
                                );
                            }
                        }),
                    );
                }
            }
        } else {
            let app_ptr = app as *mut _;
            let app_heap_ptr = heap as *mut _;
            let tokens_ptr = &tokens as &[_];
            match app.pkg.inner.get_mut(caller) {
                Some((p, v)) => {
                    let pkg: *const str = *p as *const _;
                    let pkg = unsafe { &*pkg };
                    let wrap = HeapWrapper {
                        heap: unsafe { &mut *app_heap_ptr },
                        args: unsafe { &*(tokens_ptr as *const _) },
                        pkg_name: pkg,
                        app: app_ptr,
                        allow_full: true,
                    };
                    v(&tokens[start..] as *const _, wrap, &file, &mut opt);
                    let runt = opt.rem_r_runtime();
                    if val_type && opt.r_val.is_some() {
                        let _ = heap.set(Cow::Borrowed(to_set), opt.r_val());
                    } else if val_type && runt.is_some() {
                        let _ = set_runtime_val(
                            heap,
                            Cow::Borrowed(to_set),
                            pkg,
                            RawRTValue::RT(runt.unwrap()),
                        );
                    }
                    None
                }
                _ => {
                    if &caller != &"" {
                        error(
                            &::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("Unexpected `{0}`", &caller),
                                );
                                res
                            }),
                            &file,
                        );
                    }
                    None
                }
            }
        }
    }
}
#[macro_use]
pub mod package {
    use crate::{types::MethodRes, Package};
    /// ImplPackage is meant to create a package out of Box<dyn Package>
    pub(crate) struct ImplPackage {
        pub(crate) name: &'static [u8],
        pub(crate) methods: MethodRes,
    }
    #[automatically_derived]
    impl ::core::default::Default for ImplPackage {
        #[inline]
        fn default() -> ImplPackage {
            ImplPackage {
                name: ::core::default::Default::default(),
                methods: ::core::default::Default::default(),
            }
        }
    }
    impl Package for ImplPackage {
        fn name(&self) -> &'static [u8] {
            self.name
        }
        fn methods(&self) -> MethodRes {
            self.methods
        }
    }
}
pub mod types {
    mod alloc {
        use crate::error;
        use super::BufValue;
        pub fn mkbuf(data: &str, file: &str) -> BufValue {
            let (_, val) = data.split_at(1);
            let (header, value) = val.split_at(1);
            match header {
                "'" => {
                    if value.contains(".") {
                        BufValue::Float(value.parse().unwrap())
                    } else if let Ok(v) = value.parse() {
                        BufValue::U_Int(v)
                    } else {
                        BufValue::Int(value.parse().unwrap())
                    }
                }
                "f" => BufValue::Float(value.parse().unwrap()),
                "u" => BufValue::U_Int(value.parse().unwrap()),
                "i" => BufValue::Int(value.parse().unwrap()),
                "s" => {
                    if !val.ends_with("\"") {
                        error("Unknown String closing, expected `\"`", file);
                    }
                    if !val.starts_with("\"") {
                        error("Unknown String starting, expected `\"`", file);
                    }
                    BufValue::Str(val[2..].into())
                }
                "1" => BufValue::Bool(true),
                "0" => BufValue::Bool(false),
                e => {
                    error(
                        ::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!("Unknown header `{0}` used", e),
                            );
                            res
                        }),
                        file,
                    );
                }
            }
        }
    }
    mod fns {
        use crate::Package;
        use lealang_chalk_rs::Chalk;
        use std::collections::HashMap;
        use super::{set_into_extends, ExtendsInternal, HeapWrapper, Options};
        pub type Args = *const [*const str];
        pub type PackageCallback = fn(Args, HeapWrapper, &String, &mut Options) -> ();
        pub type MethodRes = &'static [(&'static str, PackageCallback)];
        pub struct LanguagePackages<'a> {
            pub inner: HashMap<&'static str, (&'a str, PackageCallback)>,
            pub(crate) extends: ExtendsInternal,
        }
        impl<'a> LanguagePackages<'a> {
            pub fn new() -> Self {
                Self {
                    inner: HashMap::new(),
                    extends: ExtendsInternal::default(),
                }
            }
            pub fn import_dyn(&mut self, func: Box<dyn Package>) -> &mut Self {
                let name = String::from_utf8_lossy(func.name());
                let name: &'static mut str = name.to_string().leak::<'static>();
                for (key, val) in func.methods() {
                    self.inner.insert(key, (name, *val));
                }
                set_into_extends(func.prototype(), &mut self.extends);
                self
            }
            pub fn import_static(&mut self, func: &'static dyn Package) -> &mut Self {
                let name = String::from_utf8_lossy(func.name());
                let name: &'static mut str = name.to_string().leak::<'static>();
                for (key, val) in func.methods() {
                    self.inner.insert(key, (name, *val));
                }
                set_into_extends(func.prototype(), &mut self.extends);
                self
            }
            pub fn import<T: Package + 'static>(&mut self, func: T) -> &mut Self {
                self.import_dyn(Box::new(func))
            }
            pub fn list(&self, chalk: &mut Chalk) {
                {
                    ::std::io::_print(
                        format_args!(
                            "{0} {1}\n",
                            chalk.reset_weight().blue().string(&"Total Commands:"),
                            self.inner.len(),
                        ),
                    );
                };
                chalk.reset_weight().green().println(&"Commands:");
                self.inner
                    .iter()
                    .enumerate()
                    .for_each(|(no, (syntax, (name, _)))| {
                        chalk
                            .red()
                            .print(
                                &::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("{0}- ", no + 1),
                                    );
                                    res
                                }),
                            );
                        chalk.yellow().bold().print(&syntax);
                        {
                            ::std::io::_print(format_args!(" from "));
                        };
                        chalk.reset_weight().blue().println(&name);
                    });
            }
        }
    }
    mod heap {
        use std::{
            borrow::Cow, fmt::Debug, collections::HashMap, future::Future, pin::Pin,
        };
        use crate::{
            error, runtime::{RuntimeValue, _root_syntax::RTCreatedModule},
            Application,
        };
        use super::{
            handle_runtime, AppliesEq, BufValue, HeapWrapper, Options, PackageCallback,
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
                f.write_fmt(format_args!("RawRTValue {{ 0x... }}"))
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
            let _ = get_ptr(heap)
                .insert(key, BufValue::RuntimeRaw(module, AppliesEq(val)));
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
                val => return handle_runtime(app, val, caller, v, a, c, o),
            };
            let data = (ai, bi);
            match &mut data.1.0 {
                RawRTValue::RT(data) => data.call_ptr(caller, v as _, a, c, o)?,
                RawRTValue::PKG(pkg) => {
                    match pkg.get_mut(caller) {
                        Some(x) => x.call_mut((v as *const [*const str], a, c, o)),
                        None => {
                            error(
                                &::alloc::__export::must_use({
                                    let res = ::alloc::fmt::format(
                                        format_args!("Unexpected `{0}`", &caller),
                                    );
                                    res
                                }),
                                &file,
                            )
                        }
                    }
                }
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
                                tkns.into_iter()
                                    .zip(args.iter())
                                    .for_each(|(token, arg)| {
                                        let token = unsafe { &**token };
                                        let from = app_heap
                                            .remove(token)
                                            .unwrap_or_else(|| {
                                                error(
                                                    ::alloc::__export::must_use({
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("Unable to get {0} from Heap", token),
                                                        );
                                                        res
                                                    }),
                                                    ":interpreter:loadmodule",
                                                )
                                            })
                                            .unwrap_or_else(|| {
                                                error(
                                                    ::alloc::__export::must_use({
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("Unable to get {0} from Heap", token),
                                                        );
                                                        res
                                                    }),
                                                    ":interpreter:loadmodule",
                                                )
                                            });
                                        fn_heap
                                            .set(
                                                Cow::Borrowed(unsafe { &*(&arg[2..] as *const str) }),
                                                from,
                                            )
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
        pub struct Heap {
            data: HeapInnerMap,
            this: Option<*mut Self>,
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Heap {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field2_finish(
                    f,
                    "Heap",
                    "data",
                    &self.data,
                    "this",
                    &&self.this,
                )
            }
        }
        unsafe impl Send for Heap {}
        unsafe impl Sync for Heap {}
        impl Heap {
            pub fn new() -> Self {
                Self {
                    data: HashMap::new(),
                    this: None,
                }
            }
            pub fn new_with_this(this: *mut Self) -> Self {
                Self {
                    data: HashMap::new(),
                    this: Some(this),
                }
            }
            pub fn clear(&mut self) {
                *self = Self::new();
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
    }
    mod heap_wrap {
        use std::ptr;
        use crate::Application;
        use super::{BufValue, Heap};
        pub struct HeapWrapper<'a> {
            pub(crate) heap: &'a mut Heap,
            pub(crate) args: &'a [*const str],
            pub(crate) pkg_name: &'a str,
            pub(crate) app: *mut Application<'a>,
            pub(crate) allow_full: bool,
        }
        impl<'a> HeapWrapper<'a> {
            pub fn upgrade(self) -> &'a mut Heap {
                if self.allow_full {
                    return self.heap;
                }
                let app = unsafe { &mut *self.app };
                app.log_info.call_mut((self.pkg_name,));
                self.heap
            }
            pub fn get(&self, key: &str) -> Option<&BufValue> {
                if self.args.iter().any(|&x| unsafe { &*x } == key) {
                    return self.heap.get(key);
                }
                None
            }
            pub fn get_mut(&mut self, key: &str) -> Option<&mut BufValue> {
                if self.args.iter().any(|&x| ptr::eq(x, key)) {
                    return self.heap.get_mut(key);
                }
                None
            }
            pub fn remove(&mut self, key: &str) -> Option<Option<BufValue>> {
                if self.args.iter().any(|&x| ptr::eq(x, key)) {
                    return self.heap.remove(key);
                }
                None
            }
        }
    }
    use std::{
        any::Any, collections::HashMap, fmt::Debug, future::Future,
        ops::{Deref, DerefMut},
        pin::Pin, task::{Context, Poll},
        thread::JoinHandle,
    };
    pub use alloc::*;
    pub use fns::*;
    pub use heap::*;
    pub use heap_wrap::*;
    use tokio::sync::mpsc::UnboundedReceiver;
    use crate::{runtime::RuntimeValue, Application};
    pub struct RetBufValue(pub BufValue);
    impl From<BufValue> for RetBufValue {
        fn from(item: BufValue) -> Self {
            Self(item)
        }
    }
    pub struct Options {
        pub pre: *const str,
        pub r_val: Option<RetBufValue>,
        r_runtime: Option<Box<dyn RuntimeValue>>,
    }
    unsafe impl Send for Options {}
    unsafe impl Sync for Options {}
    impl Debug for Options {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_fmt(format_args!("Options {{ <inner> }}"))
        }
    }
    impl Options {
        pub fn new() -> Self {
            Self {
                pre: "" as _,
                r_val: None,
                r_runtime: None,
            }
        }
        pub fn set_return_val(&mut self, val: BufValue) {
            self.r_val = Some(RetBufValue(val));
        }
        pub(crate) fn r_val(self) -> BufValue {
            let val = self.r_val;
            val.expect("Error").0
        }
        pub(crate) fn rem_r_runtime(&mut self) -> Option<Box<dyn RuntimeValue>> {
            self.r_runtime.take()
        }
        pub fn set_r_runtime(&mut self, val: Box<dyn RuntimeValue>) {
            self.r_runtime = Some(val);
        }
    }
    pub struct AnyWrapper(pub Box<dyn Any>);
    #[automatically_derived]
    impl ::core::fmt::Debug for AnyWrapper {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AnyWrapper", &&self.0)
        }
    }
    impl Deref for AnyWrapper {
        type Target = dyn Any;
        fn deref(&self) -> &Self::Target {
            self.0.deref()
        }
    }
    impl PartialEq for AnyWrapper {
        fn eq(&self, _other: &Self) -> bool {
            false
        }
    }
    pub struct StrPointer(pub *const str);
    #[automatically_derived]
    impl ::core::marker::StructuralPartialEq for StrPointer {}
    #[automatically_derived]
    impl ::core::cmp::PartialEq for StrPointer {
        #[inline]
        fn eq(&self, other: &StrPointer) -> bool {
            self.0 == other.0
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for StrPointer {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "StrPointer", &&self.0)
        }
    }
    impl ToString for StrPointer {
        fn to_string(&self) -> String {
            unsafe { (*self.0).to_string() }
        }
    }
    use phf::Map as PhfMap;
    pub(crate) struct ExtendsInternal {
        pub(crate) int: HashMap<
            &'static str,
            fn(*mut i64, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) uint: HashMap<
            &'static str,
            fn(*mut u64, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) float: HashMap<
            &'static str,
            fn(*mut f64, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) str_slice: HashMap<
            &'static str,
            fn(*mut String, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) boolean: HashMap<
            &'static str,
            fn(*mut bool, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) array: HashMap<
            &'static str,
            fn(*mut Vec<BufValue>, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) object: HashMap<
            &'static str,
            fn(
                *mut HashMap<String, Box<BufValue>>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
        pub(crate) faillable: HashMap<
            &'static str,
            fn(
                *mut Result<Box<BufValue>, String>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
        pub(crate) str_ptr: HashMap<
            &'static str,
            fn(*mut StrPointer, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) ptr: HashMap<
            &'static str,
            fn(*mut *const BufValue, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) mut_ptr: HashMap<
            &'static str,
            fn(*mut *mut BufValue, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) rt_any: HashMap<
            &'static str,
            fn(*mut AnyWrapper, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub(crate) async_task: HashMap<
            &'static str,
            fn(
                *mut AppliesEq<JoinHandle<BufValue>>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
        pub(crate) listener: HashMap<
            &'static str,
            fn(
                *mut AppliesEq<UnboundedReceiver<BufValue>>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
    }
    #[automatically_derived]
    impl ::core::default::Default for ExtendsInternal {
        #[inline]
        fn default() -> ExtendsInternal {
            ExtendsInternal {
                int: ::core::default::Default::default(),
                uint: ::core::default::Default::default(),
                float: ::core::default::Default::default(),
                str_slice: ::core::default::Default::default(),
                boolean: ::core::default::Default::default(),
                array: ::core::default::Default::default(),
                object: ::core::default::Default::default(),
                faillable: ::core::default::Default::default(),
                str_ptr: ::core::default::Default::default(),
                ptr: ::core::default::Default::default(),
                mut_ptr: ::core::default::Default::default(),
                rt_any: ::core::default::Default::default(),
                async_task: ::core::default::Default::default(),
                listener: ::core::default::Default::default(),
            }
        }
    }
    pub struct Extends {
        pub int: PhfMap<
            &'static str,
            fn(*mut i64, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub uint: PhfMap<
            &'static str,
            fn(*mut u64, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub float: PhfMap<
            &'static str,
            fn(*mut f64, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub str_slice: PhfMap<
            &'static str,
            fn(*mut String, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub boolean: PhfMap<
            &'static str,
            fn(*mut bool, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub array: PhfMap<
            &'static str,
            fn(*mut Vec<BufValue>, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub object: PhfMap<
            &'static str,
            fn(
                *mut HashMap<String, Box<BufValue>>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
        pub faillable: PhfMap<
            &'static str,
            fn(
                *mut Result<Box<BufValue>, String>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
        pub str_ptr: PhfMap<
            &'static str,
            fn(*mut StrPointer, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub ptr: PhfMap<
            &'static str,
            fn(*mut *const BufValue, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub mut_ptr: PhfMap<
            &'static str,
            fn(*mut *mut BufValue, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub rt_any: PhfMap<
            &'static str,
            fn(*mut AnyWrapper, Args, HeapWrapper, &String, &mut Options) -> (),
        >,
        pub async_task: PhfMap<
            &'static str,
            fn(
                *mut AppliesEq<JoinHandle<BufValue>>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
        pub listener: PhfMap<
            &'static str,
            fn(
                *mut AppliesEq<UnboundedReceiver<BufValue>>,
                Args,
                HeapWrapper,
                &String,
                &mut Options,
            ) -> (),
        >,
    }
    #[automatically_derived]
    impl ::core::default::Default for Extends {
        #[inline]
        fn default() -> Extends {
            Extends {
                int: ::core::default::Default::default(),
                uint: ::core::default::Default::default(),
                float: ::core::default::Default::default(),
                str_slice: ::core::default::Default::default(),
                boolean: ::core::default::Default::default(),
                array: ::core::default::Default::default(),
                object: ::core::default::Default::default(),
                faillable: ::core::default::Default::default(),
                str_ptr: ::core::default::Default::default(),
                ptr: ::core::default::Default::default(),
                mut_ptr: ::core::default::Default::default(),
                rt_any: ::core::default::Default::default(),
                async_task: ::core::default::Default::default(),
                listener: ::core::default::Default::default(),
            }
        }
    }
    pub struct PrototypeDocs {
        pub int: HashMap<&'static str, &'static [&'static str; 3]>,
        pub uint: HashMap<&'static str, &'static [&'static str; 3]>,
        pub float: HashMap<&'static str, &'static [&'static str; 3]>,
        pub str_slice: HashMap<&'static str, &'static [&'static str; 3]>,
        pub boolean: HashMap<&'static str, &'static [&'static str; 3]>,
        pub array: HashMap<&'static str, &'static [&'static str; 3]>,
        pub object: HashMap<&'static str, &'static [&'static str; 3]>,
        pub faillable: HashMap<&'static str, &'static [&'static str; 3]>,
        pub str_ptr: HashMap<&'static str, &'static [&'static str; 3]>,
        pub ptr: HashMap<&'static str, &'static [&'static str; 3]>,
        pub mut_ptr: HashMap<&'static str, &'static [&'static str; 3]>,
        pub rt_any: HashMap<&'static str, &'static [&'static str; 3]>,
        pub async_task: HashMap<&'static str, &'static [&'static str; 3]>,
        pub listener: HashMap<&'static str, &'static [&'static str; 3]>,
    }
    #[automatically_derived]
    impl ::core::default::Default for PrototypeDocs {
        #[inline]
        fn default() -> PrototypeDocs {
            PrototypeDocs {
                int: ::core::default::Default::default(),
                uint: ::core::default::Default::default(),
                float: ::core::default::Default::default(),
                str_slice: ::core::default::Default::default(),
                boolean: ::core::default::Default::default(),
                array: ::core::default::Default::default(),
                object: ::core::default::Default::default(),
                faillable: ::core::default::Default::default(),
                str_ptr: ::core::default::Default::default(),
                ptr: ::core::default::Default::default(),
                mut_ptr: ::core::default::Default::default(),
                rt_any: ::core::default::Default::default(),
                async_task: ::core::default::Default::default(),
                listener: ::core::default::Default::default(),
            }
        }
    }
    pub(crate) fn handle_runtime<'a>(
        app: *mut Application,
        val: &mut BufValue,
        caller: &'a str,
        v: &'a [*const str],
        a: HeapWrapper,
        c: &String,
        o: &'a mut Options,
    ) -> Option<Output> {
        let app = unsafe { &mut *app };
        let extends = &app.pkg.extends;
        match val {
            BufValue::Int(data) => {
                let scope = &extends.int;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::U_Int(data) => {
                let scope = &extends.uint;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Float(data) => {
                let scope = &extends.float;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Str(data) => {
                let scope = &extends.str_slice;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Bool(data) => {
                let scope = &extends.boolean;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Array(data) => {
                let scope = &extends.array;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Object(data) => {
                let scope = &extends.object;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Faillable(data) => {
                let scope = &extends.faillable;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::StrPointer(data) => {
                let scope = &extends.str_ptr;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Pointer(data) => {
                let scope = &extends.ptr;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::PointerMut(data) => {
                let scope = &extends.mut_ptr;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Runtime(data) => {
                let scope = &extends.rt_any;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::AsyncTask(data) => {
                let scope = &extends.async_task;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            BufValue::Listener(data) => {
                let scope = &extends.listener;
                let Some(f) = scope.get(caller) else {
                    return None;
                };
                f(data as _, v, a, c, o);
            }
            _ => return None,
        }
        Some(Output::String("Prototype"))
    }
    pub(crate) fn set_into_extends(
        extends: Extends,
        extends_internal: &mut ExtendsInternal,
    ) {
        for (k, v) in extends.int.into_iter() {
            extends_internal.int.insert(k, *v);
        }
        for (k, v) in extends.uint.into_iter() {
            extends_internal.uint.insert(k, *v);
        }
        for (k, v) in extends.float.into_iter() {
            extends_internal.float.insert(k, *v);
        }
        for (k, v) in extends.str_slice.into_iter() {
            extends_internal.str_slice.insert(k, *v);
        }
        for (k, v) in extends.boolean.into_iter() {
            extends_internal.boolean.insert(k, *v);
        }
        for (k, v) in extends.array.into_iter() {
            extends_internal.array.insert(k, *v);
        }
        for (k, v) in extends.object.into_iter() {
            extends_internal.object.insert(k, *v);
        }
        for (k, v) in extends.faillable.into_iter() {
            extends_internal.faillable.insert(k, *v);
        }
        for (k, v) in extends.str_ptr.into_iter() {
            extends_internal.str_ptr.insert(k, *v);
        }
        for (k, v) in extends.ptr.into_iter() {
            extends_internal.ptr.insert(k, *v);
        }
        for (k, v) in extends.mut_ptr.into_iter() {
            extends_internal.mut_ptr.insert(k, *v);
        }
        for (k, v) in extends.rt_any.into_iter() {
            extends_internal.rt_any.insert(k, *v);
        }
        for (k, v) in extends.async_task.into_iter() {
            extends_internal.async_task.insert(k, *v);
        }
        for (k, v) in extends.listener.into_iter() {
            extends_internal.listener.insert(k, *v);
        }
    }
    #[allow(non_camel_case_types)]
    pub enum BufValue {
        Int(i64),
        U_Int(u64),
        Float(f64),
        Str(String),
        Bool(bool),
        Array(Vec<Self>),
        Object(HashMap<String, Box<Self>>),
        Faillable(Result<Box<Self>, String>),
        StrPointer(StrPointer),
        Pointer(*const Self),
        PointerMut(*mut Self),
        Runtime(AnyWrapper),
        AsyncTask(AppliesEq<JoinHandle<Self>>),
        Listener(AppliesEq<UnboundedReceiver<Self>>),
        RuntimeRaw(&'static str, AppliesEq<RawRTValue>),
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::marker::StructuralPartialEq for BufValue {}
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::cmp::PartialEq for BufValue {
        #[inline]
        fn eq(&self, other: &BufValue) -> bool {
            let __self_discr = ::core::intrinsics::discriminant_value(self);
            let __arg1_discr = ::core::intrinsics::discriminant_value(other);
            __self_discr == __arg1_discr
                && match (self, other) {
                    (BufValue::Int(__self_0), BufValue::Int(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::U_Int(__self_0), BufValue::U_Int(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Float(__self_0), BufValue::Float(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Str(__self_0), BufValue::Str(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Bool(__self_0), BufValue::Bool(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Array(__self_0), BufValue::Array(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Object(__self_0), BufValue::Object(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Faillable(__self_0), BufValue::Faillable(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::StrPointer(__self_0), BufValue::StrPointer(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Pointer(__self_0), BufValue::Pointer(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::PointerMut(__self_0), BufValue::PointerMut(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Runtime(__self_0), BufValue::Runtime(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::AsyncTask(__self_0), BufValue::AsyncTask(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (BufValue::Listener(__self_0), BufValue::Listener(__arg1_0)) => {
                        __self_0 == __arg1_0
                    }
                    (
                        BufValue::RuntimeRaw(__self_0, __self_1),
                        BufValue::RuntimeRaw(__arg1_0, __arg1_1),
                    ) => __self_0 == __arg1_0 && __self_1 == __arg1_1,
                    _ => unsafe { ::core::intrinsics::unreachable() }
                }
        }
    }
    #[automatically_derived]
    #[allow(non_camel_case_types)]
    impl ::core::fmt::Debug for BufValue {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match self {
                BufValue::Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Int",
                        &__self_0,
                    )
                }
                BufValue::U_Int(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "U_Int",
                        &__self_0,
                    )
                }
                BufValue::Float(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Float",
                        &__self_0,
                    )
                }
                BufValue::Str(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Str",
                        &__self_0,
                    )
                }
                BufValue::Bool(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Bool",
                        &__self_0,
                    )
                }
                BufValue::Array(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Array",
                        &__self_0,
                    )
                }
                BufValue::Object(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Object",
                        &__self_0,
                    )
                }
                BufValue::Faillable(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Faillable",
                        &__self_0,
                    )
                }
                BufValue::StrPointer(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "StrPointer",
                        &__self_0,
                    )
                }
                BufValue::Pointer(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Pointer",
                        &__self_0,
                    )
                }
                BufValue::PointerMut(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "PointerMut",
                        &__self_0,
                    )
                }
                BufValue::Runtime(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Runtime",
                        &__self_0,
                    )
                }
                BufValue::AsyncTask(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "AsyncTask",
                        &__self_0,
                    )
                }
                BufValue::Listener(__self_0) => {
                    ::core::fmt::Formatter::debug_tuple_field1_finish(
                        f,
                        "Listener",
                        &__self_0,
                    )
                }
                BufValue::RuntimeRaw(__self_0, __self_1) => {
                    ::core::fmt::Formatter::debug_tuple_field2_finish(
                        f,
                        "RuntimeRaw",
                        __self_0,
                        &__self_1,
                    )
                }
            }
        }
    }
    unsafe impl Send for BufValue {}
    unsafe impl Sync for BufValue {}
    impl From<String> for BufValue {
        fn from(item: String) -> Self {
            Self::Str(item)
        }
    }
    impl From<i64> for BufValue {
        fn from(item: i64) -> Self {
            Self::Int(item)
        }
    }
    impl From<u64> for BufValue {
        fn from(item: u64) -> Self {
            Self::U_Int(item)
        }
    }
    impl From<f64> for BufValue {
        fn from(item: f64) -> Self {
            Self::Float(item)
        }
    }
    impl From<bool> for BufValue {
        fn from(item: bool) -> Self {
            Self::Bool(item)
        }
    }
    impl From<StrPointer> for BufValue {
        fn from(item: StrPointer) -> Self {
            Self::StrPointer(item)
        }
    }
    impl From<AnyWrapper> for BufValue {
        fn from(item: AnyWrapper) -> Self {
            Self::Runtime(item)
        }
    }
    impl From<AppliesEq<JoinHandle<Self>>> for BufValue {
        fn from(item: AppliesEq<JoinHandle<Self>>) -> Self {
            Self::AsyncTask(item)
        }
    }
    pub struct UnsafeSend<F>(pub F);
    unsafe impl<F> Send for UnsafeSend<F> {}
    unsafe impl<F> Sync for UnsafeSend<F> {}
    impl<F: Future> Future for UnsafeSend<F> {
        type Output = F::Output;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            unsafe { self.map_unchecked_mut(|s| &mut s.0).poll(cx) }
        }
    }
    pub fn make_unsafe_send_future<F>(fut: F) -> UnsafeSend<F>
    where
        F: Future,
    {
        UnsafeSend(fut)
    }
    pub struct AppliesEq<T>(pub T);
    #[automatically_derived]
    impl<T: ::core::fmt::Debug> ::core::fmt::Debug for AppliesEq<T> {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_tuple_field1_finish(f, "AppliesEq", &&self.0)
        }
    }
    unsafe impl<T> Send for AppliesEq<T> {}
    unsafe impl<T> Sync for AppliesEq<T> {}
    impl<T> PartialEq for AppliesEq<T> {
        fn eq(&self, _: &Self) -> bool {
            false
        }
    }
    impl<T> Deref for AppliesEq<T> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<T> DerefMut for AppliesEq<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl BufValue {
        pub fn type_of(&self) -> String {
            match &self {
                BufValue::Array(_) => "array".to_string(),
                BufValue::Bool(_) => "bool".to_string(),
                BufValue::Float(_) => "float".to_string(),
                BufValue::Int(_) => "int".to_string(),
                BufValue::U_Int(_) => "u_int".to_string(),
                BufValue::Object(_) => "object".to_string(),
                BufValue::StrPointer(_) | BufValue::Str(_) => "string".to_string(),
                BufValue::Faillable(res) => {
                    match res {
                        Ok(t) => {
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("<success {0}>", t.type_of()),
                                );
                                res
                            })
                        }
                        Err(t) => {
                            ::alloc::__export::must_use({
                                let res = ::alloc::fmt::format(
                                    format_args!("<err {0}>", &t),
                                );
                                res
                            })
                        }
                    }
                }
                BufValue::Runtime(d) => {
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(
                            format_args!("<runtime {0:?}>", d.type_id()),
                        );
                        res
                    })
                }
                BufValue::Pointer(ptr) => {
                    if ptr.is_null() {
                        return "<ptr *ref NULL>".into();
                    }
                    unsafe { &**ptr }.type_of()
                }
                BufValue::PointerMut(ptr) => {
                    if ptr.is_null() {
                        return "<ptr *mut NULL>".into();
                    }
                    unsafe { &**ptr }.type_of()
                }
                BufValue::Listener(_) => "<listener ?event>".into(),
                BufValue::AsyncTask(t) => {
                    if t.is_finished() {
                        "<async recv...\\0>".into()
                    } else {
                        "<async pending...>".into()
                    }
                }
                BufValue::RuntimeRaw(_, _) => "<runtime rt>".into(),
            }
        }
        pub fn display(&self) -> String {
            match &self {
                BufValue::Array(c) => {
                    c.iter().map(|x| x.display()).collect::<Vec<_>>().join(", ")
                }
                BufValue::Bool(a) => a.to_string(),
                BufValue::Float(f) => f.to_string(),
                BufValue::Int(i) => i.to_string(),
                BufValue::U_Int(u) => u.to_string(),
                BufValue::Object(c) => {
                    ::alloc::__export::must_use({
                        let res = ::alloc::fmt::format(format_args!("{0:#?}", c));
                        res
                    })
                }
                BufValue::Str(c) => c.to_string(),
                BufValue::StrPointer(c) => c.to_string(),
                e => e.type_of(),
            }
        }
        pub fn get_vec_mut(&mut self) -> Option<&mut Vec<BufValue>> {
            match self {
                BufValue::Array(a) => Some(a),
                _ => None,
            }
        }
        pub fn gt(&self, other: &BufValue) -> bool {
            match (self, other) {
                (BufValue::Int(a), BufValue::Int(b)) => a > b,
                (BufValue::Int(a), BufValue::U_Int(b)) => (*a as i128) > (*b as i128),
                (BufValue::U_Int(a), BufValue::U_Int(b)) => a > b,
                (BufValue::U_Int(a), BufValue::Int(b)) => (*a as i128) > (*b as i128),
                (BufValue::Float(a), BufValue::Float(b)) => a > b,
                _ => false,
            }
        }
        pub fn lt(&self, other: &BufValue) -> bool {
            match (self, other) {
                (BufValue::Int(a), BufValue::Int(b)) => a < b,
                (BufValue::Int(a), BufValue::U_Int(b)) => (*a as i128) < (*b as i128),
                (BufValue::U_Int(a), BufValue::U_Int(b)) => a < b,
                (BufValue::U_Int(a), BufValue::Int(b)) => (*a as i128) < (*b as i128),
                (BufValue::Float(a), BufValue::Float(b)) => a < b,
                _ => false,
            }
        }
        pub fn eq(&self, other: &BufValue) -> bool {
            match (self, other) {
                (BufValue::Int(a), BufValue::U_Int(b)) => (*a as i128) == (*b as i128),
                (BufValue::U_Int(a), BufValue::Int(b)) => (*a as i128) == (*b as i128),
                _ => self == other,
            }
        }
    }
}
pub mod val {
    use std::{fmt::Display, process, sync::LazyLock};
    static INFO: LazyLock<String> = LazyLock::new(|| {
        String::from_utf8_lossy(
                &[
                    27,
                    91,
                    51,
                    52,
                    109,
                    27,
                    91,
                    49,
                    109,
                    73,
                    78,
                    70,
                    79,
                    58,
                    32,
                    27,
                    91,
                    109,
                ],
            )
            .to_string()
    });
    static WARN: LazyLock<String> = LazyLock::new(|| {
        String::from_utf8_lossy(
                &[
                    27,
                    91,
                    51,
                    51,
                    109,
                    27,
                    91,
                    49,
                    109,
                    87,
                    65,
                    82,
                    78,
                    58,
                    32,
                    27,
                    91,
                    109,
                ],
            )
            .to_string()
    });
    static ERROR: LazyLock<String> = LazyLock::new(|| {
        String::from_utf8_lossy(
                &[27, 91, 51, 49, 109, 27, 91, 49, 109, 69, 82, 82, 58, 32, 27, 91, 109],
            )
            .to_string()
    });
    pub fn info<T: Display>(msg: T) {
        {
            ::std::io::_print(format_args!("{0}{1}\n", *INFO, msg));
        };
    }
    pub fn warn<T: Display>(msg: T) {
        {
            ::std::io::_print(format_args!("{0}{1}\n", *WARN, msg));
        };
    }
    fn gen_build() -> usize {
        let [major, minor, patch] = "0.0.0-dev-lead-lang"
            .split(".")
            .collect::<Vec<_>>()[..] else {
            return 0;
        };
        let major = major.parse::<usize>().unwrap_or(0);
        let minor = minor.parse::<usize>().unwrap_or(0);
        let patch = patch.parse::<usize>().unwrap_or(0);
        (major * 1000) + (minor * 100) + patch
    }
    pub fn error<T: Display, F: Display>(msg: T, file: F) -> ! {
        {
            ::std::io::_print(format_args!("{0}{1}\n", *ERROR, msg));
        };
        {
            ::std::io::_print(
                format_args!("{0}--------    TRACE    --------\n", *ERROR),
            );
        };
        {
            ::std::io::_print(format_args!("{0} File: {1}\n", *ERROR, file));
        };
        {
            ::std::io::_print(
                format_args!(
                    "{0} Edition {1}\n",
                    *ERROR,
                    "0.0.0-dev-lead-lang".split_at(1).0,
                ),
            );
        };
        {
            ::std::io::_print(
                format_args!("{0} Lead v{1}\n", *ERROR, "0.0.0-dev-lead-lang"),
            );
        };
        {
            ::std::io::_print(format_args!("{0} Build #{1}\n", *ERROR, gen_build()));
        };
        {
            ::std::io::_print(format_args!("{0} Compiled with Rust Nightly\n", *ERROR));
        };
        {
            ::std::io::_print(
                format_args!("{0}-----------------------------\n", *ERROR),
            );
        };
        process::exit(1);
    }
}
pub(crate) use package::*;
use tokio::runtime::{Builder, Runtime};
use types::{Extends, Heap, LanguagePackages, MethodRes, PrototypeDocs};
pub use val::*;
pub use tokio;
pub use lealang_chalk_rs::Chalk;
pub static VERSION_INT: u16 = 7;
static RUNTIME: LazyLock<Runtime> = LazyLock::new(|| {
    Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("Unable to build async runtime")
});
pub trait Package: Sync {
    fn name(&self) -> &'static [u8];
    fn doc(&self) -> HashMap<&'static str, &'static [&'static str; 3]> {
        HashMap::new()
    }
    fn prototype_docs(&self) -> PrototypeDocs {
        PrototypeDocs::default()
    }
    fn prototype(&self) -> Extends {
        Extends::default()
    }
    fn methods(&self) -> MethodRes {
        &[]
    }
}
pub struct RespPackage {
    pub methods: MethodRes,
}
pub struct Application<'a> {
    code: HashMap<String, String>,
    pub(crate) pkg: LanguagePackages<'a>,
    entry: &'a str,
    heap: Heap,
    module_resolver: Box<dyn FnMut(&str) -> Vec<u8>>,
    pkg_resolver: Box<dyn FnMut(&str) -> Vec<RespPackage>>,
    log_info: Box<dyn FnMut(&str) -> ()>,
    pub(crate) runtime: &'static Runtime,
    inst: Instant,
}
unsafe impl Send for Application<'_> {}
unsafe impl Sync for Application<'_> {}
impl<'a> Application<'a> {
    pub fn new<
        T: FnMut(&str) -> Vec<u8> + 'static,
        F: FnMut(&str) -> Vec<RespPackage> + 'static,
        R: FnMut(&str) -> () + 'static,
    >(file: &'a str, mut fs_resolver: T, dll_resolver: F, requested_perm: R) -> Self {
        let main = String::from_utf8(fs_resolver(file)).expect("Invalid utf8");
        let mut code = HashMap::new();
        code.insert(":entry".to_string(), main);
        Self {
            code,
            pkg: LanguagePackages::new(),
            heap: Heap::new(),
            entry: &file,
            module_resolver: Box::new(fs_resolver),
            pkg_resolver: Box::new(dll_resolver),
            log_info: Box::new(requested_perm),
            runtime: &*RUNTIME,
            inst: Instant::now(),
        }
    }
    pub fn add_file(&mut self, name: String, file: String) -> &mut Self {
        self.code.insert(name, file);
        self
    }
    pub fn add_pkg<T: Package + 'static>(&mut self, package: T) -> &mut Self {
        self.pkg.import(package);
        self
    }
    pub fn add_pkg_static(&mut self, package: &'static dyn Package) -> &mut Self {
        self.pkg.import_static(package);
        self
    }
    pub fn add_pkg_box(&mut self, package: Box<dyn Package>) -> &mut Self {
        self.pkg.import_dyn(package);
        self
    }
    pub fn add_pkg_raw(&mut self, name: &'static [u8], methods: MethodRes) -> &mut Self {
        let pkg = ImplPackage { name, methods };
        self.pkg.import(pkg);
        self
    }
    pub fn list_cmds(&mut self) -> &mut Self {
        let mut chalk = Chalk::new();
        chalk.red().bold();
        chalk.println(&"The Lead Programming Language");
        chalk.reset_weight().yellow().println(&"Interpreter");
        self.pkg.list(&mut chalk);
        self
    }
    /// ⚠️ This function still is panicking
    pub fn run_non(mut self) -> Duration {
        ipreter::interpret(":entry", &mut self);
        self.inst.elapsed()
    }
    pub fn run(self, time: bool) -> ! {
        let dur = self.run_non();
        if time {
            {
                ::std::io::_print(format_args!("\nTime Elasped: {0:?}\n", dur));
            };
        }
        process::exit(0)
    }
}
