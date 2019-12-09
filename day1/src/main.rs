fn main() {
    println!("Hello, world! {}", fuel_required(7));
    println!("total fuel needed: {}", sum_fuel());
}

fn fuel_required(mass: u64) -> u64 {
    return (mass as f64 / 3.0).floor() as u64 - 2;
}

const weights: &str = "113481
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

fn sum_fuel() -> u64 {
    let mut fuel: u64 = 0;
    for line in weights.lines() {
        let weight:u64 = line.parse().unwrap();
        let lineFuel = fuel_required(weight);
        println!("weight: {}, fuel: {}", weight, lineFuel);
        fuel += lineFuel;
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
}
