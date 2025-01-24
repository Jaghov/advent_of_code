#[derive(Debug, Clone, Copy)]
enum AutoMulta {
    Start,
    M,
    U,
    L,
    Number1,
    Number2,
    Pause,
}
#[derive(Debug, Clone, Copy)]
enum Status {
    Start,
    D,
    O,
    N,
    Apostrophe,
    T,
    Pause,
    Resume,
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let mut left_num = "".to_string();
    let mut right_num = "".to_string();
    let mut state = AutoMulta::Start;
    let mut pause_state = Status::Start;
    input.chars().for_each(|character| {
        match (pause_state, character) {
            (Status::Start, 'd') => pause_state = Status::D,
            (Status::D, 'o') => pause_state = Status::O,
            (Status::O, 'n') => pause_state = Status::N,
            (Status::O, '(') => pause_state = Status::Resume,
            (Status::N, '\'') => pause_state = Status::Apostrophe,
            (Status::Apostrophe, 't') => pause_state = Status::T,
            (Status::T, '(') => pause_state = Status::Pause,
            (Status::Pause, ')') => {
                pause_state = Status::Start;
                state = AutoMulta::Pause
            }
            (Status::Resume, ')') => {
                pause_state = Status::Start;
                state = AutoMulta::Start
            }
            _ => pause_state = Status::Start,
        };

        match (state, character) {
            (AutoMulta::Start, 'm') => state = AutoMulta::M,
            (AutoMulta::M, 'u') => state = AutoMulta::U,
            (AutoMulta::U, 'l') => state = AutoMulta::L,
            (AutoMulta::L, '(') => state = AutoMulta::Number1,
            (AutoMulta::Number1, '0'..='9') => left_num.push(character),
            (AutoMulta::Number1, ',') => state = AutoMulta::Number2,
            (AutoMulta::Number2, '0'..='9') => right_num.push(character),
            (AutoMulta::Number2, ')') => {
                total += left_num.parse::<u32>().unwrap() * right_num.parse::<u32>().unwrap();
                left_num.clear();
                right_num.clear();
                state = AutoMulta::Start;
            }
            (AutoMulta::Pause, _) => {
                left_num.clear();
                right_num.clear();
            }

            _ => {
                left_num.clear();
                right_num.clear();
                state = AutoMulta::Start;
            }
        }
    });
    total
}

fn main() {
    let input = include_str!("../../input.txt");
    println!("part2 answer: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn passes_example() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(input), 161);
    }
}
