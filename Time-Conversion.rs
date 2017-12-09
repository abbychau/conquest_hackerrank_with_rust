use std::io;
fn main() {
    // variable declaration
    let mut _input = String::new();
    io::stdin().read_line(&mut _input).ok().expect("read error");
    let apm:String = _input.chars().skip(8).take(1).collect();
    let hh:String = _input.chars().skip(0).take(2).collect();
    let numh:i32 = hh.parse().unwrap();
    let _t:String = _input.chars().skip(2).take(6).collect();
    
    let f2:String;
    if apm=="A" {
        if numh==12 {
            f2 = "00".to_string();
        }else{
            f2 = format!("{:02}", numh);
        }
    }else{
        if numh==12 {
            f2 = "12".to_string();
        }else{
            f2=(numh+12).to_string();
        }  
        
        
    }
    println!("{}{}",f2, _t);

}
