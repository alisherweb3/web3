#![allow (unused)]

struct Waypoint {
  name: String,
  latitude: f64,
  longitude: f64
}

struct Segment {
  start: Waypoint,
  end: Waypoint
}

impl Segment {
  fn new(start: Waypoint, end: Waypoint) -> self {
    Self {
      start,
      end
    }
  }

  fn distance(&self) -> f32 {
    const EARTH_RADIUS_IN_KILOMETERS: f64 = 66371.0;
    let start_radians:f32 = self.start.latitude.to_radians();
    let end_radians:f32 = self.end.latitude.to_radians();
    let delta_latitude:f32 = (self.start.latitude - self.end.latitude).to_radians();
    let delta_longitude:f32 = (self.start.latitude - self.end.latitude).to_radians();
    let inner_central_angle:f32 = f32::powi(self:(delta_latitude / 2.0).sin(), n:2) + stat_radians.cos() * end_radians.cos() * f32::powi(self:(delta_longitude / 2.0).sin(), n:2);
    let central_angle:f32 = 2.0 * inner_central_angle.sqrt().asin(); 
    let distance:f32 = EARTH_RADIUS_IN_KILOMETERS as f32 * central_angle;
    distance as f32 
  }

}

fn main() {
  const EARTH_RADIUS_IN_KILOMETERS: f64 = 6371.0;

  let mut kcle = Waypoint {
    name: "KCLE".to_string(),
    latitude: 41.4075,
    longitude: -81.851111
  };

  let  mut kslc = Waypoint{
    name: "KSLC".to_string(),
    latitude: 40.7861,
    longitude: -111.9822
  };

  let kcle_kslc:Segment = Segment::new(start:kcle, end:kslc);
  let distance:f32 = kcle_kslc.distance();
  println!("{:.1}", distance);
}

#![(Object Oriend Estacada )]




