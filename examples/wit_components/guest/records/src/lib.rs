wit_bindgen::generate!({
    path: "../../records.wit",
    world: "records",
});

struct Guest;

impl Records for Guest {
    fn run(args: Vec<u32>) {
        send_item(Item { a: 1, b: 2 });
        // send_items(&[Item { a: 2, b: 3 }, Item { a: 4, b: 5 }]);
    }
}

export_records!(Guest);
