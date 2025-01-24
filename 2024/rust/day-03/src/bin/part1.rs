#[derive(Debug, Clone, Copy)]
enum ParseAutomata {
    Start,
    M,
    U,
    L,
    Number1,
    Number2,
}

fn part1(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut left_num = "".to_string();
    let mut right_num = "".to_string();
    let mut state = ParseAutomata::Start;
    input
        .chars()
        .for_each(|character| match (state, character) {
            (ParseAutomata::Start, 'm') => state = ParseAutomata::M,
            (ParseAutomata::M, 'u') => state = ParseAutomata::U,
            (ParseAutomata::U, 'l') => state = ParseAutomata::L,
            (ParseAutomata::L, '(') => state = ParseAutomata::Number1,
            (ParseAutomata::Number1, '0'..='9') => left_num.push(character),
            (ParseAutomata::Number1, ',') => state = ParseAutomata::Number2,
            (ParseAutomata::Number2, '0'..='9') => right_num.push(character),
            (ParseAutomata::Number2, ')') => {
                total += left_num.parse::<u32>().unwrap() * right_num.parse::<u32>().unwrap();
                left_num.clear();
                right_num.clear();
                state = ParseAutomata::Start;
            }
            _ => {
                left_num.clear();
                right_num.clear();
                state = ParseAutomata::Start;
            }
        });
    total
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
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(input), 161);
    }
}
