use geom::{Rect, Point};

use clap::Parser;

use std::path::PathBuf;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Parser, Debug)]
struct Args {
    gtfs_dir: PathBuf
}

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = Args::parse();

    let mut shape_path = args.gtfs_dir.clone();
    shape_path.push("shapes.txt");
    eprintln!("parsing {shape_path:?}");

    let file = File::open(shape_path)?;
    let reader = BufReader::new(file);

    fn point_from_line(line: String) -> Point {
        let mut fields =  line.split(",").skip(2);
        let lat: f64 = {
            let lat_str = fields.next().expect("missing lat");
            lat_str.parse().expect(&format!("invalid float: {lat_str}"))
        };
        let lon: f64 = {
            let lon_str = fields.next().expect("missing lon");
            lon_str.parse().expect(&format!("invalid float: {lon_str}"))
        };
        Point::new(lon, lat)
    }

    // skip header
    let mut lines = reader.lines().skip(1);

    let p1 = point_from_line(lines.next().expect("at least one point in the file")?);
    let p2 = point_from_line(lines.next().expect("at least two points in the file")?);
    let mut bbox = Rect::new(p1, p2);

    let mut num_points = 2;
    for line in lines {
        let line = line?;
        let point = point_from_line(line);
        num_points += 1;
        bbox.expand(point);
    }

    eprintln!("completed bbox calculation of {num_points} points");
    println!("{}", bbox.osm_bbox_fmt());
    Ok(())
}


mod geom {
    #[derive(Debug, Clone)]
    pub struct Point {
        x: f64, y: f64
    }

    impl Point {
        pub fn new(x: f64, y: f64) -> Self {
            Self { x, y }
        }

        pub fn x(&self) -> f64 {
            self.x
        }

        pub fn y(&self) -> f64 {
            self.y
        }
    }

    #[derive(Debug, Clone)]
    pub struct Rect { min: Point, max: Point }

    impl Rect {
        pub fn new(a: Point, b: Point) -> Self {
            let min_x = a.x().min(b.x());
            let max_x = a.x().max(b.x());
            let min_y = a.y().min(b.y());
            let max_y = a.y().max(b.y());

            let min = Point::new(min_x, min_y);
            let max = Point::new(max_x, max_y);

            Self { min, max }
        }

        pub fn expand(&mut self, point: Point) {
            if point.x() > self.max.x() {
                self.max.x = point.x();
            } else if point.x() < self.min.x() {
                self.min.x = point.x();
            }

            if point.y() > self.max.y() {
                self.max.y = point.y();
            } else if point.y() < self.min.y() {
                self.min.y = point.y();
            }
        }

        pub fn osm_bbox_fmt(&self) -> String {
            let left = self.min.x();
            let bottom = self.min.y();
            let right = self.max.x();
            let top = self.max.y();

            format!("{left},{bottom},{right},{top}")
        }
    }
}
