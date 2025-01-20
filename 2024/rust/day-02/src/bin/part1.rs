#[derive(PartialEq)]
enum Trend {
    Increasing,
    Decreasing,
}

fn part1(input: &str) -> u32 {
    input.trim().lines().fold(0, |num_safe: u32, report| {
        // Split by white-space
        let mut levels = report.split_whitespace();
        let mut prev_level: i32 = levels.next().unwrap().parse().unwrap();

        let mut trend = None;

        // for each character in line, parse then compare with the next character if available
        for level in levels {
            let curr_level: i32 = level.parse().unwrap();
            let lvl_change = curr_level - prev_level;
            match (lvl_change, &trend) {
                (-3..=-1, Some(Trend::Decreasing)) => (),
                (1..=3, Some(Trend::Increasing)) => (),

                // On first Loop check the level change trend
                (-3..=-1, None) => trend = Some(Trend::Decreasing),
                (1..=3, None) => trend = Some(Trend::Increasing),

                // Early return out of report if trend is violated
                _ => return num_safe,
            };
            prev_level = curr_level;
        }

        num_safe + 1
    })
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part1 answer: {}", part1(input));
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn passes_example() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn passes_example_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 6 7 9
8 6 4 4 1
1 3 6 7 9
";
        assert_eq!(part1(input), 3);
    }
}
