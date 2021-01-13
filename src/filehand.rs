/*
* Made by salmmanfred 2021/01/13
* MIT license
*/


use std::fs::File;
use std::io::{prelude::*,BufReader};
use std::path::Path;



pub fn readFile(names: &str) -> String {
    let s = "".to_string();
    let fnm = s+&names;
    let contents = match std::fs::read_to_string(fnm){
        Err(why) => panic!("couldn't read file: {}", why),
        Ok(contents) => contents,
    };
    
    return contents;
}



pub fn writeFile(names:&str, cont:&str) -> i8 {
    let s = "".to_string();
    let fnm = s+&names;
    let p: &str = &fnm;
    let path = Path::new(p);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(cont.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    return 1;
    
    

    
    

}
pub fn readFileLines(fnm:&str) -> Vec<String> {
    let x: &str = &fnm;
    let mut errcode = "Error".to_owned();
    errcode.push_str(x);
    let file = File::open(x).expect(&errcode);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
fn main(){

}