use wasm_bridge::{
    component::{Component, Linker},
    Config, Engine, Result, Store,
};

wasm_bridge::component::bindgen!({
    path: "../protocol.wit",
    world: "enums",
});

struct HostData;

impl EnumsImports for HostData {
    fn double_shape(&mut self, shape: Shape) -> Result<Shape> {
        Ok(match shape {
            Shape::Circle(r) => Shape::Circle(r * 2.0),
            Shape::Rectangle((w, h)) => Shape::Rectangle((w * 2.0, h * 2.0)),
            Shape::SemiCircle((r, a)) => Shape::SemiCircle((r * 2.0, a)),
            Shape::Point => Shape::Point,
        })
    }

    fn rotate_cw(&mut self, way: Direction) -> Result<Direction> {
        Ok(match way {
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::StayCenter => Direction::StayCenter,
        })
    }

    fn sqrt_import(&mut self, num: Option<f64>) -> Result<Option<f64>> {
        Ok(match num {
            Some(value) if value >= 0.0 => Some(value.sqrt()),
            _ => None,
        })
    }

    fn add_one_both(&mut self, num: Result<i32, u8>) -> Result<Result<i32, u8>> {
        Ok((move || Ok(num? + 1))())
    }

    fn add_one_ok(&mut self, num: Result<i32, ()>) -> Result<Result<i32, ()>> {
        Ok((move || Ok(num? + 1))())
    }

    fn add_one_err(&mut self, num: Result<(), u8>) -> Result<Result<(), u8>> {
        Ok(num)
    }

    fn add_one_none(&mut self, num: Result<(), ()>) -> Result<Result<(), ()>> {
        Ok(num)
    }
}

pub fn run_test(component_bytes: &[u8], _universal_bytes: &[u8]) -> Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);

    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, HostData);

    let component = Component::new(&store.engine(), &component_bytes)?;

    let mut linker = Linker::new(store.engine());
    Enums::add_to_linker(&mut linker, |data| data)?;

    let (instance, _) = Enums::instantiate(&mut store, &component, &linker)?;

    let result = instance.call_quadruple_shape(&mut store, Shape::Circle(2.0))?;
    assert_eq!(
        match result {
            Shape::Circle(radius) => radius,
            _ => unreachable!(),
        },
        8.0
    );

    let result = instance.call_quadruple_shape(&mut store, Shape::Rectangle((2.0, 3.0)))?;
    assert_eq!(
        match result {
            Shape::Rectangle((w, h)) => (w, h),
            _ => unreachable!(),
        },
        (8.0, 12.0)
    );

    let result = instance.call_quadruple_shape(&mut store, Shape::SemiCircle((2.0, 4.0)))?;
    assert_eq!(
        match result {
            Shape::SemiCircle((r, a)) => (r, a),
            _ => unreachable!(),
        },
        (8.0, 4.0)
    );

    let result = instance.call_quadruple_shape(&mut store, Shape::Point)?;
    assert_eq!(
        match result {
            Shape::Point => (),
            _ => unreachable!(),
        },
        ()
    );

    let result = instance.call_rotate_ccw(&mut store, Direction::Up)?;
    assert_eq!(result, Direction::Right);

    let result = instance.call_rotate_ccw(&mut store, Direction::StayCenter)?;
    assert_eq!(result, Direction::StayCenter);

    let result = instance.call_sqrt(&mut store, Some(16.0))?;
    assert_eq!(result, Some(4.0));
    let result = instance.call_sqrt(&mut store, Some(-16.0))?;
    assert_eq!(result, None);
    let result = instance.call_sqrt(&mut store, None)?;
    assert_eq!(result, None);

    let result = instance.call_add_three_both(&mut store, Ok(10))?;
    assert_eq!(result, Ok(13));
    let result = instance.call_add_three_both(&mut store, Err(7))?;
    assert_eq!(result, Err(7));

    let result = instance.call_add_three_ok(&mut store, Ok(10))?;
    assert_eq!(result, Ok(13));
    let result = instance.call_add_three_ok(&mut store, Err(()))?;
    assert_eq!(result, Err(()));

    let result = instance.call_add_three_err(&mut store, Ok(()))?;
    assert_eq!(result, Ok(()));
    let result = instance.call_add_three_err(&mut store, Err(7))?;
    assert_eq!(result, Err(7));

    let result = instance.call_add_three_none(&mut store, Ok(()))?;
    assert_eq!(result, Ok(()));
    let result = instance.call_add_three_none(&mut store, Err(()))?;
    assert_eq!(result, Err(()));

    Ok(())
}
