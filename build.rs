#[cfg(feature = "ui")]
use cxx_qt_build::{CxxQtBuilder, QmlModule};

fn main() {
    #[cfg(feature = "ui")]
    CxxQtBuilder::new()
        .qt_module("Quick")
        .qml_module(QmlModule {
            uri: "zone.xiv.auracite",
            rust_files: &["src/bin/ui/bridge.rs"],
            qml_files: &["src/bin/ui/Main.qml"],
            ..Default::default()
        })
        .build();
}