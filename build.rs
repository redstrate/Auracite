use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    if std::env::var("CARGO_CFG_TARGET_FAMILY").unwrap() != "wasm" {
        CxxQtBuilder::new()
            .qrc("src/bin/auracite/resources.qrc")
            .qt_module("Quick")
            .qml_module(QmlModule {
                uri: "zone.xiv.auracite",
                rust_files: &["src/bin/auracite/bridge.rs"],
                qml_files: &["src/bin/auracite/Main.qml"],
                ..Default::default()
            })
            .build();
    }
}
