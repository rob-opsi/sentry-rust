#[macro_use]
extern crate log;
extern crate pretty_env_logger;
#[macro_use]
extern crate sentry;

fn main() {
    let _sentry = sentry::init((
        "https://a94ae32be2584e0bbd7a4cbb95971fee@sentry.io/1041156",
        sentry::ClientOptions {
            release: sentry_crate_release!(),
            ..Default::default()
        },
    ));

    let mut log_builder = pretty_env_logger::formatted_builder().unwrap();
    log_builder.parse("info");
    sentry::integrations::log::init(Some(Box::new(log_builder.build())), Default::default());
    sentry::integrations::panic::register_panic_handler();

    info!("System is booting");
    warn!("System is warning");
    error!("Holy shit everything is on fire!");
}
