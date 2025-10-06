use azalea::{prelude::*, ClientInformation};
use azalea::swarm::prelude::*;
use rand::thread_rng;
use rand::{seq::IteratorRandom, Rng};

const SERVER_IP: &'static str = "play.araamc.my.id:19237";
const CHARSET: &'static str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890";

#[derive(Default, Clone, Component)]
pub struct State {}

impl State {
    pub fn new() -> Self {
        State {}
    }
}

#[derive(Default, Clone, Resource)]
pub struct SwarmState {}

#[tokio::main]
pub (crate) async fn main() -> anyhow::Result<()> {
    let mut builder = SwarmBuilder::new()
    .set_handler(handle)
    .set_swarm_handler(swarm_handle);

    let mut rng = thread_rng(); 
    for _ in 0..50 {
        let random_user: String = {(0..rng.gen_range(5..=12)).map(|_| CHARSET.chars().choose(&mut rng).unwrap()).collect()};
        let account = Account::offline(&random_user[..]);
        builder = builder.add_account_with_state(account, State::new())
    };

    builder
        .join_delay(tokio::time::Duration::from_millis(50))
        .set_swarm_state(SwarmState {  })
        .start(SERVER_IP)
        .await?;
    Ok(())
}

async fn handle(bot: Client, event: azalea::Event, _state: State) -> anyhow::Result<()> {
    let swarm = bot.resource::<SwarmState>();
    match event {
        azalea::Event::Init => {
            bot.set_client_information(ClientInformation {
                view_distance: 32,
                ..Default::default()
            }).await?;
        },
        // azalea::Event::Tick => {
        //     tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
        //     bot.chat("Nuked by ItzYuuRz");
        // },
        _ => {}
    }
    Ok(())
}

async fn swarm_handle(_swarm: Swarm, swarm_event: SwarmEvent, _swarm_state: SwarmState) -> anyhow::Result<()> {
    match swarm_event {
        SwarmEvent::Disconnect(account, _join_opts) => {
            println!("Bot got kicked: {}", account.username);
        },
        _ => {}
    }
    Ok(())
}
