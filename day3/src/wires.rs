fn parse_path(path_str: &str) -> Vec<(&str, usize)> {
    path_str.split(',').map(|x| 
        (
            &x[0..1], 
            x[1..].parse().unwrap()
        )
    ).collect()
}

fn make_points(path: Vec<(&str, usize)>) -> Vec<(usize, usize)> {
    let mut points = vec![];
    let mut last_point = (0, 0);
    for command in path.iter() {
        for _ in 0..command.1 {
            let point = match command.0 {
                "U" => (last_point.0, last_point.1 + 1),
                "D" => (last_point.0, last_point.1 - 1),
                "L" => (last_point.0 - 1, last_point.1),
                "R" => (last_point.0 + 1, last_point.1),
                _ => panic!("Unexpected command {}{}", command.0, command.1)
            };
            points.push(point);
            last_point = point
        }

    }
    points
}

fn find_intersections(patha: Vec<(usize, usize)>, pathb: Vec<(usize, usize)>) -> Vec<(usize, usize) {
    let mut intersection = Vec::new();
    // there's probably a more efficient way to do this
    for a in patha.iter() {
        for b in pathb.iter() {
            if a == b {
                intersection.push(a)
            }
        }
    }
    intersection
}

fn find closest

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple() {
        assert_eq!(parse_path("U1"), [("U", 1)]);
    }
    
    #[test]
    fn make_points_simple() {
        assert_eq!(make_points(vec![("U", 1)]), [(0, 1)]);
    }
}