extern crate CAD;

use CAD::painter::*;

use cstr::cstr;
use qmetaobject::prelude::*;
use qmetaobject::QUrl;

#[derive(QObject, Default)]
struct Greeter {
    base: qt_base_class!(trait QObject),
    points_amount: qt_property!(i32; NOTIFY points_amount_changed),
    points_amount_changed: qt_signal!(),
    name: qt_property!(QString; NOTIFY name_changed),
    name_changed: qt_signal!(),
    compute_greetings: qt_method!(fn compute_greetings(&self, verb: String, name: String) -> QString {
        format!("{}, {}", verb, name).into()
    }),
    print_hello: qt_method!(fn print_hello(&self, name: String) {
        println!("Hello, {}", name);
    }),
}

qrc!(root_qml,
    "" {
        "qrc/main/main.qml" as "main.qml",
    }
);

fn main() {
    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    let mut engine = QmlEngine::new();

    root_qml();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));
    engine.exec();

    // let coordinates = vec![(400, 400), (100, 400), (100, 200), (200, 200), (0, 100), (0, 0), (400, 0), (400, 200), (300, 300)];
    // let connections = vec![(0, 1), (1, 2), (2, 3), (2, 4), (4, 5), (5, 6), (6, 7), (7, 8)];
    // draw_figure(&coordinates, &connections);
}