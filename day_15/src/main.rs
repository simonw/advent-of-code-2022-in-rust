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

fn main() {
    let file_contents = fs::read_to_string("example.txt").unwrap();
    let mut sensors = Vec::new();
    // Sensor at x=10, y=20: closest beacon is at x=10, y=16
    let min_x = -100;
    let max_x = 100;
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
    // How many points on line y=10 could NOT have a sensor?
    let mut count = 0;
    for x in min_x..=max_x {
        let mut point_cannot_have_beacon = false;
        for sensor in &sensors {
            if sensor.point_cannot_have_beacon(x, 10) {
                point_cannot_have_beacon = true;
                break;
            }
        }
        if point_cannot_have_beacon {
            count += 1;
        }
    }
    println!("Count: {}", count);
}
