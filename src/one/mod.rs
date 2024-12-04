pub fn run() {
    let part1 = process("", false);
    println!("Part1: {}", part1.to_string());
    let part2 = process("", true);
    println!("Part2: {}", part2.to_string());
}

fn process(input: &str, part2: bool) -> u32 {
    return 1;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(1+1, 2);
    }

    #[test]
    fn part2() {
        assert_eq!(1+1,2);
    }
}
