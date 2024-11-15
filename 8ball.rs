use rand::Rng;
use std::io;
fn main() {
    let n = rand::thread_rng().gen_range(1..51);
    let mut question = String::new();
    println!("What is your question?");
    io::stdin().read_line(&mut question).expect("Read line failed.");
    println!("Your question is {}", question);
    if n == 1 {
        print!("Yes, definitely.");
    } else if n == 2 {
        print!("Ask again later.");
    } else if n == 3 {
        print!("Don't count on it.");
    } else if n == 4 {
        print!("Outlook not so good.");
    } else if n == 5 {
        print!("Yes, in due time.");
    } else if n == 6 {
        print!("Very doubtful.");
    } else if n == 7 {
        print!("Absolutely!");
    } else if n == 8 {
        print!("My sources say no.");
    } else if n == 9 {
        print!("Yes, for sure!");
    } else if n == 10 {
        print!("Can't predict now.");
    } else if n == 11 {
        print!("It is certain.");
    } else if n == 12 {
        print!("Reply hazy, try again.");
    } else if n == 13 {
        print!("Yes, definitely so.");
    } else if n == 14{
        print!("As I see it, yes.");
    } else if n == 15 {
        print!("No way!");
    } else if n == 16 {
        print!("You may rely on it.");
    } else if n == 17 {
        print!("Future looks promising.");
    } else if n == 18 {
        print!("Most likely.");
    } else if n == 19 {
        print!("No, not at all.");
    } else if n == 20 {
        print!("Yes, but be cautious.");
    } else if n == 21 {
        print!("The signs point to yes.");
    } else if n == 22 {
        print!("Very unlikely.");
    } else if n == 23 {
        print!("Yes, go for it!");
    } else if n == 24 {
        print!("It’s unclear, ask again.");
    } else if n == 25 {
        print!("Yes, but with conditions.");
    } else if n == 26 {
        print!("My answer is no.");
    } else if n == 27 {
        print!("Definitely not!");
    } else if n == 28 {
        print!("The outlook is good");
    } else if n == 29 {
        print!("Yes, indeed!");
    } else if n == 30 {
        print!("Ask later for clarity.");
    } else if n == 31 {
        print!("Highly unlikely.");
    } else if n == 32 {
        print!("Yes, trust your instincts.");
    } else if n == 33 {
        print!("The answer is unclear.");
    } else if n == 34 {
        print!("No, try again.");
    } else if n == 35 {
        print!("Yes, the time is right.");
    } else if n == 36 {
        print!("Outlook is uncertain.");
    } else if n == 37 {
        print!("Yes, but be patient.");
    } else if n == 38 {
        print!("Don't hold your breath.");
    } else if n == 39 {
        print!("Yes, it’s a good idea!");
    } else if n == 40 {
        print!("Not in this lifetime.");
    } else if n == 41 {
        print!("Yes, believe in it!");
    } else if n == 42 {
        print!("The odds are against it.");
    } else if n == 43 {
        print!("Definitely yes!");
    } else if n == 44 {
        print!("It is not looking good.");
    } else if n == 45 {
        print!("Yes, with a twist.");
    } else if n == 46 {
        print!("My answer is yes!");
    } else if n == 47 {
        print!("Unlikely, but possible.");
    } else if n == 48 {
        print!("Yes, but tread carefully.");
    } else if n == 49 {
        print!("No, that’s a bad idea.");
    } else if n == 50 {
        print!("Yes, but consider the risks.");
    }
}