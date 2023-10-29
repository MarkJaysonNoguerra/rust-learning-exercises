fn main() {
    let song_days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];

    let gifts = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];

    let mut count = 1;
    let last_index = song_days.len();

    print!("\n\n\nThe twelve Days of Christmas Lyrics \n\n\n");
    for day in song_days {
        println!("On the {day} day of Christmas, my true love sent to me");

        for gift in &gifts[(last_index - count)..] {
            println!("{gift}");
        }
        print!("\n\n");
        count += 1;
    }
}
