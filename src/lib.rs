

pub mod filehand;
//pub use crate::filehand;
/*#[cfg(file)]
mod file {
    #[file]
    fn readFile(file:&str) -> String {
        return filehand::readFile(file)
    }
}*/
pub fn readFile(filename: &str) -> String{
    return filehand::readFile(filename);
}
pub fn readFileLines(filename: &str) -> Vec<String>{
    return filehand::readFileLines(filename);
}
pub fn writeFile(filename: &str,content: &str) -> i8{
    return filehand::writeFile(filename,content)
}