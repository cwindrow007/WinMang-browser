use std::path::PathBuf;

use tao::{
    event::{Event, WindowEvent, ElementState},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
    keyboard::Key,
};

use wry::WebViewBuilder;

fn main() -> wry::Result<()> {


        let logo = r#"
 ___       __   ___  ________                 _____ ______   ________  ________  ________
|\  \     |\  \|\  \|\   ___  \               |\   _ \  _   \|\   __  \|\   ___  \|\   ____\
\ \  \    \ \  \ \  \ \  \\ \  \  ____________\ \  \\\__\ \  \ \  \|\  \ \  \\ \  \ \  \___|
 \ \  \  __\ \  \ \  \ \  \\ \  \|\____________\ \  \\|__| \  \ \   __  \ \  \\ \  \ \  \  ___
  \ \  \|\__\_\  \ \  \ \  \\ \  \|____________|\ \  \     \ \  \ \  \ \  \ \  \\ \  \ \  \|\  \
   \ \____________\ \__\ \__\\ \__\              \ \__\     \ \__\ \__\ \__\ \__\\ \__\ \_______\
    \|____________|\|__|\|__| \|__|               \|__|      \|__|\|__|\|__|\|__| \|__|\|_______|
    "#;
        println!("{}", logo);
        println!("Booting... ");


        let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("WinMang Browser")
        .with_resizable(true)
        .build(&event_loop)
        .expect("Failed to create window");

    let webview = WebViewBuilder::new()
        .with_url("https://www.google.com")
        .build(&window)?;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent { event, .. } => match event {

                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }

                WindowEvent::KeyboardInput { event: kb_event, .. } => {
                    if kb_event.state == ElementState::Pressed {

                        if let Key::Character(s) = &kb_event.logical_key {
                            if s.eq_ignore_ascii_case("g") {
                                println!("G pressed → loading Google");

                                let _ = webview.load_url("https://www.google.com");
                            }
                        }
                    }
                }

                _ => (),
            },

            _ => (),
        }
    });
}