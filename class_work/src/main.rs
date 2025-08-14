fn main() {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth",
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"
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
        "Three French hens",
        "Two turtle doves",
        "A partridge in a pear tree"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me:", days[day]);

        for gift_index in (11 - day)..12 {
            if gift_index == 11 && day != 0 {
                println!("And {}", gifts[gift_index]);
            } else {
                println!("{}", gifts[gift_index]);
            }
        }

        println!(); // blank line between verses
    }
}
