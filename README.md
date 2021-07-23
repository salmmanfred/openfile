# This is a cargo for easy opening of files.  

To get the file into a string: read_file(&str)  Returns string  
  
To get files line by line read_file_lines(&str) Retruns vector string  
  
To write to a file do : write_file(&str,&str) (file name, file content)  
  
To delete file remove_file(&str) to remove the file using the name of the file  
  
To save byte arrays do: write_file_bytes(&str,Vec u8)  
  
read_file_bytes (&str: filename)  