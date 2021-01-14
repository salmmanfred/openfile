This is a cargo for easy opening of files.

To get the file into a string: readFile(&str)  Returns string    

To get files line by line readFileLines(&str) Retruns vector string

To write to a file do : writeFile(&str,&str) (file name, file content) Returns i8(1 if success)  

To Transfer a file (or copy a file from one file to another)  use: fileTransfer(&str,&str) (The file that the contents will be dumped to)
(the file its copying from) returns the i8 results from writeFile  

To swap files you need to do: fileSwap(&str,&str) the 2 strings are the 2 files that are being changed Returns 2 if success 2 being the combined value from writefile returns i8