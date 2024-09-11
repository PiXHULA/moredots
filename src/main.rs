use crate::time_handler::Timestamp;
use circle::Circle;
use enigo::{Enigo, MouseControllable};
use lazy_static::lazy_static;
use rdev::{listen, Event, EventType, Key};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

mod time_handler;
mod circle;
mod settings;

// Make `stop_flag` static, so it can be accessed in a function pointer
lazy_static! {
    static ref STOP_FLAG: AtomicBool = AtomicBool::new(false);
    static ref RUN_FLAG: AtomicBool = AtomicBool::new(true);
}

fn event_listener(event: Event) {
    // Function to listen for key press events
    if let EventType::KeyPress(key) = event.event_type {
        if key == Key::Escape {
            STOP_FLAG.store(true, Ordering::SeqCst);
        }
        //TODO: Den här ska lyssna på Shiftleft + keyP  varför fungerar det inte att gör och?
        if key == Key::ShiftLeft && key == Key::KeyP {
            RUN_FLAG.store(!RUN_FLAG.load(Ordering::SeqCst), Ordering::SeqCst)
        }
    }
}

fn main() {
    let mut controller = Enigo::new();
    let drawing_circle = Circle::new();
    let mut worthy_clock = Timestamp::new();

    // Spawn a thread to listen for keyboard events
    thread::spawn(move || {
        if let Err(e) = listen(event_listener) {
            eprintln!("Error while listening: {:?}", e);
        }
    });

    loop {
        //TODO: Vill bara uppdatera tiden varje kvart eller halvtimma
        // men då behövs väl en tidstämpel att jämföra med.
        if worthy_clock.now >= worthy_clock.stop_time {
            println!("Now is: {}. Stopping", worthy_clock.now.format("%H:%M:%S"));
            break;
        }

        if STOP_FLAG.load(Ordering::SeqCst) {
            println!("Escape key pressed. Stopping.");
            break;
        }
        //TODO: RUN_FLAG borde styras av att man rör musen
        if RUN_FLAG.load(Ordering::SeqCst) {
            if worthy_clock.now < worthy_clock.lunch_start_time || worthy_clock.now >= worthy_clock.lunch_stop_time {
                let x_coord = Circle::get_x_coord(&drawing_circle);
                let y_coord = Circle::get_y_coord(&drawing_circle);
                controller.mouse_move_to(x_coord, y_coord);
                // controller.mouse_click(MouseButton::Left);

                Circle::increase_angle(&drawing_circle);
                sleep(Duration::from_millis(drawing_circle.wait_time));
            }
        }
        worthy_clock.now = Timestamp::now();
    }
}
