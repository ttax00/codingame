use std::io;
use chrono::naive::NaiveDate;
use chrono::Datelike;
macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let begin = NaiveDate::parse_from_str(input_line.trim(), "%d.%m.%Y").unwrap();
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let end = NaiveDate::parse_from_str(input_line.trim(), "%d.%m.%Y").unwrap();

    let dur = end.signed_duration_since(begin);
    let days  = dur.num_days();
    let weeks = dur.num_weeks();
    let mut answer = String::from("");

    match days / 365{
        0 => {},
        1 => answer.push_str("1 year, "),
        a => answer.push_str(&format!("{} years, ", a).to_string()),
    }
    // assert_eq!(5745 % 365 / 31, 0);
    let mut month: i32 = end.month() as i32 - begin.month() as i32;
    if month < 0 {
        month = (12 + end.month() - begin.month()) as i32;

    }
    if (end.day() as i32 - begin.day() as i32) < 0 {
        month -= 1;
    }
    match month  {
        12 => {},
        0 => {},
        1 => answer.push_str("1 month, "),
        a => answer.push_str(&format!("{} months, ", a).to_string()),
    }
    if days == 30 {
        println!("total {} days", days);
    } else {
        println!("{}total {} days", answer, days);   
    }
}
