use cxx_kde_frameworks::ki18n::{KLocalizedContext, KLocalizedString};
use cxx_qt_lib::{QByteArray, QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};

#[cfg(feature = "ui")]
pub mod bridge;

fn main() {
    QQuickStyle::set_style(&QString::from("org.kde.desktop"));

    let mut app = QGuiApplication::new();
    let mut engine = QQmlApplicationEngine::new();

    KLocalizedString::set_application_domain(&QByteArray::from("mgedit"));

    if let Some(mut engine) = engine.as_mut() {
        KLocalizedContext::initialize_engine(engine.as_mut().as_qqmlengine());
        // TODO: replace with loadModule (requires cxx-qt changes)
        engine.load(&QUrl::from("qrc:/qt/qml/zone/xiv/auracite/src/bin/ui/Main.qml"));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}