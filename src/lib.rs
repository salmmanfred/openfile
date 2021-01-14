

pub mod filehand;
pub mod filetransfer;

pub fn readFile(filename: &str) -> String{
    return filehand::readFile(filename);
}
pub fn readFileLines(filename: &str) -> Vec<String>{
    return filehand::readFileLines(filename);
}
pub fn writeFile(filename: &str,content: &str) -> i8{
    return filehand::writeFile(filename,content)
}
pub fn fileTransfer(filename: &str,content: &str) -> i8{
    return filetransfer::fileTransfer(filename, content)
}
pub fn fileSwap(filename: &str,content: &str) -> i8{
    return filetransfer::fileSwap(filename, content)
}