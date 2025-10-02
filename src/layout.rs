use blinksy::{
    layout::{Shape3d, Vec3},
    layout3d,
};

layout3d!(
    pub Layout,
    [
        // bottom face
        Shape3d::Grid {
            start: Vec3::new(1., -1., 1.),           // right bottom front
            horizontal_end: Vec3::new(-1., -1., 1.), // left bottom front
            vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        },
        // back face
        Shape3d::Grid {
            start: Vec3::new(-1., -1., -1.),         // left bottom back
            horizontal_end: Vec3::new(-1., 1., -1.), // left top back
            vertical_end: Vec3::new(1., -1., -1.),   // right bottom back
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        },
        // right face
        Shape3d::Grid {
            start: Vec3::new(1., 1., -1.),         // right top back
            horizontal_end: Vec3::new(1., 1., 1.), // right top front
            vertical_end: Vec3::new(1., -1., -1.), // right bottom back
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        },
        // front face
        Shape3d::Grid {
            start: Vec3::new(-1., -1., 1.),         // left bottom front
            horizontal_end: Vec3::new(1., -1., 1.), // right bottom front
            vertical_end: Vec3::new(-1., 1., 1.),   // left top front
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        },
        // left face
        Shape3d::Grid {
            start: Vec3::new(-1., 1., -1.),           // left top back
            horizontal_end: Vec3::new(-1., -1., -1.), // left bottom back
            vertical_end: Vec3::new(-1., 1., 1.),     // left top front
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        },
        // top face
        Shape3d::Grid {
            start: Vec3::new(1., 1., 1.),           // right top front
            horizontal_end: Vec3::new(1., 1., -1.), // right top back
            vertical_end: Vec3::new(-1., 1., 1.),   // left top front
            horizontal_pixel_count: 16,
            vertical_pixel_count: 16,
            serpentine: true,
        }
    ]
);
