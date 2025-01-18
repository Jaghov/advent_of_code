fn part1(input: &str) -> u32 {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    // O(2N) N is the number of lines
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    });
    // 2*O(Nlog(k)) where k is the number of distinct entries
    left.sort_unstable();
    right.sort_unstable();

    let output: u32 = left
        .into_iter()
        .zip(right.into_iter())
        .fold(0, |acc, (left, right)| acc + (left).abs_diff(right));

    output
}
fn main() {
    let input = include_str!("../../part1.txt");
    let output = part1(input);
    print!("{:#?}", output);
}

#[cfg(test)]
mod tests {
    use crate::part1;

    #[test]
    fn example_works() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part1(input), 11 as u32);
    }
}
