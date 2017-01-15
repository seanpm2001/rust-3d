/*
Copyright 2017 Martin Buck
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

extern crate rust_3d;

use rust_3d::traits::is_2d::*;
use rust_3d::traits::is_buildable_2d::*;
use rust_3d::traits::is_moveable_2d::*;
use rust_3d::traits::has_bounding_box_2d::*;
use rust_3d::point_2d::*;
use rust_3d::point_cloud_2d::*;

#[test]
fn point_cloud_2d_test() {

    let mut pc = PointCloud2D::<Point2D>::new();

    assert!(pc.len() == 0);

    let p = *Point2D::build(0.1, 0.2);
    pc.push(p);

    assert!(pc.len() == 1);
    assert!(pc.data[0].x() == 0.1);
    assert!(pc.data[0].y() == 0.2);

    assert!(pc.bounding_box().is_err());

    let p = *Point2D::build(0.2, 0.3);
    pc.push(p);
    assert!(pc.len() == 2);

    assert!(pc.bounding_box().is_ok());

    match pc.bounding_box() {
        Err(_) => assert!(false),
        Ok((bbmin, bbmax)) => {
            assert!(bbmin.x() == 0.1);
            assert!(bbmin.y() == 0.2);
            assert!(bbmax.x() == 0.2);
            assert!(bbmax.y() == 0.3);
        }
    }
    assert!(pc.to_str() == "0.1 0.2\n0.2 0.3\n");

    match PointCloud2D::<Point2D>::parse(pc.to_str()) {
        Err(_) => assert!(false),
        Ok(pcparsed) => assert!(pcparsed.to_str() == "0.1 0.2\n0.2 0.3\n")
    };

    let pccloned = pc.clone();
    assert!(pccloned.to_str() == "0.1 0.2\n0.2 0.3\n");

    pc.move_by(1.0, 2.0);
    println!("pc: {}", pc);
    assert!(pc.to_str() == "1.1 2.2\n1.2 2.3\n");
}
