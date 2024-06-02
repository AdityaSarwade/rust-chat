#[macro_use]
extern crate rocket;

use rocket::{
    form::Form,
    fs::{relative, FileServer},
    response::stream::{Event, EventStream},
    serde::{Deserialize, Serialize},
    tokio::{
        select,
        sync::broadcast::{channel, error::RecvError, Sender},
    },
    Shutdown, State,
};
mod jokes;

#[get("/health-check")]
fn health_check() -> &'static str {
    "Server is Online."
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Message {
    #[field(validate=len(..30))]
    pub room: String,
    #[field(validate=len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
async fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    // A send fails if there are no active subscribers. This is ok.
    let mut msg: Message = form.into_inner();
    if msg.message.starts_with("/chuck") {
        if msg.message.starts_with("/chuck help") {
            msg.message = jokes::get_help();
        } else if msg.message.starts_with("/chuck cat") {
            msg.message = jokes::get_categories().await;
        } else {
            msg.message = jokes::get_random_joke().await;
        }
        let _res = queue.send(msg);
    } else {
        let _res = queue.send(msg);
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
