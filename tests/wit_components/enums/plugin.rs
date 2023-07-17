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

    // fn add_three_both(num: Result<i32, u8>) -> Result<i32, u8> {
    //     let num = add_one_both(num);
    //     let num = add_one_both(num);
    //     let num = add_one_both(num);
    //     num
    // }
    fn add_three_both(num: i32) -> Result<i32, u8> {
        if num < 0 {
            Err(0)
        } else {
            Ok(num + 3)
        }
    }
}

export_enums!(Plugin);
