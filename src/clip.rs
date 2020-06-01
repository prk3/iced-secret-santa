use std::error::Error;
use clipboard::*;
use std::sync::{Arc, Mutex};
use lazy_static::*;

pub use clipboard::ClipboardContext;

lazy_static! {
    static ref CLIP: Arc<Mutex<ClipboardContext>> = Arc::new(Mutex::new(ClipboardProvider::new().unwrap()));
}

/*
pub fn read() -> Result<String, Box<dyn Error>> {
    let clip_arc = CLIP.clone();
    let mut clip = clip_arc.lock().unwrap();
    clip.get_contents()
}
*/

pub fn write(value: String) -> Result<(), Box<dyn Error>> {
    let clip_arc = CLIP.clone();
    let mut clip = clip_arc.lock().unwrap();
    clip.set_contents(value)
}
