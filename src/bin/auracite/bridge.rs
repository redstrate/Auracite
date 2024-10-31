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

use crate::archive_character_blocking;
use cxx_qt_lib::QString;

#[derive(Default)]
pub struct BackendRust {
}

impl bridge::Backend {
    pub fn archive_character(&self, character_name: &QString, use_dalamud: bool) {
        archive_character_blocking(&character_name.to_string(), use_dalamud);
    }
}