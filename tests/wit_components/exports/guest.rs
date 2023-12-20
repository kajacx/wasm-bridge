wit_bindgen::generate!({
    path: "../protocol.wit",
    world: "example",
    exports: {
        world: Component,
    }
});

struct Component;

impl Guest for Component {
    fn xy_export(_d: Xy) -> Xy {
        todo!()
    }

    fn xy_export_tuple(_d: (Xy, Xy)) -> (Xy, Xy) {
        todo!()
    }

    fn xy_export_pair(_d: Xy, _e: Xy) -> (Xy, Xy) {
        todo!()
    }

    fn xy_export_list(_d: Vec<Xy>) -> Vec<Xy> {
        todo!()
    }

    fn xyz_export(_d: Xyz) -> Xyz {
        todo!()
    }

    fn xyz_export_tuple(_d: (Xyz, Xyz)) -> (Xyz, Xyz) {
        todo!()
    }

    fn xyz_export_pair(_d: Xyz, _e: Xyz) -> (Xyz, Xyz) {
        todo!()
    }

    fn xyz_export_list(_d: Vec<Xyz>) -> Vec<Xyz> {
        todo!()
    }

    fn void_args_s32() -> i32 {
        todo!()
    }

    fn void_ret_s32(_d: i32) {
        todo!()
    }

    fn void_both() {
        todo!()
    }
}
