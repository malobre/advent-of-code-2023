use std::io::{self, BufRead};

use anyhow::{bail, Ok};

fn main() -> Result<(), anyhow::Error> {
    let sum = io::stdin().lock().lines().try_fold(0, |sum, line| {
        let line = line?;

        if line.is_empty() {
            return Ok(sum);
        }

        let mut index = 0usize;

        let mut first = None;
        while first.is_none() {
            let buffer = &line[index..];

            first = buffer
                .starts_with(|c: char| c.is_ascii_digit())
                .then_some(buffer.chars().next().expect("starts with a digit"))
                .or_else(|| buffer.starts_with("one").then_some('1'))
                .or_else(|| buffer.starts_with("two").then_some('2'))
                .or_else(|| buffer.starts_with("three").then_some('3'))
                .or_else(|| buffer.starts_with("four").then_some('4'))
                .or_else(|| buffer.starts_with("five").then_some('5'))
                .or_else(|| buffer.starts_with("six").then_some('6'))
                .or_else(|| buffer.starts_with("seven").then_some('7'))
                .or_else(|| buffer.starts_with("eight").then_some('8'))
                .or_else(|| buffer.starts_with("nine").then_some('9'));

            if index > line.len() {
                bail!("line has no digit: {line}");
            }

            index += 1;
        }

        let mut index = line.len();
        let mut last = None;
        while last.is_none() {
            let buffer = &line[..index];

            last = buffer
                .ends_with(|c: char| c.is_ascii_digit())
                .then_some(buffer.chars().last().expect("ends_with returned true"))
                .or_else(|| buffer.ends_with("one").then_some('1'))
                .or_else(|| buffer.ends_with("two").then_some('2'))
                .or_else(|| buffer.ends_with("three").then_some('3'))
                .or_else(|| buffer.ends_with("four").then_some('4'))
                .or_else(|| buffer.ends_with("five").then_some('5'))
                .or_else(|| buffer.ends_with("six").then_some('6'))
                .or_else(|| buffer.ends_with("seven").then_some('7'))
                .or_else(|| buffer.ends_with("eight").then_some('8'))
                .or_else(|| buffer.ends_with("nine").then_some('9'));

            index -= 1;
        }

        let number = format!("{}{}", first.unwrap(), last.unwrap()).parse::<u8>()?;

        Ok(sum + u16::from(number))
    })?;

    println!("Calibration values sum: {sum}");

    Ok(())
}
