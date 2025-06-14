fn main() {
    let gifts = [
        "a partridge in a pear tree",
        "two turtle doves",
        "three French hens", 
        "four calling birds",
        "five golden rings",
        "six geese a-laying",
        "seven swans a-swimming",
        "eight maids a-milking",
        "nine ladies dancing",
        "ten lords a-leaping",
        "eleven pipers piping",
        "twelve drummers drumming",
    ];
    let days = [
        "first", "second", "third", "fourth", "fifth", 
        "sixth", "seventh", "eighth", "ninth", "tenth", 
        "eleventh", "twelfth"
    ];

    for day in 0..12 {
        println!("On the {} day of Christmas my true love gave to me:", days[day]);
        for gift in (0..=day).rev() {
            if day == 0 && gift == 0 {
                println!("A partridge in a pear tree.");
            } else if gift == 0 {
                println!("And a {}", gifts[gift]);
            } else {
                println!("{}", gifts[gift]);
            }
        }
        if day < 11 {
            println!("oops");
        }
    }
}
