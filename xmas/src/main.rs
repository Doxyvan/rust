fn main() {
    let song_str= ["A partridge in a pear tree", "Two turtle doves", "Three French hens", "Four calling birds", "Five golden rings", "Six geese-a-laying","Seven swans-a-swimming","Eight maids-a-milking","Nine drummers drumming","Ten pipers piping","Eleven ladies dancing","Twelve lords-a-leaping"];
    let num_for_song = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let mut main_counter = 1;
    let mut counter = 1;
    loop {
        let num_of_year = num_for_song[main_counter-1];
        println!("On the {} day of Christmas", num_of_year);
        println!("My true love gavе to me");
        loop {
            let string_for_print = song_str[counter-1];
            println!("{}", string_for_print);
            counter = counter+1;
            if counter>main_counter{
                println!("");
                counter = 1;
                break
            }
        }

        main_counter = main_counter+1;
        if main_counter >12{
            break
        }
    }
}
