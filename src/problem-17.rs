fn w_and<'a>(n: u32) -> &'a str {
    match n {
        0 => "",
        _ => " and "
    }
}

fn lexi(n: u32, zero: bool) -> String {
    let w_20 = ["", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten", "eleven", "twelve", "thriteen", "fourteen", "fifteen", "sixteen", "seventeen", "eighteen", "nineteen", "twenty"];
    let w_100 = ["", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety"];
    match n {
        0 if zero => "zero".to_string(),
        0..21 => w_20[n as usize].to_string(),
        21..100 => w_100[(n / 10) as usize].to_string() + lexi(n%10, false).as_str(),
        100..1000 => lexi(n/100, false) + " hundred" + w_and(n % 100) + lexi(n%100, false).as_str(),
        _ => "one thousand".to_string(),
    }
}

fn main() {
    (1..=1000).for_each(|n| println!("{0}: {1}", n, lexi(n, true)));
    let r:usize = (1..=1000).map(|n| lexi(n, true).chars().filter(|ch| ch.is_alphabetic()).count()).sum();
    println!("result is: {0}", r);
}
