use std::io::{self, BufRead};

use anyhow::{bail, Ok};

fn main() -> Result<(), anyhow::Error> {
    let sum = io::stdin().lock().lines().try_fold(0, |sum, line| {
        let line = line?;

        if line.is_empty() {
            return Ok(sum);
        }

        let mut index = 0usize;
        let first;
        loop {
            let buffer = &line[index..];

            if let Some(digit) = buffer
                .chars()
                .next()
                .and_then(|c| c.to_digit(10))
                .or_else(|| buffer.starts_with("one").then_some(1))
                .or_else(|| buffer.starts_with("two").then_some(2))
                .or_else(|| buffer.starts_with("three").then_some(3))
                .or_else(|| buffer.starts_with("four").then_some(4))
                .or_else(|| buffer.starts_with("five").then_some(5))
                .or_else(|| buffer.starts_with("six").then_some(6))
                .or_else(|| buffer.starts_with("seven").then_some(7))
                .or_else(|| buffer.starts_with("eight").then_some(8))
                .or_else(|| buffer.starts_with("nine").then_some(9))
            {
                first = digit;
                break;
            }

            index += 1;

            if index >= line.len() {
                bail!("line has no digit: {line}");
            }
        }

        let mut index = line.len();
        let last;
        loop {
            let buffer = &line[..index];

            if let Some(digit) = buffer
                .chars()
                .last()
                .and_then(|c| c.to_digit(10))
                .or_else(|| buffer.ends_with("one").then_some(1))
                .or_else(|| buffer.ends_with("two").then_some(2))
                .or_else(|| buffer.ends_with("three").then_some(3))
                .or_else(|| buffer.ends_with("four").then_some(4))
                .or_else(|| buffer.ends_with("five").then_some(5))
                .or_else(|| buffer.ends_with("six").then_some(6))
                .or_else(|| buffer.ends_with("seven").then_some(7))
                .or_else(|| buffer.ends_with("eight").then_some(8))
                .or_else(|| buffer.ends_with("nine").then_some(9))
            {
                last = digit;
                break;
            }

            index -= 1;
        }

        let number = first * 10 + last;

        Ok(sum + number)
    })?;

    println!("Calibration values sum: {sum}");

    Ok(())
}
