use crate::role::Vote::Reject;
use crate::role::{Assassin, LoyalOfficial, Merlin, Morgana, Pai, Pawn, Player, Role, RoleImpl, Vote};
use lazy_static::lazy_static;
use rand::prelude::SliceRandom;
use rand::{thread_rng, Rng};
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Mutex;

mod role;
mod info;

fn main() {
    // start_game();
    bench_mark();
}


pub fn bench_mark() {
    let mut average_win = 0;
    for _i in 0..TEST_COUNT {
        let mut win_count = 0;
        for _j in 0..GAME_COUNT {
            let res = start_game();
            win_count += if res { 1 } else { 0 }
        }
        average_win += win_count;
        // println!("当前好人胜率: {}", win_count as f64 * 1.0 / GAME_COUNT as f64);
    }
    println!("{} 次测试中，好人平均胜率: {}", TEST_COUNT, average_win as f64 * 1.0 / (GAME_COUNT * TEST_COUNT) as f64);
}
const TEST_COUNT: usize = 100;
const GAME_COUNT: usize = 100;
const ROUND_COUNT: usize = 5;
const PLAYER_COUNT: usize = 8;

const PROPOSAL_MAX_COUNT: usize = 3;

lazy_static! {
    static ref CAR_ROUND: Mutex<Vec<usize>> = Mutex::new(vec![
        3,
        4,
        4,
        5,
        5,
    ]);
}
fn start_game() -> bool {
    let players = init_players();
    start_vote(players)
}

pub fn init_players() -> Vec<Player> {
    let roles: Vec<RoleImpl> = vec![
        Rc::new(RefCell::new(Box::new(LoyalOfficial::default(PLAYER_COUNT)))),
        Rc::new(RefCell::new(Box::new(LoyalOfficial::default(PLAYER_COUNT)))),
        Rc::new(RefCell::new(Box::new(LoyalOfficial::default(PLAYER_COUNT)))),
        Rc::new(RefCell::new(Box::new(Pai::default(PLAYER_COUNT)))),
        Rc::new(RefCell::new(Box::new(Merlin::default(PLAYER_COUNT)))),
        Rc::new(RefCell::new(Box::new(Morgana::default()))),
        Rc::new(RefCell::new(Box::new(Pawn::default()))),
        Rc::new(RefCell::new(Box::new(Assassin::default()))),
    ];
    let mut players: Vec<Player> = Vec::with_capacity(PLAYER_COUNT);
    let mut indices: Vec<usize> = (0..PLAYER_COUNT).collect();
    let mut rng = thread_rng();
    indices.shuffle(&mut rng);

    for index in 0..PLAYER_COUNT {
        players.push(Player::new(index as u32, roles.get(indices[index]).unwrap().clone()));
    }

    for (index, player) in players.iter().enumerate() {
        // println!("{} - {:?}", index, player.role);
    }

    players
}

pub fn start_vote(players: Vec<Player>) -> bool {
    let car_round = CAR_ROUND.lock().unwrap();
    let mut rng = rand::thread_rng();
    let mut car_leader: usize = rng.gen_range(0..PLAYER_COUNT);
    let first_car_leader = car_leader;
    let mut map: HashMap<i32, (Vec<Player>, u32)> = HashMap::new();

    for (round, round_size) in car_round.iter().enumerate() {
        // println!("===================round: {}, round_size:{}===================", round, round_size);
        let car = proposal_for_car(first_car_leader, *round_size, &players);
        // 发车后投票
        vote_for_car(round, car, &players, &mut map);
        car_leader += 1;
    }
    let game_res = check_game_res(ROUND_COUNT as i32, ROUND_COUNT as i32, &map);
    game_res
}

// 发车前生成车队以及对车队表决
pub fn proposal_for_car(first_car_leader: usize, round_size: usize, players: &Vec<Player>) -> Vec<Player> {
    // 策略一：只是简单的顺时针+1
    let mut car: Vec<Player> = Vec::new();
    for i in first_car_leader..(first_car_leader + round_size) {
        car.push(players[i % PLAYER_COUNT].clone());
    }
    // println!("初始化车队: {:?}", car.iter().map(|player| player.id).collect::<Vec<_>>());
    // 发车前表决
    let mut proposal_for_car: Vec<bool> = Vec::new();
    for i in 0..PROPOSAL_MAX_COUNT {
        for player in players {
            proposal_for_car.push(player.role.borrow().proposal_for_car(player.id, &car));
        }
        if check_proposal_res(&proposal_for_car) {
            break;
        } else {
            // 如果未通过，策略是顺时针加下一个人
            car.remove(round_size - 1);
            car.push(players[(first_car_leader + round_size + i + 1) % PLAYER_COUNT].clone())
        }
    }
    // println!("表决后车队: {:?}", car.iter().map(|player| player.id).collect::<Vec<_>>());
    car
}

// 发车后投票
pub fn vote_for_car(round: usize, car: Vec<Player>, players: &Vec<Player>, map: &mut HashMap<i32, (Vec<Player>, u32)>) {
    let mut vote_res: Vec<Vote> = vec![];
    let mut vote_res_info = String::new();
    for passenger in &car {
        let vote = passenger.role.borrow().vote_with_round(round as i32);
        vote_res.push(vote);
        vote_res_info += format!("\n{}-{:?}", passenger.id, vote).as_str();
    }
    map.insert(round as i32, (car, count_vote_res(&vote_res)));
    for player in players {
        player.role.borrow_mut().update_after_vote(round as i32, &map);
    }
    // println!("投票情况: {}", vote_res_info);
    // println!("{:?}", map.get(&(round as i32)).unwrap().0);
    // println!("===================最终结果反对：{}===================", count_vote_res(&vote_res));
}


// 检查发车前表决结果
pub fn check_proposal_res(proposal_res: &Vec<bool>) -> bool {
    proposal_res.iter().filter(|proposal| **proposal == true).count() > (proposal_res.len() / 2)
}

// 检查发车后投票结果
pub fn count_vote_res(vote_res: &Vec<Vote>) -> u32 {
    vote_res.iter().filter(|vote| **vote == Reject).count() as u32
}

// 检查游戏结果
pub fn check_game_res(round: i32, round_count: i32, map: &HashMap<i32, (Vec<Player>, u32)>) -> bool {
    let mut lost_count = 0;
    for i in 0..round {
        lost_count += if map.get(&i).unwrap().1 == 0 { 0 } else { 1 }
    }
    lost_count <= round_count / 2
}