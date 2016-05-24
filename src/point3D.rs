use std::f64;
use std::fmt;
use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};



use traits::{IsMoveable3D, HasPosition2D, HasPosition3D, TransFormableTo2D};
use functions::{sqr_dist3D};

#[derive (PartialEq, PartialOrd)]
pub struct Point3D {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Eq for Point3D {}
impl Ord for Point3D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point3D::new();
        sqr_dist3D(&origin, self).partial_cmp(&sqr_dist3D(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Point3D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
        (self.z as u64).hash(state);
    }
}

impl IsMoveable3D for Point3D {
    fn move_by(&mut self, x: f64, y: f64, z: f64) {
        self.x += x;
        self.y += y;
        self.z += z;
    }
}

impl HasPosition3D for Point3D {
    fn new() -> Box<Self> {
        Box::new(Point3D{x: 0.0, y: 0.0, z: 0.0})
    }

    fn build(x: f64, y: f64, z: f64) -> Box<Self> {
        Box::new(Point3D{x: x, y: y, z: z})
    }


    fn x(&self) -> f64 {
        self.x
    }
    fn y(&self) -> f64 {
        self.y
    }
    fn z(&self) -> f64 {
        self.z
    }

    fn set_x(&mut self, val: f64) {
        self.x = val;
    }

    fn set_y(&mut self, val: f64) {
        self.y = val;
    }

    fn set_z(&mut self, val: f64) {
        self.z = val;
    }

    fn clone(&self) -> Point3D {
        Point3D { x: self.x, y: self.y, z: self.z }
    }
}

impl TransFormableTo2D for Point3D {
    fn transform_to_2D<P>(&self) -> P where P: HasPosition2D {
        *P::build(self.x, self.y)
    }
}

impl fmt::Display for Point3D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}