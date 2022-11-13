use bevy::prelude::*;
use shared::*;
use protocol::Command;
use nats_lite::nats;
use rand::{thread_rng, Rng};
pub fn new_ball(mut commands: ResMut<protocol::Commands>){
    info!("sending welcome");
    let n = nats::proto::ClientOp::Sub{
        subject:String::from("welcome"),
        queue_group:None,
        sid:16,
    };
    commands.push(Command::Nats(String::from("default"),n));
    let mut rng = thread_rng();
    let n: u32 = rng.gen_range(100000..999999);
    let tv = ClientMessage::Welcome{
        game_id:String::from("hello"),
        ball_id:BallId(n),
        ball_label:BallLabel(n.to_string(),String::from("")),
    };
    let tv_= rmp_serde::to_vec(&tv).unwrap();
    let n = nats::proto::ClientOp::Pub{
        subject: String::from("client_handler.hello"),
        reply_to: None,
        payload: tv_,
    };
    commands.push(protocol::Command::Nats(String::from("default"),n));

}