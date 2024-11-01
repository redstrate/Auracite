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

        #[qsignal]
        #[cxx_name = "archiveSuccessful"]
        fn archive_successful(self: Pin<&mut Backend>);

        #[qsignal]
        #[cxx_name = "archiveFailed"]
        fn archive_failed(self: Pin<&mut Backend>, message: &QString);
    }

    unsafe extern "RustQt" {
        #[qinvokable]
        #[cxx_name = "archiveCharacter"]
        fn archive_character(self: Pin<&mut Backend>, character_name: &QString, use_dalamud: bool);
    }
}

use std::pin::Pin;
use cxx_kde_frameworks::ki18n::i18n;
use crate::archive_character_blocking;
use cxx_qt_lib::QString;
use auracite::ArchiveError;

#[derive(Default)]
pub struct BackendRust {
}

impl bridge::Backend {
    pub fn archive_character(mut self: Pin<&mut Self>, character_name: &QString, use_dalamud: bool) {
        match archive_character_blocking(&character_name.to_string(), use_dalamud) {
            Ok(_) => { self.archive_successful() }
            Err(err) => { 
                match err {
                    // TODO: Pass the URL up
                    ArchiveError::DownloadFailed(_) => { self.archive_failed(&i18n("Download failed")) }
                    ArchiveError::CharacterNotFound => { self.archive_failed(&i18n("Character not found")) }
                    ArchiveError::ParsingError => { self.archive_failed(&i18n("Parsing error")) }
                    ArchiveError::UnknownError => { self.archive_failed(&i18n("Unknown error")) }
                }
            }
        }
    }
}