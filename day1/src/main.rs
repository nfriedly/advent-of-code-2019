use std::cmp;

fn main() {
    println!("Hello, world! {}", fuel_required(7));
    println!("total fuel needed for mass + fuel: {}", sum_fuel())
}

fn fuel_required(mass: usize) -> usize {
    let fuel: isize = (mass as f64 / 3.0).floor() as isize - 2;
    return cmp::max(fuel, 0) as usize;
}

fn fuel_required_including_fuel_mass(mass: usize) -> usize {
    let mut fuel = fuel_required(mass);
    let mut fuel_fuel = fuel;
    loop {
        fuel_fuel = fuel_required(fuel_fuel);
        if fuel_fuel == 0 {
            return fuel;
        }
        fuel += fuel_fuel;
    }
}

const WEIGHTS: &str = "113481
140620
123826
86474
71091
126880
103784
140154
124024
54281
80810
109441
68828
144207
99151
136876
99398
138555
118619
133215
139302
137780
136649
83358
63027
75067
73974
90158
94691
86847
61466
81184
86043
119923
116576
131380
102136
143364
124421
123141
138131
73274
84598
61410
67240
136186
63878
135804
73599
84526
116178
114587
58606
79162
124031
120329
61270
89887
54859
67618
96669
56796
55725
96105
68833
52417
72249
53930
139995
86217
131618
137145
54944
76456
82141
69754
102656
57461
108747
79510
105715
98046
116903
139339
127451
135374
88468
69524
76112
110928
99160
137229
121433
65951
56267
117209
61358
73659
69633
149274";

fn sum_fuel() -> usize {
    let mut fuel: usize = 0;
    for line in WEIGHTS.lines() {
        let weight: usize = line.parse().unwrap();
        let line_fuel = fuel_required_including_fuel_mass(weight);
        println!("weight: {}, fuel: {}", weight, line_fuel);
        fuel += line_fuel;
    }
    return fuel;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(fuel_required(12), 2);
        assert_eq!(fuel_required(14), 2);
        assert_eq!(fuel_required(1969), 654);
        assert_eq!(fuel_required(100756), 33583);
    }

    #[test]
    fn no_negative() {
        assert_eq!(fuel_required(2), 0);
    }

    #[test]
    fn fuel_fuel() {
        assert_eq!(fuel_required_including_fuel_mass(14), 2);
        assert_eq!(fuel_required_including_fuel_mass(1969), 966);
        assert_eq!(fuel_required_including_fuel_mass(100756), 50346);
    }
}
