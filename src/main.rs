use enigo::{Enigo, MouseControllable};
use lazy_static::lazy_static;
use rdev::{listen, Event, EventType, Key};
use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, sleep};
use std::time::Duration;
use crate::circle::Circle;

mod time;
mod circle;
mod settings;
mod file;

// Make `stop_flag` static, so it can be accessed in a function pointer
lazy_static! {
    static ref STOP_FLAG: AtomicBool = AtomicBool::new(false);
    static ref RUN_FLAG: AtomicBool = AtomicBool::new(true);
}

fn event_listener(event: Event) {
    static SHIFT_PRESSED: AtomicBool = AtomicBool::new(false);
    fn print_action(action: bool) {
        if action {
            print!("\r### R E S U M E ####")
        } else {
            print!("\r### P A U S E ######")
        }
        io::stdout().flush().unwrap();
    }

    if let EventType::KeyPress(key) = event.event_type {
        if key == Key::Escape {
            STOP_FLAG.store(true, Ordering::SeqCst);
        }

        if key == Key::ShiftLeft {
            SHIFT_PRESSED.store(true, Ordering::SeqCst);
        }

        if SHIFT_PRESSED.load(Ordering::SeqCst) && key == Key::KeyP {
            RUN_FLAG.store(!RUN_FLAG.load(Ordering::SeqCst), Ordering::SeqCst);
            print_action(RUN_FLAG.load(Ordering::SeqCst));
        }
    }
    if let EventType::KeyRelease(key) = event.event_type {
        if key == Key::ShiftLeft {
            SHIFT_PRESSED.store(false, Ordering::SeqCst);
        }
    }
}

fn main() {
    let settings = settings::load_settings();
    match settings {
        Ok(mut set) => {
            let mut controller = Enigo::new();
            let drawing_circle = Circle::new();
            thread::spawn(move || {
                if let Err(e) = listen(event_listener) {
                    eprintln!("Error while listening: {:?}", e);
                }
            });
            loop {
                //TODO: Vill bara uppdatera tiden varje kvart eller halvtimma
                // men då behövs väl en tidstämpel att jämföra med.
                if set.now >= set.stop_time {
                    print!("\rNow is: {}. Stopping", set.now.format("%H:%M:%S"));
                    break;
                }

                if STOP_FLAG.load(Ordering::SeqCst) {
                    print!("\r### BYE FOR NOW! ###");
                    break;
                }
                //TODO: RUN_FLAG borde styras av att man rör musen
                if RUN_FLAG.load(Ordering::SeqCst) {
                    if set.now < set.lunch_start_time || set.now >= set.lunch_stop_time {
                        let x_coord = Circle::get_x_coord(&drawing_circle);
                        let y_coord = Circle::get_y_coord(&drawing_circle);
                        controller.mouse_move_to(x_coord, y_coord);
                        // controller.mouse_click(MouseButton::Left);

                        Circle::increase_angle(&drawing_circle);
                        sleep(Duration::from_millis(drawing_circle.wait_time));
                    }
                }
                set.now = time::now();
            }
        }
        _ => {
            panic!("FAILED TO START PROGRAM; FAILED TO DELIVER SOLUTION")
        }
    }

}
