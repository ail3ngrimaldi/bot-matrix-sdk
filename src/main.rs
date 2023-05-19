use matrix_sdk::{
    Client, config::SyncSettings,
    room::Room,
    ruma::{user_id, events::room::message::SyncRoomMessageEvent,
           events::room::member::StrippedRoomMemberEvent, room_id, RoomId
          },
};
use tokio::time::{sleep, Duration};
use dotenv::dotenv;

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

    //Intentar pasar este parámetro como password
    let bot_user = user_id!("@virto_bot:matrix.org");
    let client = Client::builder().server_name(bot_user.server_name()).build().await?;

    // First we need to log in.
    client.login_username(bot_user, &password).send().await?;

    // let default = String::from("Unexisting room");
    // let room_id = RoomId::parse(std::env::var("ROOM_ID").unwrap_or(default));
    // let room = client.join_room_by_id(room_id!("!oqWOHFNSqdBaqVabYS:matrix.org")).await?;


    // We add an event handler that listens if our user is invited to a room
    client.add_event_handler(on_stripped_state_member);


    client.add_event_handler(|ev: SyncRoomMessageEvent| async move {
      //  No entiendo como poder acceder al body del message desde el código
    //    if ev.content.msgtype.body == "palta" {
    //        room.send_message("The word 'palta' is a type of fruit that is native to South America.");
    //    }

        println!("Received a message {:?}", ev);
    });

    // Syncing is important to synchronize the client state with the server.
    // This method will never return unless there is an error.
    client.sync(SyncSettings::default()).await?;

    Ok(())
}