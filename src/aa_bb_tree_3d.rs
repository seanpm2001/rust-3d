/*
Copyright 2018 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! AABBTree3D, an axis aligned bounding box tree in 3D for fast collision detection

use prelude::*;
use functions::intersection;

use std::marker::PhantomData;

/// AABBTree3D, an axis aligned bounding box tree in 3D for fast collision detection
pub enum AABBTree3D<HB> where
    HB: HasBoundingBox3D + Clone {

    Empty,
    Leaf(AABBTree3DLeaf<HB>),
    Branch(AABBTree3DBranch<HB>)
}

// currently often calculates the bounding box, try cache it
impl<HB> AABBTree3D<HB> where
    HB: HasBoundingBox3D + Clone {

    pub fn new(data: Vec<HB>, maxdepth: usize) -> Result<Self> {
        for x in data.iter() {
            x.bounding_box()?; //ensure bbs are known
        }
        Ok(Self::new_rec(data, maxdepth, 0))
    }

    pub fn for_each_intersection_candidate<'a>(&'a self, line: &Line3D, f: &mut FnMut(&HB)) {
        match self {
            AABBTree3D::Empty          => (),
            AABBTree3D::Leaf(leaf)     => leaf  .for_each_intersection_candidate(line, f),
            AABBTree3D::Branch(branch) => branch.for_each_intersection_candidate(line, f)
        }
    }

    pub fn bb_colliding(&self, bb: &BoundingBox3D) -> Vec<&HB> {
        match self {
            AABBTree3D::Empty          => Vec::new(),
            AABBTree3D::Leaf(leaf)     => leaf.bb_colliding(bb),
            AABBTree3D::Branch(branch) => branch.bb_colliding(bb)
        }
    }

    pub fn bb_crossing_x_value(&self, x: f64) -> Vec<&HB> {
        match self {
            AABBTree3D::Empty          => Vec::new(),
            AABBTree3D::Leaf(leaf)     => leaf.bb_crossing_x_value(x),
            AABBTree3D::Branch(branch) => branch.bb_crossing_x_value(x)
        }
    }

    pub fn bb_crossing_y_value(&self, y: f64) -> Vec<&HB> {
        match self {
            AABBTree3D::Empty          => Vec::new(),
            AABBTree3D::Leaf(leaf)     => leaf.bb_crossing_y_value(y),
            AABBTree3D::Branch(branch) => branch.bb_crossing_y_value(y)
        }
    }

    pub fn bb_crossing_z_value(&self, z: f64) -> Vec<&HB> {
        match self {
            AABBTree3D::Empty          => Vec::new(),
            AABBTree3D::Leaf(leaf)     => leaf.bb_crossing_z_value(z),
            AABBTree3D::Branch(branch) => branch.bb_crossing_z_value(z)
        }
    }

    fn new_rec(data: Vec<HB>, maxdepth: usize, depth: usize) -> Self {
        match data.len() {
            0 => AABBTree3D::Empty,
            1 => {
                let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                AABBTree3D::Leaf(AABBTree3DLeaf::new(data, bb))
            },
            _ => {
                if depth >= maxdepth {
                    let bb = Self::bb_of(&data).unwrap(); //unwrap fine, since data non empty and with valid bbs (see new)
                    AABBTree3D::Leaf(AABBTree3DLeaf::new(data, bb))
                } else {
                    let comp  = match depth % 3 {
                        0 => Compare::X,
                        1 => Compare::Y,
                        _ => Compare::Z
                    };
                    let mut bb = Self::bb_of(&data).unwrap(); //unwrap fine due to early return in new and data not empty
                    let center = bb.center_bb();

                    let dleft  = data.iter().cloned().filter(|x| Self::is_left_of(&comp,  &x.bounding_box().unwrap(), &center)).collect::<Vec<_>>(); //unwrap fine due to early return in new
                    let dright = data.iter().cloned().filter(|x| Self::is_right_of(&comp, &x.bounding_box().unwrap(), &center)).collect::<Vec<_>>(); //unwrap fine due to early return in new

                    if (dleft.len() == dright.len()) && dleft.len() == data.len() {
                        AABBTree3D::Leaf(AABBTree3DLeaf::new(data, bb))
                    } else {
                        let left  = Box::new(Self::new_rec(dleft, maxdepth, depth+1));
                        let right = Box::new(Self::new_rec(dright, maxdepth, depth+1));

                        AABBTree3D::Branch(AABBTree3DBranch::new(left, right, bb))
                    }
                }
            }
        }
    }

    fn is_left_of(comp: &Compare, bb: &BoundingBox3D, center: &Point3D) -> bool {
        match comp {
            Compare::X => bb.min_p().x() < center.x(),
            Compare::Y => bb.min_p().y() < center.y(),
            Compare::Z => bb.min_p().z() < center.z()
        }
    }

    fn is_right_of(comp: &Compare, bb: &BoundingBox3D, center: &Point3D) -> bool {
        match comp {
            Compare::X => bb.max_p().x() >= center.x(),
            Compare::Y => bb.max_p().y() >= center.y(),
            Compare::Z => bb.max_p().z() >= center.z()
        }
    }

    fn bb_of(data: &Vec<HB>) -> Result<BoundingBox3D> {
        if data.len() == 0 {
            return Err(ErrorKind::IndexOutOfBounds) //@todo better type?
        }
        let mut result = data[0].bounding_box().unwrap(); //unwrap fine due to early return in new
        for x in data.iter() {
            result.consume(x.bounding_box().unwrap()); //unwrap fine due to early return in new
        }

        Ok(result)
    }
}

enum Compare { X, Y, Z}

//todo describe
pub struct AABBTree3DLeaf<HB> where
    HB: HasBoundingBox3D {

    data: Vec<HB>,
    bb: BoundingBox3D,
    _marker: PhantomData<HB>
}

impl<HB> AABBTree3DLeaf<HB> where
    HB: HasBoundingBox3D + Clone {

    pub fn new(data: Vec<HB>, bb: BoundingBox3D) -> Self {
        AABBTree3DLeaf{data, bb, _marker: PhantomData}
    }

    pub fn for_each_intersection_candidate<'a>(&'a self, line: &Line3D, f: &mut FnMut(&HB)) {
        if intersection(line, &self.bb).is_none() {
            return;
        }
        for x in self.data.iter() {
            if intersection(line, &x.bounding_box().unwrap()).is_some() { //unwrap fine due to early return in new
                f(x)
            }
        }
    }

    pub fn bb_colliding(&self, bb: &BoundingBox3D) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.collides_with(bb) {
            return result;
        }
        for x in self.data.iter() {
            if x.bounding_box().unwrap().collides_with(bb) { //unwrap fine due to early return in new
                result.push(x)
            }
        }
        result
    }

    pub fn bb_crossing_x_value(&self, x: f64) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.crossing_x_value(x) {
            return result;
        }
        for d in self.data.iter() {
            if d.bounding_box().unwrap().crossing_x_value(x) { //unwrap fine due to early return in new
                result.push(d)
            }
        }
        result
    }

    pub fn bb_crossing_y_value(&self, y: f64) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.crossing_y_value(y) {
            return result;
        }
        for d in self.data.iter() {
            if d.bounding_box().unwrap().crossing_y_value(y) { //unwrap fine due to early return in new
                result.push(d)
            }
        }
        result
    }

    pub fn bb_crossing_z_value(&self, z: f64) -> Vec<&HB> {
        let mut result = Vec::new();
        if !self.bb.crossing_z_value(z) {
            return result;
        }
        for d in self.data.iter() {
            if d.bounding_box().unwrap().crossing_z_value(z) { //unwrap fine due to early return in new
                result.push(d)
            }
        }
        result
    }
}

//todo describe
pub struct AABBTree3DBranch<HB> where
    HB: HasBoundingBox3D + Clone {

    left: Box<AABBTree3D<HB>>,
    right: Box<AABBTree3D<HB>>,
    bb: BoundingBox3D,
    _marker: PhantomData<HB>
}

impl<HB> AABBTree3DBranch<HB> where
    HB: HasBoundingBox3D + Clone {

    pub fn new(left: Box<AABBTree3D<HB>>, right: Box<AABBTree3D<HB>>, bb: BoundingBox3D) -> Self {
        AABBTree3DBranch{left, right, bb, _marker: PhantomData}
    }

    pub fn for_each_intersection_candidate<'a>(&'a self, line: &Line3D, f: &mut FnMut(&HB)) {
        if intersection(line, &self.bb).is_none() {
            return;
        }

        self.left .for_each_intersection_candidate(line, f);
        self.right.for_each_intersection_candidate(line, f);
    }

    pub fn bb_colliding(&self, bb: &BoundingBox3D) -> Vec<&HB> {
        if !self.bb.collides_with(bb) {
            return Vec::new();
        }

        let mut result = self.left.bb_colliding(bb);
        result.append(&mut self.right.bb_colliding(bb));
        result
    }

    pub fn bb_crossing_x_value(&self, x: f64) -> Vec<&HB> {
        if !self.bb.crossing_x_value(x) {
            return Vec::new();
        }

        let mut result = self.left.bb_crossing_x_value(x);
        result.append(&mut self.right.bb_crossing_x_value(x));
        result
    }

    pub fn bb_crossing_y_value(&self, y: f64) -> Vec<&HB> {
        if !self.bb.crossing_y_value(y) {
            return Vec::new();
        }

        let mut result = self.left.bb_crossing_y_value(y);
        result.append(&mut self.right.bb_crossing_y_value(y));
        result
    }

    pub fn bb_crossing_z_value(&self, z: f64) -> Vec<&HB> {
        if !self.bb.crossing_z_value(z) {
            return Vec::new();
        }

        let mut result = self.left.bb_crossing_z_value(z);
        result.append(&mut self.right.bb_crossing_z_value(z));
        result
    }
}
