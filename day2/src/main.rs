mod intcode_computer;

fn main() {
    let (int1, int2) = intcode_computer::find_inputs_for_result(19690720);
    println!("{}", int1*100+int2);
}
