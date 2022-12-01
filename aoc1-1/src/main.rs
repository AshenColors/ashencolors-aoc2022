// input: food item per line, newline between elves
// each elf = collection of items represented by their calorie count
// iterate over collection of elves, find the inventory with highest summed calorie count,
// return that count.

fn main() {
    //TODO: File I/O
    let input: Vec<Vec<i32>> = Vec::new();
    let output = aoc1_1(input);
    unimplemented!("{output}");
}

fn aoc1_1(elves: Vec<Vec<i32>>) -> i32 {
    // TODO: Better elf data structure
    // TODO: Elf calculation
    let calorie_sums: Vec<i32> = elves.iter().map(|x| x.iter().sum()).collect();
    return *calorie_sums.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_answer() {
        let elves: Vec<Vec<i32>> = vec![
            vec![1000, 2000, 3000],
            vec![4000],
            vec![5000, 6000],
            vec![7000, 8000, 9000],
            vec![10000],
        ];
        assert_eq!(aoc1_1(elves), 24000);
    }
}
