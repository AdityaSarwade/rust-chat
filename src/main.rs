mod api;
mod jokes;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::chat::health_check;
use models::message_model::Message;
use rocket::{
    form::Form,
    fs::{relative, FileServer},
    response::stream::{Event, EventStream},
    tokio::{
        select,
        sync::broadcast::{channel, error::RecvError, Sender},
    },
    Shutdown, State,
};

#[post("/message", data = "<form>")]
async fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let mut msg: Message = form.into_inner();

    async fn handle_chuck_command(command: &str) -> String {
        match command {
            "/chuck help" => jokes::get_help(),
            "/chuck" => jokes::get_random_joke().await,
            _ if command.starts_with("/chuck @") => {
                let parts: Vec<&str> = command.split_whitespace().collect();
                if parts.len() == 2 {
                    let name = parts[1].trim_start_matches('@');
                    jokes::get_random_joke_from_name(name.to_string()).await
                } else if parts.len() > 3 && parts[2] == "cat" {
                    let name = parts[1].trim_start_matches('@');
                    let categories: String = parts[3..].join(",");
                    jokes::get_random_joke_from_name_and_categories(name.to_string(), categories)
                        .await
                } else {
                    "Invalid command format. Use /chuck help for the list of commands.".to_string()
                }
            }
            _ if command.starts_with("/chuck cat") => {
                let categories: String = command
                    .trim_start_matches("/chuck cat")
                    .split_whitespace()
                    .collect();
                if categories.is_empty() {
                    jokes::get_categories().await
                } else {
                    jokes::get_random_joke_from_categories(categories).await
                }
            }
            _ => "Invalid command format. Use /chuck help for the list of commands.".to_string(),
        }
    }

    if msg.message.starts_with("/chuck") {
        msg.message = handle_chuck_command(&msg.message).await;
    }

    // Attempt to send the message and handle potential failure.
    if let Err(e) = queue.send(msg) {
        eprintln!("Failed to send message: {:?}", e);
    }
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();

    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg{
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };
            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(channel::<Message>(1024).0)
        .mount("/", routes![health_check, post, events])
        .mount("/", FileServer::from(relative!("static")))
}
