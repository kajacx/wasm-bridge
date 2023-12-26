wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "enums",
    exports: {
        world: MyGuest,
    }
});

struct MyGuest;

impl Guest for MyGuest {
    fn quadruple_shape(shape: Shape) -> Shape {
        let shape = double_shape(shape);
        let shape = double_shape(shape);
        shape
    }

    fn raise_temperature_times(mut temp: Temperature, times: u32) -> Temperature {
        for _ in 0..times {
            temp = raise_temperature(temp);
        }
        temp
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

    fn add_three_both(num: Result<i32, u8>) -> Result<i32, u8> {
        let num = add_one_both(num);
        let num = add_one_both(num);
        let num = add_one_both(num);
        num
    }

    fn add_three_ok(num: Result<i32, ()>) -> Result<i32, ()> {
        let num = add_one_ok(num);
        let num = add_one_ok(num);
        let num = add_one_ok(num);
        num
    }

    fn add_three_err(num: Result<(), u8>) -> Result<(), u8> {
        let num = add_one_err(num);
        let num = add_one_err(num);
        let num = add_one_err(num);
        num
    }

    fn add_three_none(num: Result<(), ()>) -> Result<(), ()> {
        let num = add_one_none(num);
        let num = add_one_none(num);
        let num = add_one_none(num);
        num
    }
}
