use crate::models::message_model::Message;
use crate::plugins::chuck_norris::handle_chuck_command;
use rocket::{
    form::Form,
    response::stream::{Event, EventStream},
    tokio::{
        select,
        sync::broadcast::{error::RecvError, Sender},
    },
    Shutdown, State,
};

#[post("/message", data = "<form>")]
pub async fn post(form: Form<Message>, queue: &State<Sender<Message>>) {
    let mut msg: Message = form.into_inner();

    if msg.message.starts_with("/chuck") {
        msg.message = handle_chuck_command(&msg.message).await;
    }

    // Attempt to send the message and handle potential failure.
    if let Err(e) = queue.send(msg) {
        eprintln!("Failed to send message: {:?}", e);
    }
}

#[get("/events")]
pub async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![] {
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
