use raylib::prelude::*;
pub fn handle_and_thread(height : i32, width : i32) -> (RaylibHandle, RaylibThread)  {
    let (mut handle, thread) = raylib::init().size(width,height).title("self-rs").build();
    (handle, thread)  
}