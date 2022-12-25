fn snafu_to_decimal(snafu: &str) -> i64 {
    let mut decimal = 0;
    let mut place_value = 1;

    for c in snafu.chars().rev() {
        match c {
            '2' => decimal += 2 * place_value,
            '1' => decimal += 1 * place_value,
            '0' => (),
            '-' => decimal -= 1 * place_value,
            '=' => decimal -= 2 * place_value,
            _ => {}
        }

        place_value *= 5;
    }

    decimal
}

fn decimal_to_snafu(decimal: i64) -> String {
    let mut num = decimal;
    let mut result = String::new();

    while num > 0 {
        let digit = match num % 5 {
            0 => '0',
            1 => {
                num -= 1;
                '1'
            }
            2 => {
                num -= 2;
                '2'
            }
            3 => {
                num -= -2;
                '='
            }
            4 => {
                num -= -1;
                '-'
            }
            _ => panic!("No way"),
        };
        result.push(digit);
        num /= 5;
    }

    result.chars().rev().collect::<String>()
}

fn parse_input(input: &str) -> Vec<i64> {
    input.lines().map(| line | snafu_to_decimal(line)).collect()
}

pub fn solution_1(input: &str) -> String {
    let number = parse_input(input).iter().sum::<i64>();
    
    decimal_to_snafu(number)
}

pub fn solution_2(_input: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_DATA: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn solution_1_test() {
        assert_eq!(solution_1(TEST_DATA), "2=-1=0");
    }

    #[test]
    #[ignore]
    fn solution_2_test() {
        assert_eq!(solution_2(TEST_DATA), 54);
    }
}
