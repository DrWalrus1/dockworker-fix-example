#[macro_use]
extern crate rocket;
use std::process;

use dockworker::{container::ContainerFilters, Docker};
use rocket::get;

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![test])
}

#[get("/")]
async fn test() -> String {
    get_containers().await
}

async fn get_containers() -> String {
    let docker: Docker = match Docker::connect_with_defaults() {
        Ok(docker) => docker,
        Err(e) => {
            println!("Error connecting to Docker: {}", e);
            process::exit(1);
        }
    };

    let containers = docker
        .list_containers(None, None, None, ContainerFilters::default())
        .await
        .unwrap();
    containers[0].Names[0].clone()
}
