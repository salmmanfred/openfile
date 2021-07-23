/*
* Made by salmmanfred 2021/01/13
* MIT license
*/

/*
* Made by salmmanfred 2021/01/13
* MIT license
*/

use std::fs::File;
use std::io::{prelude::*,BufReader};
use std::path::Path;
use std::fs;

pub fn remove_file(fnm:&str) -> std::io::Result<()> {
    fs::remove_file(fnm)?;
    Ok(())
}

pub fn read_file(names: &str) -> String {
    //reads the file from names
    let s = "".to_string();
    let fnm = s+&names;
    let contents =  std::fs::read_to_string(fnm).unwrap();
    return contents;

    
}



pub fn write_file(names:&str, cont:&str) -> std::io::Result<()> {
    // writes to a file
    let s = "".to_string();
    let fnm = s+&names;
    let path = Path::new(&fnm);
    

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    file.write_all(cont.as_bytes())?;
    
    Ok(())
    

    
    

}
pub fn read_file_lines(fnm:&str) -> Vec<String> {
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

pub fn write_file_bytes(names:&str, cont:Vec<u8>) -> std::io::Result<()> {
    // writes to a file
    let s = "".to_string();
    let fnm = s+&names;
    let path = Path::new(&fnm);
    

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(&path)?;
    //use std::io::Cursor;

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    for x in cont{
        file.write(&[x])?;
    }
    Ok(())

    

}
pub fn read_file_bytes(filename: &str) -> Vec<u8> {
    let mut f = File::open(&filename).expect("no file found");
    let metadata = fs::metadata(&filename).expect("unable to read metadata");
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect("buffer overflow");

    buffer
}


