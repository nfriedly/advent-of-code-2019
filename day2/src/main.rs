fn main() {
    println!("{}", run_intcode("1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,1,10,23,27,2,27,13,31,1,31,6,35,2,6,35,39,1,39,5,43,1,6,43,47,2,6,47,51,1,51,5,55,2,55,9,59,1,6,59,63,1,9,63,67,1,67,10,71,2,9,71,75,1,6,75,79,1,5,79,83,2,83,10,87,1,87,5,91,1,91,9,95,1,6,95,99,2,99,10,103,1,103,5,107,2,107,6,111,1,111,5,115,1,9,115,119,2,119,10,123,1,6,123,127,2,13,127,131,1,131,6,135,1,135,10,139,1,13,139,143,1,143,13,147,1,5,147,151,1,151,2,155,1,155,5,0,99,2,0,14,0")[0]);
}

// returns position 0 upon halting
fn run_intcode(string: &str) -> Vec<usize> {
    let mut ints: Vec<usize> = string.split(',').map(|x| x.parse().unwrap()).collect();

    for n in 0..(ints.len() / 4) {
        let i = n * 4;
        let opcode = ints[i];
        match opcode {
            1 | 2 => {
                let num1 = ints[ints[i + 1]];
                let num2 = ints[ints[i + 2]];
                let target = ints[i + 3];
                match opcode {
                    1 => ints[target] = num1 + num2,
                    2 => ints[target] = num1 * num2,
                    _ => panic!("inconcievable!"),
                }
            }

            99 => break,
            _ => panic!("invalid opcode at index {} with value {}", i, opcode),
        }
    }

    return ints;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halt() {
        assert_eq!(run_intcode("99,0,0,0"), [99, 0, 0, 0]);
    }
    #[test]
    fn halt_short() {
        assert_eq!(run_intcode("99"), [99]);
    }

    #[test]
    fn add() {
        assert_eq!(run_intcode("1,0,0,0,99"), [2, 0, 0, 0, 99]);
    }

    #[test]
    fn multiply() {
        assert_eq!(run_intcode("2,3,0,3,99"), [2, 3, 0, 6, 99]);
    }

    #[test]
    fn complex() {
        assert_eq!(
            run_intcode("1,9,10,3,2,3,11,0,99,30,40,50"),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run_intcode("2,4,4,5,99,0"), [2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            run_intcode("1,1,1,4,99,5,6,0,99"),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }
}
