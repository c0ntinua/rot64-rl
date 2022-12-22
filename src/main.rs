use rand::random;
use raylib::prelude::*;
mod calc; use calc::*;
mod plot; use plot::*;
mod init; use init::*;
mod respond; use respond::*;


fn main() {
    let mut f = random::<u64>();
    let mut F = [0u64;64];
    let mut saved : Vec<u64> = vec!();
    let mut s = f;
    let (mut handle, thread) = handle_and_thread(32*32,32*32);
    unsafe{::raylib::ffi::SetTargetFPS(60);}
	while !handle.window_should_close() {
        respond(&mut f, &handle, &mut saved);
        F[0] = f;
        for i in 1..64 {F[i] = turn(F[i-1],f);}
        let mut screen = handle.begin_drawing(&thread);
        plot(&F,&mut screen);
    }
    for s in saved {
        print!("{:064b}\n",s);
    }

}



