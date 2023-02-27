// Get sum of elements in a vector
pub fn run(list: &Vec<i64>) -> i64 {
    // get sum of elements
    let sum: i64 = list.iter().sum();
    sum
}

#[cfg(test)]
mod tests {
    // In order to test functions
    use super::*;

    #[test]
    fn test_run() {
        let numbers = vec![10, 20, 30, 40, 50];
        assert_eq!(run(&numbers), 150);
    }
}
