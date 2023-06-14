

use matrix_sdk::{
    Client, config::SyncSettings,
    room::Room,
    ruma::{user_id, events::room::message::{SyncRoomMessageEvent, OriginalSyncRoomMessageEvent, MessageType, RoomMessageEventContent},
           events::room::member::StrippedRoomMemberEvent
          },
};
use tokio::time::{sleep, Duration};
use dotenv::dotenv;


// Async function that awaits for an invitation and accepts it automatically
async fn on_stripped_state_member(
    room_member: StrippedRoomMemberEvent,
    client: Client,
    room: Room,
) {
    if room_member.state_key != client.user_id().unwrap() {
        return;
    }

    if let Room::Invited(room) = room {
        tokio::spawn(async move {
            println!("Autojoining room {}", room.room_id());
            let mut delay = 2;

            while let Err(err) = room.accept_invitation().await {
                // retry autojoin due to synapse sending invites, before the
                // invited user can join for more information see
                // https://github.com/matrix-org/synapse/issues/4345
                eprintln!("Failed to join room {} ({err:?}), retrying in {delay}s", room.room_id());

                sleep(Duration::from_secs(delay)).await;
                delay *= 2;

                if delay > 3600 {
                    eprintln!("Can't join room {} ({err:?})", room.room_id());
                    break;
                }
            }
            println!("Successfully joined room {}", room.room_id());
        });
    }
}


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let password = std::env::var("PASSWORD").expect("PASSWORD must be set.");

    let bot_user = user_id!("@virto_bot:matrix.org");
    let client = Client::builder().server_name(bot_user.server_name()).build().await?;

    // First we need to log in.
    client.login_username(bot_user, &password).send().await?;

    // We add an event handler that listens if our user is invited to a room
    client.add_event_handler(on_stripped_state_member);

    // We add an event handler that listens if our user is invited to a room
    client.add_event_handler(on_room_message);

    // This event handler listens and prints every message it's received or sent
    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
         println!("Received a message {:?}", ev);
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}

// Async function that gets the text content of a room and answers if it matches. 
// Cómo sabe de que room puede venir el mensaje: hay que pasarselo por parámetros
async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) {
    // First, we need to unpack the message: We only want messages from rooms we are
    // still in and that are regular text messages - ignoring everything else.
    let Room::Joined(room) = room else { return };
    let MessageType::Text(text_content) = event.content.msgtype else { return };

    // here comes the actual "logic": when the bot see's a `!party` in the message,
    // it responds
    if text_content.body.contains("!kusamaupdates") {
        let content = RoomMessageEventContent::text_plain("🎉🎊🥳 Cooming soon 🥳🎊🎉");

        println!("sending");

        // send our message to the room we found the "!party" command in
        // the last parameter is an optional transaction id which we don't
        // care about.
        room.send(content, None).await.unwrap();

        println!("message sent");
    }
}