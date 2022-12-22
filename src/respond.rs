use crate::calc::*;
use raylib::ffi::KeyboardKey::*;
use raylib::prelude::*;
use rand::random;

pub fn respond(f : &mut u64, handle : &RaylibHandle, saved : &mut Vec<u64>) {
	//if handle.is_key_down(KEY_Q) {::std::process::exit(0);}
	if handle.is_key_released(KEY_SPACE) {*f = random::<u64>();}
    if handle.is_key_released(KEY_S) {saved.push(*f);}

}