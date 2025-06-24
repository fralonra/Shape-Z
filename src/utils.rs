use crate::editor::RENDERBUFFER;
use std::sync::Arc;

pub fn reset_render() {
    let rb = Arc::clone(&RENDERBUFFER);
    let mut buffer = rb.lock().unwrap();
    buffer.accum = 1;
}
