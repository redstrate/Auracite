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
        #[cxx_name = "archiveCharacterByName"]
        fn archive_character_by_name(
            self: Pin<&mut Backend>,
            character_name: &QString,
            use_dalamud: bool,
            filename: &QString,
        );

        #[qinvokable]
        #[cxx_name = "archiveCharacterById"]
        fn archive_character_by_id(
            self: Pin<&mut Backend>,
            character_id: &QString,
            use_dalamud: bool,
            filename: &QString,
        );
    }
}

use crate::{archive_character_blocking, search_character_blocking};
use auracite::ArchiveError;
use cxx_kde_frameworks::ki18n::i18n;
use cxx_qt_lib::QString;
use std::pin::Pin;

#[derive(Default)]
pub struct BackendRust {}

impl bridge::Backend {
    pub fn archive_character_by_name(
        mut self: Pin<&mut Self>,
        character_name: &QString,
        use_dalamud: bool,
        filename: &QString,
    ) {
        let Some(id) = search_character_blocking(&character_name.to_string()) else {
            self.archive_failed(&i18n("Character not found"));
            return;
        };

        match archive_character_blocking(id, use_dalamud, &filename.to_string()) {
            Ok(_) => self.archive_successful(),
            Err(err) => {
                match err {
                    // TODO: Pass the URL up
                    ArchiveError::DownloadFailed(_) => {
                        self.archive_failed(&i18n("Download failed"))
                    }
                    ArchiveError::CharacterNotFound => {
                        self.archive_failed(&i18n("Character not found"))
                    }
                    ArchiveError::ParsingError => self.archive_failed(&i18n("Parsing error")),
                    ArchiveError::UnknownError => self.archive_failed(&i18n("Unknown error")),
                    ArchiveError::CouldNotConnectToDalamud => {
                        self.archive_failed(&i18n("Could not connect to Dalamud plugin"))
                    }
                }
            }
        }
    }

    pub fn archive_character_by_id(
        mut self: Pin<&mut Self>,
        character_id: &QString,
        use_dalamud: bool,
        filename: &QString,
    ) {
        let id = character_id.to_string().parse().unwrap();
        match archive_character_blocking(id, use_dalamud, &filename.to_string()) {
            Ok(_) => self.archive_successful(),
            Err(err) => {
                match err {
                    // TODO: Pass the URL up
                    ArchiveError::DownloadFailed(_) => {
                        self.archive_failed(&i18n("Download failed"))
                    }
                    ArchiveError::CharacterNotFound => {
                        self.archive_failed(&i18n("Character not found"))
                    }
                    ArchiveError::ParsingError => self.archive_failed(&i18n("Parsing error")),
                    ArchiveError::UnknownError => self.archive_failed(&i18n("Unknown error")),
                    ArchiveError::CouldNotConnectToDalamud => {
                        self.archive_failed(&i18n("Could not connect to Dalamud plugin"))
                    }
                }
            }
        }
    }
}
