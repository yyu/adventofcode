#![allow(dead_code)]

#[derive(Debug)]
struct ProdIdRange {
    first: i128,
    last: i128,
}

impl std::str::FromStr for ProdIdRange {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<i128> = input
            .split('-')
            .map(|s| s.parse::<i128>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Invalid range {input}: {e}."))?;

        if numbers.len() != 2 {
            return Err(format!("Invalid range: {input}"));
        }

        Ok(ProdIdRange {
            first: numbers[0],
            last: numbers[1],
        })
    }
}

fn count_digits(x: i128) -> u32 {
    if x < 0 {
        return count_digits(-x);
    }

    if x == 0 {
        return 1;
    }

    let mut x = x;
    let mut count = 0;
    while x > 0 {
        count += 1;
        x /= 10;
    }
    count
}

impl ProdIdRange {
    // string manipulation -- more readable but slow
    fn sum_repeat1(&self) -> i128 {
        (self.first..=self.last)
            .map(|x| {
                let s = x.to_string();
                let mid = s.len() / 2;
                if s.len() % 2 == 0 && s[..mid] == s[mid..] {
                    // println!("{:?} {x}", self);
                    x
                } else {
                    0
                }
            })
            .sum()
    }

    // number manipulation -- faster but slightly more complex
    fn sum_repeat2(&self) -> i128 {
        let mut result_sum = 0;
        let mut x = self.first;
        while x <= self.last {
            let digits = count_digits(x);
            if digits % 2 == 0 {
                let divider = 10_i128.pow(digits / 2);
                let upper = x / divider;
                let candidate = upper * divider + upper;
                if self.first <= candidate && candidate <= self.last {
                    result_sum += candidate;
                }
                x = (upper + 1) * divider;
            } else {
                x *= 10;
            }
        }
        result_sum
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = include_str!("input.txt");
    println!("{input}");

    let answer: i128 = input
        .trim()
        .split(',')
        .map(|s| {
            s.parse::<ProdIdRange>()
                .expect("Bad range {s}")
                .sum_repeat2()
        })
        .sum();

    println!("{answer}");

    assert_eq!(38437576669, answer);

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(3), 1);

        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(19), 2);
        assert_eq!(count_digits(31415), 5);

        assert_eq!(count_digits(-1), 1);
        assert_eq!(count_digits(-3), 1);
        assert_eq!(count_digits(-19), 2);
    }
}
