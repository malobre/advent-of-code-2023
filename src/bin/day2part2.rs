use std::io::{self, BufRead};

use anyhow::{anyhow, Ok};

fn main() -> Result<(), anyhow::Error> {
    let sum = io::stdin().lock().lines().try_fold(0, |sum, line| {
        let line = line?;

        if line.is_empty() {
            return Ok(sum);
        }

        let sets = line
            .split_once(':')
            .ok_or_else(|| anyhow!("expected `:`"))
            .and_then(|(_, sets)| Ok(sets.split(';')))?;

        let mut red_cubes_count = 0;
        let mut green_cubes_count = 0;
        let mut blue_cubes_count = 0;

        for cubes in sets {
            for cube in cubes.split(',') {
                let (qty, color) = cube
                    .trim_start()
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("expected ` `"))
                    .and_then(|(qty, color)| Ok((qty.parse::<u32>()?, color)))?;

                match color {
                    "red" if qty > red_cubes_count => {
                        red_cubes_count = qty;
                    }
                    "blue" if qty > blue_cubes_count => {
                        blue_cubes_count = qty;
                    }
                    "green" if qty > green_cubes_count => {
                        green_cubes_count = qty;
                    }
                    _ => {}
                }
            }
        }

        let power = red_cubes_count * green_cubes_count * blue_cubes_count;

        Ok(sum + power)
    })?;

    println!("ID sum: {sum}");

    Ok(())
}
