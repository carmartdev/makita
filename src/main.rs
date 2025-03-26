use core_graphics::event::{
    CGEvent, CGEventFlags, CGEventType, CGMouseButton,
};
use core_graphics::geometry::CGPoint;
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};
use std::thread;
use std::time::Duration;
use core_graphics::event::CGEventTapLocation;

fn main() {
    let event_source = CGEventSource::new(CGEventSourceStateID::HIDSystemState).unwrap();
    let point = CGPoint::new(0.0, 400.0); //click coordiantes

    let mouse_move_event = CGEvent::new_mouse_event(event_source, CGEventType::MouseMoved, point, CGMouseButton::Left);
    match mouse_move_event{
	Ok(event) => {
	    event.post(CGEventTapLocation::HID);
	}
	Err(_) => {
	    eprintln!("Failed to create mouse move event.");
            return;
	}
    }
    // Delay to allow the mouse to move.
    thread::sleep(Duration::from_millis(100));
}
