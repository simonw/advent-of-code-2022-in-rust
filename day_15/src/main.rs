use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    beacon_x: i32,
    beacon_y: i32,
}

// A method
impl Sensor {
    fn distance(&self) -> i32 {
        // Manhattan distance from sensor to beacon
        (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs()
    }
    fn point_cannot_have_beacon(&self, x: i32, y: i32) -> bool {
        // This point cannot have a beacon if it is within distance of sensor
        (x - self.x).abs() + (y - self.y).abs() <= self.distance()
    }
    fn points_around_edge(&self) -> Vec<(i32, i32)> {
        // Find diagonal points on the border around the edge of the shape
        let mut points = Vec::new();
        let mut x = self.x - self.distance();
        let mut y = self.y - self.distance();
        let mut dx = 1;
        let mut dy = 1;
        let mut d = 0;
        let mut i = 0;
        while i < 4 * self.distance() {
            points.push((x, y));
            x += dx;
            y += dy;
            d += 1;
            if d == self.distance() {
                d = 0;
                if dx == 1 {
                    dx = 0;
                    dy = 1;
                } else if dy == 1 {
                    dx = -1;
                    dy = 0;
                } else if dx == -1 {
                    dx = 0;
                    dy = -1;
                } else if dy == -1 {
                    dx = 1;
                    dy = 0;
                }
            }
            i += 1;
        }
        points
    }
}

fn beacon_exists_at(sensors: &Vec<Sensor>, x: i32, y: i32) -> bool {
    for sensor in sensors {
        if sensor.beacon_x == x && sensor.beacon_y == y {
            return true;
        }
    }
    false
}

fn beacon_or_sensor_exists_at(sensors: &Vec<Sensor>, x: i32, y: i32) -> bool {
    for sensor in sensors {
        if (sensor.beacon_x == x && sensor.beacon_y == y) || (sensor.x == x && sensor.y == y) {
            return true;
        }
    }
    false
}

fn main() {
    let file_contents = fs::read_to_string("example.txt").unwrap();
    let mut sensors = Vec::new();
    // Sensor at x=10, y=20: closest beacon is at x=10, y=16
    let re =
        Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)").unwrap();
    for cap in re.captures_iter(&file_contents) {
        let sensor = Sensor {
            x: cap[1].parse().unwrap(),
            y: cap[2].parse().unwrap(),
            beacon_x: cap[3].parse().unwrap(),
            beacon_y: cap[4].parse().unwrap(),
        };
        sensors.push(sensor);
    }
    // println!("{:#?}", sensors);

    // Figure out dimensions of possible space
    // For each sensor, look at how big its range could possibly be,
    // then find min and max x and y based on that
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for sensor in &sensors {
        let distance = sensor.distance();
        if sensor.x - distance < min_x {
            min_x = sensor.x - distance;
        }
        if sensor.x + distance > max_x {
            max_x = sensor.x + distance;
        }
        if sensor.y - distance < min_y {
            min_y = sensor.y - distance;
        }
        if sensor.y + distance > max_y {
            max_y = sensor.y + distance;
        }
    }
    println!(
        "min_x: {}, max_x: {}, min_y: {}, max_y: {}",
        min_x, max_x, min_y, max_y
    );

    // How many points on line y=line_to_search could NOT have a sensor?
    let line_to_search = 10; // 10 or 2000000
    let mut count = 0;
    for x in min_x..=max_x {
        let mut point_cannot_have_beacon = false;
        for sensor in &sensors {
            if sensor.point_cannot_have_beacon(x, line_to_search) {
                point_cannot_have_beacon = true;
                break;
            }
        }
        if point_cannot_have_beacon && !beacon_exists_at(&sensors, x, line_to_search) {
            count += 1;
        }
    }
    println!("Count: {}", count);

    println!("\nPart 2\n======\n");

    // Search the space to find the beacon
    let max_search = 20; // 20 or 4000000

    // I'm just going to search the points that are a single square outside each sensor
    for sensor in &sensors {
        let points = sensor.points_around_edge();
        println!("Sensor: {:?}", sensor);
        println!("Points: {:?}", points);
        continue;
        'outer: for (x, y) in points {
            /*
            if x == 14 && y == 11 {
                println!("Found it! {:?}", (x, y));
                // Check no sensor can see it
                for sensor in &sensors {
                    if sensor.point_cannot_have_beacon(x, y) {
                        println!("Sensor {:?} says it cannot be here", sensor);
                        break;
                    }
                }
            }
            */
            if x < 0 || y < 0 {
                continue;
            }

            for sensor in &sensors {
                if sensor.point_cannot_have_beacon(x, y) {
                    println!("Sensor {:?} says {:?} cannot be here", sensor, (x, y));
                    break 'outer;
                }
            }
            println!("Found it! {:?}", (x, y));
        }
    }
}
