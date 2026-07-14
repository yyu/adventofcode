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

impl ProdIdRange {
    fn sum_repeat(&self) -> i128 {
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
                .sum_repeat()
        })
        .sum();

    println!("{answer}");

    assert_eq!(38437576669, answer);

    Ok(())
}
