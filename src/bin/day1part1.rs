use std::{
    io::{self, BufRead},
    ops::Add,
};

use anyhow::{anyhow, Ok};

fn main() -> Result<(), anyhow::Error> {
    let sum = io::stdin().lock().lines().try_fold(0u32, |sum, line| {
        let line = line?;

        if line.is_empty() {
            return Ok(sum);
        }

        let number = line
            .chars()
            .find(char::is_ascii_digit)
            .and_then(|c| c.to_digit(10).map(|n| n * 10))
            .ok_or_else(|| anyhow!("line has no digit: {line}"))?
            .add(
                line.chars()
                    .rfind(char::is_ascii_digit)
                    .and_then(|c| c.to_digit(10))
                    .expect("we got the first digit so the string must have a digit"),
            );

        Ok(sum + number)
    })?;

    println!("Calibration values sum: {sum}");

    Ok(())
}
