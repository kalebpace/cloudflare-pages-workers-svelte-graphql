pub fn recurse_fibonacci(n: u32) -> u32 {
    match n { 
        0 => 1,
        1 | 2 => 1,
        _ => recurse_fibonacci(n - 1) + recurse_fibonacci(n - 2),
    }
}

pub fn factorial(n: u32) -> u32 {
    let result = (1..=n).rev().reduce(|acc, x| acc * x);
    match result {
        Some(result) => result,
        None => 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_recurse_fibonacci() {
        let expected = 144;
        let actual = recurse_fibonacci(12);
        assert_eq!(expected, actual)
    }

    #[test]
    fn test_factorial() {
        let expected = 3628800;
        let actual = factorial(10);
        assert_eq!(expected, actual)
    }
}