mod apps;
mod global_renderer;
mod input;
mod launcher;
mod ui;
use crate::apps::mainmenu::mainmenu::MainMenu;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use ui::main_menu::handle_main_menu_input;
use ui::main_menu::InputEvent as MenuInput;
use ui::state::UIScreen;
use ui::welcome::handle_welcome_input;
use ui::welcome::InputEvent as WelcomeInput;

fn main() {
    let sdl_context = sdl2::init().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut screen = UIScreen::Welcome;

    // Create an instance of your Renderer
    // let mut renderer = GlobalRenderer {
    //     canvas,
    //     texture_creator: &texture_creator, // Pass a reference to texture_creator
    //     font,
    // };

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    screen = match screen {
                        UIScreen::MainMenu(sel) => handle_main_menu_input(sel, MenuInput::Select),
                        UIScreen::Welcome => handle_welcome_input(WelcomeInput::Button(0)),
                    };
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Up),
                    ..
                } => {
                    if let UIScreen::MainMenu(sel) = screen {
                        screen = handle_main_menu_input(sel, MenuInput::Up);
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Down),
                    ..
                } => {
                    if let UIScreen::MainMenu(sel) = screen {
                        screen = handle_main_menu_input(sel, MenuInput::Down);
                    }
                }
                Event::MouseButtonDown { x, y, .. } => {
                    screen = handle_welcome_input(WelcomeInput::Touch(x, y));
                }
                _ => {}
            }
        }

        // Initialize SDL2 contexts
        // let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();
        let ttf_context = sdl2::ttf::init().unwrap();

        // Create window and canvas
        let window = video_subsystem
            .window("PipBoy", 800, 480)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        // Create the MainMenu renderer
        let mut renderer = MainMenu::new(canvas, &ttf_context);

        // In your game loop:
        let screen = UIScreen::Welcome; // or UIScreen::MainMenu(0)
        renderer.render(&screen);
        std::thread::sleep(Duration::from_millis(16));
    }
}
