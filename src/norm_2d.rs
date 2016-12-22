/*
Copyright 2016 Martin Buck
This file is part of rust-3d.
rust-3d is free software: you can redistribute it and/or modify
it under the terms of the GNU Lesser General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.
rust-3d is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU Lesser General Public License for more details.
You should have received a copy of the GNU Lesser General Public License
along with rust-3d.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::cmp::{Eq, Ordering};
use std::hash::{Hash, Hasher};

use point_2d::Point2D;
use traits::is_2d::Is2D;
use traits::is_normalized_2d::IsNormalized2D;
use traits::is_buildable_2d::IsBuildable2D;
use functions::{sqr_dist_2d};

#[derive (PartialEq, PartialOrd)]
pub struct Norm2D {
    pub x: f64,
    pub y: f64
}

impl Eq for Norm2D {}

impl Ord for Norm2D {
    fn cmp(&self, other: &Self) -> Ordering {
        let origin = *Point2D::new();
        sqr_dist_2d(&origin, self).partial_cmp(&sqr_dist_2d(&origin, other)).unwrap_or(Ordering::Equal)
    }
}

impl Hash for Norm2D { //@todo poor precision this way
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as u64).hash(state);
        (self.y as u64).hash(state);
    }
}

impl Is2D for Norm2D {
    fn x(&self) -> f64 {
        self.x
    }

    fn y(&self) -> f64 {
        self.y
    }

    fn clone(&self) -> Self {
        Norm2D {
            x: self.x,
            y: self.y
        }
    }
}

impl IsNormalized2D for Norm2D {
    fn new<P>(p: P) -> Option<Box<Self>> where
        P: Is2D {

        match p.abs() {
            0.0 => None,
            l => Some(Box::new(Norm2D {
                x: p.x() / l,
                y: p.y() / l
            }))
        }
    }

    fn norm_x() -> Self {
        Norm2D {
            x: 1.0,
            y: 0.0
        }
    }

    fn norm_y() -> Self {
        Norm2D {
            x: 0.0,
            y: 1.0
        }
    }
}
