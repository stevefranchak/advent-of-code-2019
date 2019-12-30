use std::io;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn manhattan_distance_from_origin(&self) -> u32 {
        (self.x.abs() + self.y.abs()) as u32
    } 
}

#[derive(Debug, PartialEq)]
enum LineKind {
    Horizontal,
    Vertical
}

#[derive(Debug)]
struct Line {
    endpoints: (Point, Point),
    kind: LineKind
}

enum LineExtentKind {
    Domain,
    Range
}

impl Line {
    fn is_in(&self, line_extent_kind: LineExtentKind, cmp_line: &Line) -> bool {
        let (min_point, max_point) = if cmp_line.endpoints.0 < cmp_line.endpoints.1 {
            (cmp_line.endpoints.0, cmp_line.endpoints.1)
        } else {
            (cmp_line.endpoints.1, cmp_line.endpoints.0)
        };
        match &line_extent_kind {
            LineExtentKind::Domain => min_point.x <= self.endpoints.0.x && max_point.x >= self.endpoints.0.x,
            LineExtentKind::Range =>  min_point.y <= self.endpoints.0.y && max_point.y >= self.endpoints.0.y
        }
    }

    fn intersects_with(&self, cmp_line: &Line) -> Option<Point> {
        // If line kind is the same, then the lines are parallel
        if self.kind == cmp_line.kind {
            return None;
        }

        let (horizontal_line, vertical_line) = match &self.kind {
            LineKind::Horizontal => (self, cmp_line),
            LineKind::Vertical => (cmp_line, self)
        };
        if !vertical_line.is_in(LineExtentKind::Domain, &horizontal_line) || !horizontal_line.is_in(LineExtentKind::Range, &vertical_line) {
            return None;
        }

        Some(Point{x: vertical_line.endpoints.0.x, y: horizontal_line.endpoints.0.y})
    }
}

#[derive(Debug)]
struct Wire {
    lines: Vec<Line>
}

impl Wire {
    fn new(lines: Vec<Line>) -> Wire {
        Wire {lines: lines}
    }

    fn new_from_path(path: &String) -> Wire {
        let mut lines = vec![];
        let path_tokens = path.split(',')
            .map(|t| (
                &t[0..1], 
                String::from(&t[1..]).parse::<u16>().expect("could not parse token steps as u16")
            ));
        let mut last_point = Point{x: 0, y: 0};
        for (direction, steps) in path_tokens {
            let point = match direction {
                "D" => Point{x: last_point.x, y: last_point.y - steps as i32},
                "U" => Point{x: last_point.x, y: last_point.y + steps as i32},
                "L" => Point{x: last_point.x - steps as i32, y: last_point.y},
                "R" => Point{x: last_point.x + steps as i32, y: last_point.y},
                _ => panic!("Unsupported path direction")
            };
            let line_kind = if last_point.x == point.x {LineKind::Vertical} else {LineKind::Horizontal}; 
            lines.push(Line{
                endpoints: (last_point, point),
                kind: line_kind
            });
            last_point = point;
        }
        Wire::new(lines)
    }

    fn new_from_stdin_line() -> Wire {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).expect("could not read line from stdin");
        Wire::new_from_path(&String::from(input_line.trim()))
    }

    fn get_intersection_points_with(&self, cmp_wire: &Wire) -> Vec<Point> {
        let mut intersection_points: Vec<Point> = vec![];
        let origin_point = Point{x: 0, y: 0};

        for line_a in &self.lines {
            for line_b in &cmp_wire.lines {
                let intersection_point = line_a.intersects_with(line_b);
                match intersection_point {
                    Some(intersection_point) => if intersection_point != origin_point {intersection_points.push(intersection_point)} else {},
                    None => continue
                }
            }
        }

        intersection_points
    }
}

fn main() -> io::Result<()> {
    let wire_a = Wire::new_from_stdin_line();
    let wire_b = Wire::new_from_stdin_line();
    let intersection_points = wire_a.get_intersection_points_with(&wire_b);
    
    let min_distance = intersection_points.into_iter()
        .map(|p| p.manhattan_distance_from_origin())
        .fold(u32::max_value(), |acc, x| if acc < x {acc} else {x});
    println!("Minimum Distance: {}", min_distance);

    Ok(())
}
