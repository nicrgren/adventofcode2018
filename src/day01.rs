pub fn solve() -> crate::Result<()> {
    let input = crate::read_input("day01.txt")?;

    println!("Day01 part1: {}", part1(&input));
    println!("Day01 part2: {}", part2(&input));
    Ok(())
}

fn part1(input: &str) -> i32 {
    input
        .trim()
        .lines()
        .map(|s| s.parse::<i32>().expect("parsing delta"))
        .sum()
}

fn part2(input: &str) -> i32 {
    let mut seen = std::collections::HashSet::new();
    seen.insert(0);
    let mut freq = 0;

    loop {
        for n in input
            .trim()
            .lines()
            .map(|s| s.parse::<i32>().expect("parsing delta"))
        {
            freq += n;
            if seen.contains(&freq) {
                return freq;
            }
            seen.insert(freq);
        }
    }
}
