# word-count

word-count is a program which can return a comprehensive list of all the words, and the number of occurances (roughly), in a PDF file.

By developing the program in Rust, word-count can parse large PDF files at a quick speed.

# Features

word-count can: 

* Find all words and their corresponding number of instances in a PDF
* Output to the terminal
* Save to JSON

# How to use word-count?

By running the executable in the same directory as your PDF file, the executable will be able to detect the PDF. From there, you can follow the configurations provided by the program.

> It is recommended to run this program in an empty folder. word-count **does** create directories/files.

The executable can be found in ***releases***

# Future Plans

This project is mostly complete. 

There may be plans to create a Rust crate, allowing other programmers to utilize the same functions that word-count uses. As of present day, this initiative has not be started.