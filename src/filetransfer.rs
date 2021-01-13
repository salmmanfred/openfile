use crate::filehand;



pub fn fileTransfer(fnm:&str,fnm2:&str) -> i8{
    // returns the i8 from wrtiefile and also takes the data from fnm2 and writes it to fnm
    // fnm == filename ( since you cant use fn)
    return filehand::writeFile(fnm, &filehand::readFile(fnm2));
}