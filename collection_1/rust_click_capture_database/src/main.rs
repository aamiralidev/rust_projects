extern crate num;
#[macro_use]
extern crate num_derive;


mod database;

use database::{Key, KeyboardEvent, Modifier, MouseEvent, EventDatabase, MouseButton, Coordinates};
use std::fs::File;
use std::io::{Read, Write};
//```rust
//use fltk::{app, button::Button, enums::Color, frame::Frame, image, prelude::*, window::Window};
//use std::io;
use winput::{Input, Vk, Action, MouseMotion, Button};
use winput::message_loop;
use std::time::{SystemTime, Duration};
use crate::Key::K;


fn main() {
    /*  let app = app::App::default().with_scheme(app::AppScheme::Gtk);
      let mut window = Window::new(0, 0, 200, 70, "Hello from rust");
          window.set_color(Color::White);
      //let mut frame = Frame::new(0, 0, 325, 125, "");
      let mut play_button = Button::default()
          .with_size(50,50)
          .with_label("Play")
          .with_pos(10, 10);
      let mut stop_button = Button::default()
          .with_size(50,50)
          .with_label("Stop")
          //.with_pos(50,50)
          .right_of(&play_button,10);
      let graph_button = Button::new(0, 0, 50, 50, "Graph")
          .right_of(&stop_button, 10);

      let play_img = image::PngImage::load(&std::path::Path::new("Play Button Png.png")).unwrap();
      let (s,r) = app::channel::<i32>();

      window.end();
      window.show();
      //play_button.set_callback(move |_| stop_button.set_color(Color::Red));
      play_button.emit(s,1);
      play_button.clone().set_callback(move |_| play_button.set_label("Recording"));

      app.run().unwrap();
      while app.wait() {
          match r.recv() {
              Some(val) => startListen(),
              None => (),

          }
      }*/


    startListen();


}

fn startListen(){
    let mut event_database = EventDatabase::load_database("database.db".to_string());
    let receiver = message_loop::start().unwrap();
    //let sys_time = SystemTime::now();

    //let new_sys_time = SystemTime::now();
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Release,
                ..
            } => {
                if vk == Vk::Escape {
                    break;
                } else {
                    event_database.add_keyboard_event(
                        KeyboardEvent{
                            // key: Key::new(vk),
                            key: Key::from(vk),
                            modifier: Modifier::Release,
                            event_time: SystemTime::now()
                        }
                    );
                    println!("{:?} was release!", vk);
                    //sys_time = SystemTime::now();
                    //Goes into database
                }
            },

            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::Escape {
                    break;
                } else {
                    event_database.add_keyboard_event(
                        KeyboardEvent{
                            // key: Key::new(vk),
                            key: Key::from(vk),
                            modifier: Modifier::Press,
                            event_time: SystemTime::now()
                        }
                    );
                    //let sys_time = SystemTime::now();
                    println!("{:?} was pressed!", vk);
                }
            },
            message_loop::Event::MouseButton{
                button,
                action: Action::Press,
            } => {
                event_database.add_mouse_event(
                    MouseEvent{
                        button: MouseButton::new(button),
                        modifier: Modifier::Press,
                        event_time: SystemTime::now(),
                        event_coordinate: Coordinates { x: 0.0, y: 0.0 }
                    }
                );
                println!("{:?} was clicked", button);
            },

            message_loop::Event::MouseButton{
                button,
                action: Action::Release,
            } => {
                event_database.add_mouse_event(
                    MouseEvent{
                        button: MouseButton::new(button),
                        modifier: Modifier::Release,
                        event_time: SystemTime::now(),
                        event_coordinate: Coordinates { x: 0.0, y: 0.0 }
                    }
                );
                println!("{:?} was released! Time was", button);
            },
            message_loop::Event::MouseMoveAbsolute{
                x,
                y,
                ..
            } => {
                event_database.add_mouse_event(
                    MouseEvent{
                        button: MouseButton::Left,
                        modifier: Modifier::Press,
                        event_time: SystemTime::now(),
                        event_coordinate: Coordinates { x, y }
                    }
                );
                println!("Mouse is located at X:{:?} and Y: {:?}", x, y);
            }
            _=> (),

        }
        event_database.save_database("database.db".to_string());
    }
}//```