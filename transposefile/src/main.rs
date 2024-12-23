use std::{collections, fs};

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn main() {
    // --snip--
    let file_path = "C:\\Users\\kotom\\OneDrive\\Документы\\Py\\Rust\\transposefile\\text.txt";

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let parts = contents.replace("\r", "");
    let a = parts.split("\n");
    let collection: Vec<&str> = a.collect();
    let first_row = collection.get(0).expect("none");
    let first_row_split= first_row.split(" ");
    let first_row_in_vec: Vec<&str> = first_row_split.clone().collect();

    let cnt_of_rows = collection.len();
    let cnt_of_cols = first_row_in_vec.len();
    let mut i = 0;
    let mut n = 0;

    while i < cnt_of_cols{
        n = 0;
        while n < cnt_of_rows{
            let n_row: Vec<&str> = (((collection.get(n).expect("none")).split(" ")).clone()).collect();
            let i_word = n_row.get(i).expect("none");
            print!("{} ", i_word);
            n+=1;
        }
        println!("");
        i+=1;
    }


}
        