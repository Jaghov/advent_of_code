use std::collections::HashMap;

/// Approaches to solving this problem
/// 1. Eager memoized approach
/// - Split the list into left and right
/// - Create an empty hash map
/// - For each number in the left list
///   - If number is already in the hash table continue
///   - else count all occurences of the number in the right list O(N)
///
/// worst case Time Complexity O(N^2): where there are no duplicates in
/// the left table
///
/// 2. Simpler approach
/// - Convert right list into a HashMap Key=value doable in N*T:
///   Where T is the time for insert or checking an entry (should be armortised O(1))
/// - iterate over the left list, checking each entry in the hashmap, and perform the mult O(N*T)
///
/// WCTC: 2N*T -> O(N)
///

pub fn split_list(input: &str) -> (Vec<u32>, Vec<u32>) {
    let mut left: Vec<u32> = Vec::new();
    let mut right: Vec<u32> = Vec::new();

    // O(2N) N is the number of lines
    input.lines().for_each(|line| {
        let mut parts = line.split_whitespace();
        left.push(parts.next().unwrap().parse().unwrap());
        right.push(parts.next().unwrap().parse().unwrap());
    });
    (left, right)
}

fn part2(input: &str) -> u32 {
    let (left, right) = split_list(input);
    let counts = right.into_iter().fold(HashMap::new(), |mut acc, elem| {
        *acc.entry(elem).or_insert(0 as u32) += 1;
        acc
    });

    left.into_iter().fold(0, |similarity, num| {
        similarity + num * counts.get(&num).unwrap_or(&0)
    })
}

fn main() {
    let input = include_str!("../../part1.txt");
    let out = part2(input);

    println!("Solution to part 2 = {}", out)
}

#[cfg(test)]
mod tests {
    use crate::part2;

    #[test]
    fn example_works() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3
";
        assert_eq!(part2(input), 31 as u32);
    }
}
