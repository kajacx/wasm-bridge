wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "enums",
});

struct Plugin;

impl Enums for Plugin {
    fn quadruple_shape(shape: Shape) -> Shape {
        let shape = double_shape(shape);
        let shape = double_shape(shape);
        shape
    }

    fn rotate_ccw(way: Direction) -> Direction {
        let way = rotate_cw(way);
        let way = rotate_cw(way);
        let way = rotate_cw(way);
        way
    }

    fn sqrt(num: Option<f64>) -> Option<f64> {
        sqrt_import(num)
    }
}

export_enums!(Plugin);
