use std::{env::{args, consts::{ARCH, OS}}, fs};

use lead_docs_lib::utils::package::Package;
use serde_json::to_string_pretty;
use tao::{
    dpi::LogicalSize, event::{Event, WindowEvent}, event_loop::{ControlFlow, EventLoop}, window::{Icon, WindowBuilder}
};
use wry::{http::{HeaderValue, Response, StatusCode}, WebViewBuilder, WebViewBuilderExtWindows};

#[cfg(not(debug_assertions))]
use include_dir::{include_dir, Dir};

#[cfg(not(debug_assertions))]
static FILES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/ui/dist");

fn main() {
    let arg = args().last().unwrap_or("".to_string());

    if &arg == "--cli" {
        lead_docs_lib::run();
        return;
    }

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

    let is_workspace = fs::read_dir("./.lead_libs").is_ok();

    let webview = WebViewBuilder::new()
        .with_initialization_script(&format!("window.leadver = {:?}; window.target = {:?}; window.os = {OS:?}; window.arch = {ARCH:?};\nwindow.workspace = {is_workspace}", env!("CARGO_PKG_VERSION"), env!("TARGET")))
        .with_https_scheme(true)
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
            
            if pathname == "/core" {
                status = StatusCode::OK;

                let docs = lead_docs_lib::utils::docs::lead_lib()
                    .into_iter()
                    .map(|x| Package::new(&x))
                    .collect::<Vec<_>>();

                body = to_string_pretty(&docs).unwrap_or("".into());
            } else if pathname == "/workspace" {
                status = StatusCode::OK;

                let docs = lead_docs_lib::utils::docs::lead_ws()
                    .into_iter()
                    .map(|x| Package::new(&x))
                    .collect::<Vec<_>>();

                body = to_string_pretty(&docs).unwrap_or("".into());
            }

            res.respond(Response::builder()
                .status(status)
                .header("Access-Control-Allow-Origin", "*")
                .body(body.into_bytes())
                .unwrap()
            );
        });

    #[cfg(debug_assertions)]
    let webview = webview.with_devtools(true).with_url("http://localhost:3000");

    #[cfg(all(windows, not(debug_assertions)))]
    let webview = webview.with_url("https://app.localhost/index.html");

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
