use std::io::{self, BufRead};

use anyhow::{anyhow, Ok};

fn main() -> Result<(), anyhow::Error> {
    let sum = io::stdin().lock().lines().try_fold(0, |sum, line| {
        const MAX_RED_CUBES: u8 = 12;
        const MAX_GREEN_CUBES: u8 = 13;
        const MAX_BLUE_CUBES: u8 = 14;

        let line = line?;

        if line.is_empty() {
            return Ok(sum);
        }

        let (id, sets) = line
            .strip_prefix("Game")
            .ok_or_else(|| anyhow!("expected `Game` prefix"))?
            .trim_start()
            .split_once(':')
            .ok_or_else(|| anyhow!("expected `:`"))
            .and_then(|(id, sets)| Ok((id.parse::<u16>()?, sets.split(';'))))?;

        for cubes in sets {
            for cube in cubes.split(',') {
                let (qty, color) = cube
                    .trim_start()
                    .split_once(' ')
                    .ok_or_else(|| anyhow!("expected ` `"))
                    .and_then(|(qty, color)| Ok((qty.parse::<u8>()?, color)))?;

                if matches!(color, "red" if qty > MAX_RED_CUBES)
                    | matches!(color, "green" if qty > MAX_GREEN_CUBES)
                    | matches!(color, "blue" if qty > MAX_BLUE_CUBES)
                {
                    return Ok(sum);
                }
            }
        }

        Ok(sum + id)
    })?;

    println!("ID sum: {sum}");

    Ok(())
}
