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
}

fn beacon_exists_at(sensors: &Vec<Sensor>, x: i32, y: i32) -> bool {
    for sensor in sensors {
        if sensor.beacon_x == x && sensor.beacon_y == y {
            return true;
        }
    }
    false
}

fn main() {
    let file_contents = fs::read_to_string("input.txt").unwrap();
    let mut sensors = Vec::new();
    // Sensor at x=10, y=20: closest beacon is at x=10, y=16
    let re = Regex::new(r"Sensor at x=(\d+), y=(\d+): closest beacon is at x=(\d+), y=(\d+)").unwrap();
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
    println!("min_x: {}, max_x: {}, min_y: {}, max_y: {}", min_x, max_x, min_y, max_y);

    // How many points on line y=line_to_search could NOT have a sensor?
    let line_to_search = 2000000;
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
}
