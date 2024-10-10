use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;
use crate::info::CarInfo;
use crate::role::Camp::{Bad, Good, UNKNOWN};
use crate::role::Vote::{Approve, Reject};

const A: i32 = 20;
const B: i32 = 10;
const C: i32 = 5;


pub type RoleImpl = Rc<RefCell<Box<dyn Role>>>;

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Vote {
    Approve,
    Reject,
}

#[derive(Debug, PartialOrd, PartialEq, Clone, Copy)]
pub enum Camp {
    Good,
    Bad,
    UNKNOWN,
}

#[derive(Debug)]
pub enum Description {
    LoyalOfficial,
    Pai,
    Merlin,
    Morgana,
    Pawn,
    Assassin,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub(crate) id: u32,
    pub(crate) role: RoleImpl,
}

impl Player {
    pub fn new(id: u32, role: RoleImpl) -> Self {
        Self { id, role }
    }
}

pub trait Role: Debug + Send + Sync {
    // 
    // // 组装车队
    // fn build_for_car(&self, id: u32, size: i32, map: &HashMap<i32, (Vec<Player>, u32)>) -> Vec<i32>;

    // 投票前是否同意车队阵容
    fn proposal_for_car(&self, id: u32, car: &CarInfo) -> bool;

    // 投票
    fn vote_with_round(&self, round: i32) -> Vote;

    // 投票后更新信息
    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>);

    // 阵营
    fn get_role_camp(&self) -> Camp;

    fn info(&self) -> String;
}

#[derive(Debug)]
struct GoodRoleImpl {
    pub score: Vec<i32>,
}
impl GoodRoleImpl {
    fn new(score: Vec<i32>) -> Self {
        Self {
            score
        }
    }
}
impl Role for GoodRoleImpl {
    fn proposal_for_car(&self, _id: u32, car_info: &CarInfo) -> bool {
        // 基本策略：通过对应成员的分数是否大于0进行表决
        // !car.iter().any(|player| self.score[player.id as usize] < 0)
        // 调整为：判断当前发车成员是否是top N？
        let len = car_info.car_size;
        let mut scores = self.score.clone();
        scores.sort_by(|a, b| b.cmp(a));
        let top_scores = &scores[0..len];
        car_info.car.iter().all(|player| top_scores.contains(&self.score[player.id as usize]))
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        // 基本策略：好人无脑投赞成票
        Approve
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        // println!("update_self_info");
        // 基本策略：好人通过加分减分更新对局势的判断
        if let Some(value) = map.get(&round) {
            let car: Vec<u32> = value.0.iter().map(|index| index.id).collect();
            let reject_res = value.1;
            // println!("round : {:?} car: {:?} reject_res:{:?}", round, car, reject_res);
            if reject_res == 0 {
                // 车队通过，每人+10
                for index in car {
                    self.score[index as usize] += B;
                }
            } else {
                let mut pre_round = round - 1;
                let mut pre_car: Vec<u32> = Vec::new();
                // 循环找到上一个通过的车队
                while let Some(pre_value) = map.get(&pre_round) {
                    if pre_value.1 == 0 {
                        pre_car = pre_value.clone().0.iter().map(|index| index.id).collect();
                        break;
                    }
                    pre_round -= 1;
                }
                // println!("pre_round : {:?} pre_car: {:?}", pre_round, pre_car);
                if !pre_car.is_empty() {
                    // 与上一个通过的车队进行diff
                    let diff: Vec<u32> = car.clone().into_iter()
                        .filter(|item| !pre_car.contains(item))
                        .collect();
                    for index in car {
                        // 上一轮车发成功，这一轮没成功，diff -20 ，剩下的 -5 * reject_res
                        self.score[index as usize] -= if diff.contains(&index) { A } else { C * (reject_res as i32) };
                    }
                } else {
                    // 没有找到之前通过的车队，所以这次每人进行减分
                    for index in 0..self.score.len() {
                        self.score[index] -= C * (reject_res as i32);
                    }
                }
            }
        } else {
            panic!("投票结果有误!")
        }
    }

    fn get_role_camp(&self) -> Camp {
        Good
    }

    fn info(&self) -> String {
        format!("GoodRoleImpl")
    }
}

#[derive(Debug)]
struct BadRoleImpl {}

impl Role for BadRoleImpl {
    fn proposal_for_car(&self, id: u32, car_info: &CarInfo) -> bool {
        // 基本策略：判断车队有自己，或者有同伴才同意发车
        car_info.car.iter().any(|player| player.id == id) || car_info.car.iter().any(|player| player.role.borrow().get_role_camp() == Bad)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        if round == 0 {
            Approve
        } else {
            Reject
        }
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        // do noting
    }

    fn get_role_camp(&self) -> Camp {
        Bad
    }

    fn info(&self) -> String {
        format!("[BadRoleImpl]")
    }
}

#[derive(Debug)]
pub struct LoyalOfficial {
    proxy: GoodRoleImpl,
}
impl LoyalOfficial {
    pub fn default(size: usize) -> Self {
        Self {
            proxy: GoodRoleImpl::new(vec![0; size])
        }
    }
}
impl Role for LoyalOfficial {
    fn proposal_for_car(&self, id: u32, car_info: &CarInfo) -> bool {
        self.proxy.proposal_for_car(id, car_info)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        self.proxy.vote_with_round(round)
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        self.proxy.update_after_vote(round, map)
    }


    fn get_role_camp(&self) -> Camp {
        Good
    }

    fn info(&self) -> String {
        format!("[LoyalOfficial + {:?}]", self.proxy.score)
    }
}
#[derive(Debug)]
pub struct Merlin {
    proxy: GoodRoleImpl,
}
impl Merlin {
    pub fn default(size: usize) -> Self {
        Self {
            proxy: GoodRoleImpl::new(vec![0; size])
        }
    }
}
impl Role for Merlin {
    fn proposal_for_car(&self, _id: u32, car_info: &CarInfo) -> bool {
        // 策略一：梅林判断若车队有任何一个坏蛋都不同意
        car_info.car.iter().any(|player| player.role.borrow().get_role_camp() == Bad)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        self.proxy.vote_with_round(round)
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        self.proxy.update_after_vote(round, map)
    }


    fn get_role_camp(&self) -> Camp {
        UNKNOWN
    }

    fn info(&self) -> String {
        format!("[Merlin, score:{:?}]", self.proxy.score)
    }
}
#[derive(Debug)]
pub struct Pai {
    proxy: GoodRoleImpl,
}
impl Pai {
    pub fn default(size: usize) -> Self {
        Self {
            proxy: GoodRoleImpl::new(vec![0; size])
        }
    }
}
impl Role for Pai {
    fn proposal_for_car(&self, id: u32, car_info: &CarInfo) -> bool {
        self.proxy.proposal_for_car(id, car_info)
    }

    fn vote_with_round(&self, _round: i32) -> Vote {
        Approve
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        self.proxy.update_after_vote(round, map)
    }

    fn get_role_camp(&self) -> Camp {
        Good
    }

    fn info(&self) -> String {
        format!("[Pai + {:?}]", self.proxy.score)
    }
}
#[derive(Debug)]
pub struct Morgana {
    proxy: BadRoleImpl,
}
impl Morgana {
    pub fn default() -> Self {
        Self {
            proxy: BadRoleImpl {}
        }
    }
}
impl Role for Morgana {
    fn proposal_for_car(&self, id: u32, car_info: &CarInfo) -> bool {
        // 策略一：莫甘娜判断车队有自己，并且没有同伴才发车
        car_info.car.iter().any(|player| player.id == id) && !car_info.car.iter().any(|player| player.role.borrow().get_role_camp() == Bad)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        self.proxy.vote_with_round(round)
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        // do noting
    }

    fn get_role_camp(&self) -> Camp {
        UNKNOWN
    }

    fn info(&self) -> String {
        format!("[Morgana]")
    }
}
#[derive(Debug)]
pub struct Pawn {
    proxy: BadRoleImpl,
}
impl Pawn {
    pub fn default() -> Self {
        Self {
            proxy: BadRoleImpl {}
        }
    }
}
impl Role for Pawn {
    fn proposal_for_car(&self, id: u32, car_info: &CarInfo) -> bool {
        self.proxy.proposal_for_car(id, car_info)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        self.proxy.vote_with_round(round)
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        // do nothing!
    }

    fn get_role_camp(&self) -> Camp {
        Bad
    }

    fn info(&self) -> String {
        format!("[Pawn]")
    }
}
#[derive(Debug)]
pub struct Assassin {
    proxy: BadRoleImpl,
}
impl Assassin {
    pub fn default() -> Self {
        Self {
            proxy: BadRoleImpl {}
        }
    }
}
impl Role for Assassin {
    fn proposal_for_car(&self, id: u32, car_info: &CarInfo) -> bool {
        self.proxy.proposal_for_car(id, car_info)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        self.proxy.vote_with_round(round)
    }

    fn update_after_vote(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        // do nothing
    }

    fn get_role_camp(&self) -> Camp {
        Bad
    }

    fn info(&self) -> String {
        format!("[Assassin]")
    }
}