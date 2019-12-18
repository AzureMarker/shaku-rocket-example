#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

mod autofac;

use crate::autofac::{ConsoleOutput, IDateWriter, TodayWriter};
use rocket::State;
use shaku::{Container, ContainerBuilder};
use std::sync::Mutex;

#[get("/")]
fn index(container: State<Mutex<Container>>) -> String {
    let mut container = container.lock().unwrap();
    let writer = container
        .with_typed_parameter::<dyn IDateWriter, String>("June 19".to_string())
        .resolve_ref::<dyn IDateWriter>()
        .unwrap();

    writer.write_date();
    writer.get_date()
}

fn main() {
    let mut builder = ContainerBuilder::new();

    builder
        .register_type::<ConsoleOutput>()
        .with_named_parameter("prefix", "PREFIX > ".to_string())
        .with_typed_parameter::<usize>(117 as usize);
    builder.register_type::<TodayWriter>();
    let container = builder.build().unwrap();

    rocket::ignite()
        .manage(Mutex::new(container))
        .mount("/", routes![index])
        .launch();
}
