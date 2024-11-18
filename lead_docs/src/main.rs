use std::{env::consts::{OS, ARCH}, fs, path::Path, process};

use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use tao::{
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Icon, WindowBuilder}
};
use wry::{http::{HeaderValue, Response, StatusCode}, WebViewBuilder};

#[cfg(not(debug_assertions))]
use include_dir::{include_dir, Dir};

use crate::module::LeadModule;

#[cfg(not(debug_assertions))]
static FILES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/ui/dist");

mod module;

fn main() {
    let app = EventLoop::new();

    let window = WindowBuilder::new()
        .with_maximized(true)
        .with_focused(true)
        .with_title("Lead Lang Docs")
        .with_min_inner_size(LogicalSize {
            height: 500.0,
            width: 800.0
        })
        .with_window_icon(Some({
            let img = image::load_from_memory(include_bytes!("./icon.png")).unwrap();
            let img = img.as_rgba8().unwrap();
            let vect = img.to_vec();

            Icon::from_rgba(vect, img.height(), img.width()).unwrap()
        }))
        .build(&app)
        .unwrap();

    let is_workspace = fs::read_dir("./lib").is_ok();

    let webview = WebViewBuilder::new()
        .with_initialization_script(&format!("window.leadver = {:?}; window.os = {OS:?}; window.arch = {ARCH:?};\nwindow.workspace = {is_workspace}", env!("CARGO_PKG_VERSION")))
        .with_asynchronous_custom_protocol("app".into(), |_, req, res| {
            #[cfg(not(debug_assertions))]
            {
                let url = req.uri().to_string();
                let path = url.replace("app://", "").replace("localhost/", "");

                let file = FILES.get_file(&path).unwrap();
                
                let mut resp = Response::new(
                    file.contents()
                );

                resp.headers_mut().append("Content-Type", HeaderValue::from_str({
                    if path.ends_with(".html") {
                        "text/html"
                    } else if path.ends_with("js") {
                        "text/javascript"
                    } else if path.ends_with(".css") {
                        "text/css"
                    } else {
                        ""
                    }
                }).unwrap());

                res.respond(resp);
            }
        })
        .with_asynchronous_custom_protocol("api".into(), |_, req, res| {
            let url = req.uri();
            let pathname = url.path();

            let mut status = StatusCode::NOT_FOUND;
            let mut body = String::new();
            if pathname == "/base_pkg" {
                status = StatusCode::OK;
                body = to_string_pretty(&base_docs()).unwrap_or("".into());
            }

            res.respond(Response::builder()
                .status(status)
                .header("Access-Control-Allow-Origin", "*")
                .body(body.into_bytes())
                .unwrap()
            );
        });

    #[cfg(debug_assertions)]
    let webview = webview.with_devtools(true).with_url("http://localhost:5173");

    #[cfg(all(windows, not(debug_assertions)))]
    let webview = webview.with_url("http://app.localhost/index.html");

    #[cfg(all(not(windows), not(debug_assertions)))]
    let webview = webview.with_url("app://localhost/index.html");
    
    let _webview = webview
        .build(&window)
        .unwrap();

    app.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match ev {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit
            },
            _ => {}
        }
    });
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LeadPackage {
    pub name: String,
    pub modules: Vec<LeadModule>
}

fn base_docs() -> Vec<LeadPackage> {
    use std::env::var;

    let Ok(lead_home) = var("LEAD_HOME") else {
        println!("ERR ** LEAD_HOME environment variable not set");
        process::exit(1);
    };
    let path = Path::new(&lead_home);
    let mut path = path.to_owned();

    path.push("docs");

    let mut res = vec![];

    for pkg in fs::read_dir(&path).unwrap() {
        let pkg = pkg.unwrap();

        let own = pkg.file_name();
        let own = own.to_str().unwrap_or("unknown");

        let mut path = pkg.path();
        path.push("pkg");

        let packages = fs::read_to_string(&path).expect("Failed to read pkg entry");
        let pkgs = packages.split("\n")
            .filter(|x| x.contains("->"))
            .collect::<Vec<_>>();

        path.pop();
        path.push("file");
        let map_file = fs::read_to_string(&path).expect("File read failed somehow");
        let refs = map_file.split("\n")
            .filter(|x| x.contains("->"))
            .collect::<Vec<_>>();

        let docs = LeadModule::new(own, refs, pkgs, true);
        res.push(LeadPackage { name: own.into(), modules: docs });
    }

    res
}