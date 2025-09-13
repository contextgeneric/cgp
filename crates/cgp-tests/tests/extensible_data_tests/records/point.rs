use cgp::extra::field::impls::CanBuildWithDefault;
use cgp::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point2d {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point3d {
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, Clone, Eq, PartialEq, CgpData)]
struct Point4d {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

#[test]
pub fn test_point_cast() {
    let point_2d = Point2d { x: 1, y: 2 };
    let point_3d = Point3d::build_with_default(point_2d.clone());
    let point_4d = Point4d::build_with_default(point_2d.clone());

    assert_eq!(point_3d, Point3d { x: 1, y: 2, z: 0 });

    assert_eq!(
        point_4d,
        Point4d {
            x: 1,
            y: 2,
            z: 0,
            w: 0,
        }
    );
}
