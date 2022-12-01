// input: food item per line, newline between elves
// each elf = collection of items represented by their calorie count
// iterate over collection of elves, find the inventory with highest summed calorie count,
// return that count.

fn main() {
    let output = aoc1_1();
    println!("{output}");
}

fn aoc1_1() -> i32 {
    // TODO: Elf file I/O
    // TODO: Better elf data structure

    let mut elves: Vec<Vec<i32>> = vec![];
    #[allow(clippy::vec_init_then_push)]
    {
        elves.push(vec![1000, 2000, 3000]);
        elves.push(vec![4000]);
        elves.push(vec![5000, 6000]);
        elves.push(vec![7000, 8000, 9000]);
        elves.push(vec![10000]);
    }

    // TODO: Elf calculation
    let calorie_sums: Vec<i32> = elves.iter().map(|x| x.iter().sum()).collect();
    return *calorie_sums.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn answer() {
        assert_eq!(aoc1_1(), 24000);
    }
}
