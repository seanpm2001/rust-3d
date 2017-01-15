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

use result::*;
use traits::is_buildable_3d::IsBuildable3D;
use traits::is_mesh_3d::IsMesh3D;

pub trait IsEditableMesh3D<P> : IsMesh3D<P> where
    P: IsBuildable3D {

    fn new() -> Self;

    fn add_vertex(&mut self, vertex: P) -> usize;

    fn add_face(&mut self, v1: P, v2: P, v3: P) -> usize;

    fn try_add_connection(&mut self, vid1: usize, vid2: usize, vid3: usize) -> Result<usize>;
}
