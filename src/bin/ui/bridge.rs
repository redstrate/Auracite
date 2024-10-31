#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        type Backend = super::BackendRust;
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "archiveCharacter"]
        fn archive_character(self: &Backend, character_name: &QString, use_dalamud: bool);
    }
}

use std::fs::write;
use cxx_qt_lib::QString;
use auracite::archive_character;

#[derive(Default)]
pub struct BackendRust {
}

impl bridge::Backend {
    pub fn archive_character(&self, character_name: &QString, use_dalamud: bool) {
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();
        
        let inner = rt.block_on(archive_character(&character_name.to_string(), use_dalamud));
        write("/home/josh/test.zip", inner);
    }
}