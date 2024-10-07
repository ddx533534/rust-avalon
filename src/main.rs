use crate::role::{Assassin, Camp, LoyalOfficial, Merlin, Morgana, Pai, Pawn, Player, Role, RoleImpl, Vote};
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::sync::{Arc, Mutex};

mod role;

fn main() {
    println!("Hello, world!");
    start_game();
}

const COUNT: usize = 8;

// const CAR_LEN: Vec<usize> = vec![
//     3,
//     4,
//     4,
//     5,
//     5,
// ];

lazy_static! {
    static ref ROLES_8: Mutex<Vec<RoleImpl>> = Mutex::new(vec![
        Arc::new(Box::new(LoyalOfficial {})),
        Arc::new(Box::new(LoyalOfficial {})),
        Arc::new(Box::new(LoyalOfficial {})),
        Arc::new(Box::new(Merlin {})),
        Arc::new(Box::new(Pai {})),
        Arc::new(Box::new(Morgana {})),
        Arc::new(Box::new(Assassin {})),
        Arc::new(Box::new(Pawn {})),
    ]);

    static ref CAR_ROUND: Mutex<Vec<usize>> = Mutex::new(vec![
        3,
        4,
        4,
        5,
        5,
    ]);
}
fn start_game() {
    let players = init_players();
    start_vote(players);
}

pub fn init_players() -> Vec<Player> {
    let roles = ROLES_8.lock().unwrap();
    let mut players: Vec<Player> = Vec::with_capacity(COUNT);
    let mut indices: Vec<usize> = (0..COUNT).collect();
    let mut rng = thread_rng();
    indices.shuffle(&mut rng);

    for index in 0..COUNT {
        players.push(Player::new(index, roles.get(indices[index]).unwrap().clone()));
    }

    for (index, player) in players.iter().enumerate() {
        println!("{} - {:?}", index, player.role);
    }

    players
}

pub fn start_vote(players: Vec<Player>) {
    let car_round = CAR_ROUND.lock().unwrap();
    let mut rng = rand::thread_rng();
    let mut car_leader: usize = rng.gen_range(0..COUNT);

    let mut vote_res_role: Vec<(Vote, &Box<dyn Role>)> = vec![];
    let mut vote_res: Vec<Vote> = vec![];
    for (round, round_size) in car_round.iter().enumerate() {
        println!("round: {}, round_size:{}", round, round_size);
        println!("leader:{}-{:?}", car_leader, players[car_leader % COUNT].role);
        for index in 0..*round_size {
            let real_index = (index + car_leader) % COUNT;
            let vote = players[real_index].role.vote_with_round(round as u32);
            vote_res_role.push((vote, players[real_index].role.as_ref()));
            vote_res.push(vote);
        }
        println!("{:?}", vote_res_role);
        println!("===================发车{}===================", check_vote_res(&vote_res));
        vote_res.clear();
        vote_res_role.clear();
        // todo car_leader 只是简单+1
        car_leader += 1;
    }
}


pub fn check_vote_res(vote_res: &Vec<Vote>) -> bool {
    !vote_res.contains(&Vote::Reject)
}