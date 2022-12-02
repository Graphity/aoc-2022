fn main() {
    let input = include_str!("input.txt");
    let lines = input.split("\n\n");
    let mut lines: Vec<u32> = lines
        .map(|line| {
            line.split("\n")
                .flat_map(|num: &str| num.parse::<u32>())
                .sum()
        })
        .collect();

    lines.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {:?}", lines[0]);
    println!("Part 2: {:?}", lines.iter().take(3).sum::<u32>());
}
