#[derive(PartialEq)]
enum Trend {
    Increasing,
    Decreasing,
}

fn is_safe_report(report: &Vec<i32>, ignore_idx: Option<i32>) -> bool {
    // Minor optimisation: no point checking reports where we're ignoring a negative index
    if ignore_idx.is_some_and(|ignore| ignore < 0) {
        return false;
    }
    let mut levels = report.clone().into_iter();
    let mut prev_level: i32 = levels.next().unwrap();

    let mut trend = None;
    let mut idx = 0;

    if ignore_idx.is_some_and(|ignore| ignore == idx) {
        prev_level = levels.next().unwrap();
        idx += 1;
    }
    idx += 1;
    // for each character in line, parse then compare with the next character if available
    for level in levels {
        if ignore_idx.is_some_and(|ignore| ignore == idx) {
            idx += 1;
            continue;
        }
        let curr_level: i32 = level;
        let lvl_change = curr_level - prev_level;

        match (lvl_change, &trend, ignore_idx) {
            (-3..=-1, Some(Trend::Decreasing), _) | (1..=3, Some(Trend::Increasing), _) => (),

            // On first Loop check the level change trend
            (-3..=-1, None, _) => trend = Some(Trend::Decreasing),
            (1..=3, None, _) => trend = Some(Trend::Increasing),

            // Early return out of report if trend is violated
            (_, _, None) => {
                for offset in 0..3 {
                    if is_safe_report(report, Some(idx - offset)) {
                        return true;
                    }
                }
                return false;
            }
            _ => return false,
        };
        prev_level = curr_level;
        idx += 1;
    }
    true
}

fn part1(input: &str) -> u32 {
    input.trim().lines().fold(0, |num_safe: u32, report| {
        // Split by white-space
        let levels = report
            .split_whitespace()
            .map(|num| num.parse().unwrap())
            .collect::<Vec<i32>>();

        num_safe + is_safe_report(&levels, None) as u32
    })
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2 answer: {}", part1(input));
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
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn passes_example_2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
9 7 6 2 1
8 6 4 4 1
1 3 6 7 9";
        assert_eq!(part1(input), 3);
    }
}
