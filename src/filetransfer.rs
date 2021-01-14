use crate::filehand;


/*
* Made by salmmanfred 2021/01/13
* MIT license
*/
pub fn fileTransfer(fnm:&str,fnm2:&str) -> i8{
    // returns the i8 from wrtiefile and also takes the data from fnm2 and writes it to fnm
    // fnm == filename ( since you cant use fn)
    return filehand::writeFile(fnm, &filehand::readFile(fnm2));
}
pub fn fileSwap(fnm:&str,fnm2:&str) -> i8{
    // gets the 2 files into varibles 
    let mut x = filehand::readFile(fnm);
    let mut y = filehand::readFile(fnm2);
    // make sure they are not empty
    if x == ""{
        x.push_str(" ")
    }
    if y == ""{
        y.push_str(" ")
    }
    // writes too the files
    let xr = filehand::writeFile(fnm, &y);
    let yr = filehand::writeFile(fnm2, &x);
    // returns the 2 combined values of the write functions
    return xr + yr;
}