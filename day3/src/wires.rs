fn parse_path(path_str: &str) -> Vec<(&str, isize)> {
    path_str.split(',').map(|x| 
        (
            &x[0..1], 
            x[1..].parse().unwrap()
        )
    ).collect()
}

// probably not the most efficient way to do this
fn make_points(path: Vec<(&str, isize)>) -> Vec<(isize, isize)> {
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

// definately not the most efficient way to do this...
fn find_intersections(patha: Vec<(isize, isize)>, pathb: Vec<(isize, isize)>) -> Vec<(isize, isize)> {
    let mut intersection = Vec::new();
    // there's probably a more efficient way to do this
    for a in patha.iter() {
        for b in pathb.iter() {
            if a == b {
                intersection.push(*a)
            }
        }
    }
    intersection
}

fn distance_from_origin(point: (isize, isize)) -> isize {
    point.0.abs() + point.1.abs()
}

fn find_closest(intersections: Vec<(isize, isize)>) -> (isize, isize) {
    let mut closest = (std::isize::MAX, std::isize::MAX);
    let mut closest_distance = std::isize::MAX;
    for point in intersections.iter() {
        let distance = distance_from_origin(*point);
        if distance < closest_distance {
            closest = *point;
            closest_distance = distance;
        }
    }
    closest
}

pub fn find_distance_to_closest_intersection(patha: &str, pathb: &str) -> isize {
    distance_from_origin(
        find_closest(
            find_intersections(
                make_points(
                    parse_path(patha)
                ), 
                make_points(
                    parse_path(pathb)
                )
            )
        )
    )
}

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

    #[test]
    fn test_intersection() {
        assert_eq!(find_intersections(
            vec![(1, 1)], 
            vec![(2,2), (1,1), (3,3)]
        ), [(1, 1)]);
    }

    #[test]
    fn find_closest_single() {
        assert_eq!(find_closest(vec![(1,1)]), (1, 1));
    }

    #[test]
    fn find_closest_multiple() {
        assert_eq!(find_closest(vec![(2,2), (1,1), (3,3)]), (1, 1));
    }

    #[test]
    fn example_1() {
        assert_eq!(find_distance_to_closest_intersection("R8,U5,L5,D3", "U7,R6,D4,L4"), 6);
    }

    #[test]
    fn other_examples() {
        assert_eq!(find_distance_to_closest_intersection("R75,D30,R83,U83,L12,D49,R71,U7,L72", "U62,R66,U55,R34,D71,R55,D58,R83"), 159);
        assert_eq!(find_distance_to_closest_intersection("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51", "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"), 135);
    }
}