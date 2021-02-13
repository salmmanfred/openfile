/*
* Made by salmmanfred 2021/01/13
* MIT license
*/

use std::fs::File;
use std::io::{prelude::*,BufReader};
use std::path::Path;
use std::fs;

pub fn removeFile(fnm:&str) -> std::io::Result<()> {
    fs::remove_file(fnm)?;
    Ok(())
}

pub fn readFile(names: &str) -> String {
    //reads the file from names
    let s = "".to_string();
    let fnm = s+&names;
    let contents =  std::fs::read_to_string(fnm).unwrap();
    return contents;

    
}



pub fn writeFile(names:&str, cont:&str) -> i8 {
    // writes to a file
    let s = "".to_string();
    let fnm = s+&names;
    let path = Path::new(&fnm);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(cont.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => print!(""),
    }
    return 1;
    
    

    
    

}
pub fn readFileLines(fnm:&str) -> Vec<String> {
    // reads the file line by line
    let x: &str = &fnm;
    let mut errcode = "Error".to_owned();
    errcode.push_str(x);
    let file = File::open(x).expect(&errcode);
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

pub fn writeFileBytes(names:&str, cont:Vec<u8>) -> i8 {
    // writes to a file
    let s = "".to_string();
    let fnm = s+&names;
    let path = Path::new(&fnm);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };
    //use std::io::Cursor;

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    for x in cont{
        match file.write(&[x]) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => print!(""),
        }
    }
    return 1;

    

}
