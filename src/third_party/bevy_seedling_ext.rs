#[cfg(feature = "backend")]
pub fn plugin(app: &mut bevy::app::App) {
    if std::env::var("DISABLE_SEEDLING").is_err() {
        #[cfg(feature = "native")]
        use bevy_seedling::SeedlingPlugin;

        #[cfg(feature = "native")]
        app.add_plugins(SeedlingPlugin::default());
    }

    #[cfg(feature = "web")]
    use bevy_seedling::SeedlingPlugin;

    #[cfg(feature = "web")]
    app.add_plugins(SeedlingPlugin::new_web_audio());
}

#[cfg(not(feature = "backend"))]
pub fn plugin(_app: &mut bevy::app::App) {
}
