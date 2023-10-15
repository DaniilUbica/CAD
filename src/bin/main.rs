use cstr::cstr;
use qmetaobject::{prelude::*, QUrl};

// The `QObject` custom derive macro allows to expose a class to Qt and QML
#[derive(QObject, Default)]
struct Greeter {
    base: qt_base_class!(trait QObject),
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
        "qrc/main.qml" as "main.qml",
    }
);


fn main() {
    qml_register_type::<Greeter>(cstr!("Greeter"), 1, 0, cstr!("Greeter"));
    let mut engine = QmlEngine::new();

    root_qml();

    engine.load_url(QUrl::from(QString::from("qrc:///main.qml")));
    engine.exec();
}