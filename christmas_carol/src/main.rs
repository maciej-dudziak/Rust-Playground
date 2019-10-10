fn main() {
    println!("Hello to carol printer!");
    let header = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eight", "ninth",
        "10th", "11th", "12th",
    ];
    let lyrics = [
        "A partridge in a pear tree",
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "10 lords a-leaping",
        "11 pipers piping",
        "12 drummers drumming",
    ];
    for i in 0..12 {
        println!(
            "On the {} day of Christmas my true love sent to me",
            header[i]
        );
        for line in (0..(i + 1)).rev() {
            println!("{}", lyrics[line]);
        }
    }
}
