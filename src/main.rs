#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rusqlite;

mod mikrotik;
mod api;
mod router;
mod syslog;

fn main() {
    rocket::ignite().mount("/", routes![api::router::start_controlling, api::router::list_routers, api::router::stop_controlling, api::router::update_router,
        api::backups::force_backup, api::backups::list_backups, api::backups::restore_from_backup,
        api::logs::get_logs,
        api::updates::check_for_updates, api::updates::update]).launch();
}
