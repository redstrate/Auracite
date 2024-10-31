use std::env::args;
use std::fs::write;
use cxx_kde_frameworks::kcoreaddons::{KAboutData, License};
use cxx_kde_frameworks::ki18n::{i18n, i18nc, KLocalizedContext, KLocalizedString};
use cxx_qt_lib::{QByteArray, QGuiApplication, QList, QQmlApplicationEngine, QQuickStyle, QString, QStringList, QUrl};
use cxx_qt_lib_extras::{QCommandLineOption, QCommandLineParser};
use auracite::archive_character;

pub mod bridge;

fn archive_character_blocking(character_name: &String, use_dalamud: bool) {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    let inner = rt.block_on(archive_character(&character_name.to_string(), use_dalamud)).unwrap();
    write("/home/josh/test.zip", inner);
}

fn main() {
    QQuickStyle::set_style(&QString::from("org.kde.desktop"));

    let mut app = QGuiApplication::new();

    KLocalizedString::set_application_domain(&QByteArray::from("auracite"));

    let mut about_data = KAboutData::from(
        QString::from("auracite"),
        i18nc("@title", "Auracite"),
        QString::from(option_env!("CARGO_PKG_VERSION").unwrap()),
        i18nc("@title", "Export your FFXIV character in portable, generic formats."),
        License::GPL_V3,
    );

    KAboutData::set_application_data(about_data.as_ref().unwrap());

    let Some(mut about_data) = about_data.as_mut() else {
        return;
    };

    let mut command_line_parser = QCommandLineParser::default();
    about_data.as_mut().setup_command_line(&mut command_line_parser);

    let mut name_option = QCommandLineOption::from(&QString::from("name"));
    name_option.set_description(&i18n("The character's name."));
    name_option.set_value_name(&QString::from("name"));
    command_line_parser.add_option(&name_option);

    let mut dalamud_option = QCommandLineOption::from(&QString::from("dalamud"));
    dalamud_option.set_description(&i18n("Whether to import more data from the Auracite Dalamud plugin."));
    command_line_parser.add_option(&dalamud_option);

    command_line_parser.process(&QStringList::from(&QList::from(&args().map(|x| QString::from(x)).collect::<Vec<QString>>())));
    about_data.as_mut().process_command_line(&mut command_line_parser);

    if command_line_parser.is_set(&QString::from("name")) {
        let character_name = command_line_parser.value(&QString::from("name")).to_string();

        println!("Downloading character data for {}...", character_name);

        archive_character_blocking(&character_name, command_line_parser.is_set(&QString::from("dalamud")));

        return;
    }

    let mut engine = QQmlApplicationEngine::new();

    if let Some(mut engine) = engine.as_mut() {
        KLocalizedContext::initialize_engine(engine.as_mut().as_qqmlengine());
        // TODO: replace with loadModule (requires cxx-qt changes)
        engine.load(&QUrl::from("qrc:/qt/qml/zone/xiv/auracite/src/bin/auracite/Main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}