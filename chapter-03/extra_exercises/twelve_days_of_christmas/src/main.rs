const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth",
];

// First gift doesn't have a quantifier (such as "a"), because sometimes that "a" is capitalized
// and sometimes it isn't, and apparently capitalizing a letter is hard in rust (or is generally
// hard and every other language does it wrong)
const GIFTS: [&str; 12] = [
    "partridge in a pear tree",
    "Two turtledoves",
    "Three French hens",
    "Four calling birds",
    "Five gold rings (five golden rings)",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

const SPECIAL_ELEVENTH_GIFT: &str = "I sent eleven pipers piping";

fn main() {
    for i in 0..12 {
        println!("On the {} day of Christmas, my true love sent to me", DAYS[i]);
        for j in (1..i + 1).rev() {
            if (i == 10) & (j == 10) {
                println!("{}", SPECIAL_ELEVENTH_GIFT);
                continue;
            }
            println!("{}", GIFTS[j]);
        }
        println!("{} {}", if i == 0 { "A" } else { "And a" }, GIFTS[0]);
        println!("");
    }
    println!("And a {}", GIFTS[0]);
}
