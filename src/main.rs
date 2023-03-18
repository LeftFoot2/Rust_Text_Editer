use std::fs::File;
use std::io::prelude::*;
use std::io::{Write, BufReader};


fn rust_if_statements(){
    let number = 5;

    // Use of if statements in Rust.

    if number < 3{
        println!("Number is less then three.")
    }
    else if number == 3{
        println!("Number is equal to three.")
    }
    else if number > 3{
        println!("Number is greater then three.")
    }

    // COunt to number with Rust for loop.

    for i in 1..number + 1{
        println!("{}", i)
    }

}

// Returns copy of the book that has the changes.

fn replace_words(copy: String) -> String {
    let mut _copy_of_txt = String::new();
    _copy_of_txt = copy;

    // Replace finds and replaces the first word with the second.
    // You can add as many filters as you want.

    // These are just silly examples of replacement words. In really life you would change them to words that you don't want to read
    // such as curse words.

    // This is far from perfect and will likely mess up some the text.

    _copy_of_txt = _copy_of_txt.replace(" not", " dog");
    _copy_of_txt = _copy_of_txt.replace(" because", " cat");
    _copy_of_txt = _copy_of_txt.replace(" axe", " rubber chicken");

    _copy_of_txt

}


fn main() -> std::io::Result<()> {

    /* If you want to copy in a book you must create a new book first. You must un comment these next two lines of code and run them after commenting out the rest of the code with exception of the Ok(()) at the end. */
    /* If you have a txt file of your book already then right click then put it into the path to the left of the screen and name it Book.txt */


        // let book = "Book.txt";

        // let og_book = File::create(book);


        // This assumes that you have done the steps above to get a txt file into your program. If you have not this code will have issues.

        // SIde note: Although I changed this a little bit the big chuck comes from the doc.rust-lang.org website and a youtube video.

        let og_book = File::open("Book.txt")?;
        
        let mut read_book = BufReader::new(og_book);
        let mut contents = String::new();
        read_book.read_to_string(&mut contents)?;

        let new_copy = "Book_Copy.txt";

        // Output needed to be a file type.
        // This code comes from the youtube video.

        let output = File::create(new_copy);

        let mut output = match output{
            Ok(file) => file,
            Err(error) => {
                panic!("Problem : {:?}",
            error);
            }
        };


        // Calls the replace words function so that the words that you don't want in your book will be replaced.
    
        let best_copy = replace_words(contents);

        write!(output, "{}", best_copy);

        // Calls rust if statements function to show how if statements can me used in Rust, along with a for loop.

        rust_if_statements();

        Ok(())


}