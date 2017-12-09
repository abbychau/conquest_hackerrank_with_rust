use std::io;
fn main() {
    // variable declaration
    let mut _num_str_count = String::new();
    io::stdin().read_line(&mut _num_str_count).ok().expect("read error");
    
    let mut _num_str_1 = String::new();
    io::stdin().read_line(&mut _num_str_1).ok().expect("read error");
    
    let mut _nums_ = _num_str_1.trim().split(" ");
    let mut _max:u64=std::u64::MIN;
    let mut vec = Vec::new();
    for s in _nums_ {
        let _num:u64 = s.trim().parse().ok().expect("parse error");
        vec.push(_num);
    }
    for x in &vec {
        if x > &_max { 
            _max = *x;
        }
    }
    let mut _count:u64=0;
    for x in &vec {
        if x == &_max { 
            _count = _count+1;
        }
    }
    println!("{}",_count)

}
