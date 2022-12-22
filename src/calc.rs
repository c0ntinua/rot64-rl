pub fn trident(x : u64 , i : u32) -> u64 { (7u64.rotate_left(i)&x).rotate_right(i) }
pub fn wings(x : u64, i : u32) -> u64 {trident(x,i)|(trident(x,i+4).rotate_left(3))}
pub fn eval(f : u64, i : u32) -> u64 {(1u64.rotate_left(i)&f).rotate_right(i)}
pub fn next(s : u64, f : u64, i : u32) -> u64 { eval(f,wings(s,i) as u32).rotate_left(i+3)}
pub fn turn(s : u64, f : u64)-> u64 { let mut r = 0u64; for i in 0..64 { r |= next(s,f,i);} r}
pub fn prnt(s : u64) { for i in 0..64 {_prnt(s,i);} print!("\n"); }
pub fn _prnt(s : u64, i : u32) { if eval(s,i) == 1 {print!("\u{2588}");} else {print!(" ");}}