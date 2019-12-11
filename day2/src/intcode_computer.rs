fn run_intcode(mut ints: Vec<usize>) -> Vec<usize> {
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

const PROGRAM: &str = "1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,1,10,19,1,6,19,23,1,10,23,27,2,27,13,31,1,31,6,35,2,6,35,39,1,39,5,43,1,6,43,47,2,6,47,51,1,51,5,55,2,55,9,59,1,6,59,63,1,9,63,67,1,67,10,71,2,9,71,75,1,6,75,79,1,5,79,83,2,83,10,87,1,87,5,91,1,91,9,95,1,6,95,99,2,99,10,103,1,103,5,107,2,107,6,111,1,111,5,115,1,9,115,119,2,119,10,123,1,6,123,127,2,13,127,131,1,131,6,135,1,135,10,139,1,13,139,143,1,143,13,147,1,5,147,151,1,151,2,155,1,155,5,0,99,2,0,14,0";

fn parse_program(progstr: &str) -> Vec<usize> {
    return progstr.split(',').map(|x| x.parse().unwrap()).collect();
}

fn get_program_with_initial_data(int1: usize, int2: usize) -> Vec<usize> {
    let mut ints: Vec<usize> = parse_program(PROGRAM);
    ints[1] = int1;
    ints[2] = int2;
    return ints;
}

fn get_result(ints: Vec<usize>) -> usize {
    return ints[0];
}

pub fn run_iteration(int1: usize, int2: usize) -> usize {
    get_result(run_intcode(get_program_with_initial_data(int1, int2)))
}

// there's probably a more clever way to do this than brute force, 
// but I'm tired and this runs in < 1 second
pub fn find_inputs_for_result(desired_result: usize) -> (usize, usize) {
    for i in 0..100 {
        for j in 0..100 {
            let result = run_iteration(i, j);
            if result == desired_result {
                return (i, j);
            }
        }
    }
    panic!("np inputs found to produce desired result {}", desired_result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn halt() {
        assert_eq!(run_intcode(vec![99, 0, 0, 0]), [99, 0, 0, 0]);
    }
    #[test]
    fn halt_short() {
        assert_eq!(run_intcode(vec![99]), [99]);
    }

    #[test]
    fn add() {
        assert_eq!(run_intcode(vec![1, 0, 0, 0, 99]), [2, 0, 0, 0, 99]);
    }

    #[test]
    fn multiply() {
        assert_eq!(run_intcode(vec![2, 3, 0, 3, 99]), [2, 3, 0, 6, 99]);
    }

    #[test]
    fn complex() {
        assert_eq!(
            run_intcode(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50]
        );
        assert_eq!(run_intcode(vec![2, 4, 4, 5, 99, 0]), [2, 4, 4, 5, 99, 9801]);
        assert_eq!(
            run_intcode(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            [30, 1, 1, 4, 2, 5, 6, 0, 99]
        );
    }

    #[test]
    fn phase_1() {
        assert_eq!(run_iteration(12, 2), 6327510);
    }

    #[test]
    fn initial_search() {
        assert_eq!(find_inputs_for_result(6327510), (12,2));
    }
}
