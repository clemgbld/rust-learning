pub fn farenheit_to_celsius(farenheit: f32) -> f32 {
    (farenheit - 32.0) / 1.8
}

pub fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut prev_prev: u32 = 0;
    let mut prev: u32 = 1;

    for _ in 1..n {
        let temp = prev;
        prev = prev + prev_prev;
        prev_prev = temp;
    }

    return prev;
}

pub fn christmas_song() {
    let days = [
        "first day",
        "second day",
        "third day",
        "fourth day",
        "fith day",
        "sixth day",
        "seventh day",
        "eighth day",
        "ninth day",
        "tenth day",
        "eleventh day",
        "twelfth day",
    ];

    let gifts = [
        "a partri-idge in a pear tree",
        "Two turtle doves",
        "Three french hens",
        "Four calling birds",
        "Five gold rings",
        "Six geese-a-laying",
        "Seven swans a-swimming",
        "Eight maids-a-milking",
        "Nine ladies dancing",
        "Ten lords-a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    for i in 0..12 {
        let day = days[i];
        println!("On the {day} of Christmas my true love sent to me,");
        if i == 0 {
            let first_gift = gifts[0];
            println!("{}{}", &first_gift[0..1].to_uppercase(), &first_gift[1..]);
        } else {
            let end_range = i + 1;
            for y in (0..end_range).rev() {
                if y == 0 {
                    println!("and {}", gifts[y]);
                } else {
                    println!("{}", gifts[y]);
                }
            }
        }
    }
}
