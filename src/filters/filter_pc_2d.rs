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

//! FilterPC2D, a filter which can transform any IsFilter2D into an IsFilterPC2D

use std::collections::HashSet;

use point_cloud_2d::*;
use view::*;
use traits::is_2d::*;
use traits::is_filter_2d::*;
use traits::is_filter_pc_2d::*;

//@todo untested
//@todo concave hull, convex hull

/// FilterPC2D, a filter which can transform any IsFilter2D into an IsFilterPC2D
pub struct FilterPC2D<F> where
    F: IsFilter2D {

    filter_2d: Box<F>
}

impl<F> FilterPC2D<F> where
    F: IsFilter2D {

    pub fn build(filter_2d: F) -> Self {
        FilterPC2D {filter_2d: Box::new(filter_2d)}
    }
}

impl<P,F> IsFilterPC2D<P> for FilterPC2D<F> where
    P: Is2D,
    F: IsFilter2D {

    fn filter(&self, pc: &PointCloud2D<P>, view: &mut View) {
        if pc.len() == 0 {
            *view = View::Full;
            return;
        }
        match view {
            &mut View::Full => {
                let mut indices = HashSet::new();
                for (i, p) in pc.data.iter().enumerate() {
                    let ref tmp = **p; //@todo get rid of this
                    if self.filter_2d.is_allowed(tmp) {
                        indices.insert(i);
                    }
                }
                *view = View::Restricted(indices);
            }
            &mut View::Restricted(ref mut indices) => {
                let mut indices_to_remove = Vec::<usize>::new();
                let max = pc.len() - 1;
                for index in indices.iter() {
                    if *index > max {
                        indices_to_remove.push(*index);
                        continue;
                    }
                    let ref p = *pc.data[*index];
                    if !self.filter_2d.is_allowed(p) {
                        indices_to_remove.push(*index);
                    }
                }

                for index_to_remove in indices_to_remove {
                    indices.remove(&index_to_remove);
                }
            }
        }
    }
}