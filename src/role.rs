use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

use crate::role::Camp::{Bad, Good, UNKNOWN};
use crate::role::Vote::{Approve, Reject};

const A: i32 = 20;
const B: i32 = 10;
const C: i32 = 5;


pub type RoleImpl = Arc<Box<dyn Role>>;

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

pub struct Player {
    pub(crate) id: usize,
    pub(crate) role: RoleImpl,
}

impl Player {
    pub fn new(id: usize, role: RoleImpl) -> Self {
        Self { id, role }
    }
}

pub trait Role: Debug + Send + Sync {
    // 投票前是否同意发车
    fn proposal_for_car(&self, id: u32, car: &Vec<Player>) -> bool;

    // 投票
    fn vote_with_round(&self, round: i32) -> Vote;

    // 投票后更新信息
    fn update_self_info(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>);

    // 阵营
    fn get_role_camp(&self) -> Camp;
}

#[derive(Debug)]
pub struct LoyalOfficial {
    score: Vec<i32>,
}
impl LoyalOfficial {
    fn default(size: usize) -> Self {
        Self {
            score: Vec::with_capacity(size),
        }
    }
}
impl Role for LoyalOfficial {
    fn proposal_for_car(&self, id: u32, car: &Vec<Player>) -> bool {
        // 目前是最简单的方案，好人通过对应成员的分数是否大于0进行表决
        !car.iter().any(|player| self.score[player.id] < 0)
    }

    fn vote_with_round(&self, _round: i32) -> Vote {
        Approve
    }

    fn update_self_info(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        if let Some(value) = map.get(&round) {
            let car: Vec<usize> = value.0.iter().map(|index| index.id).collect();
            let reject_res = value.1;
            if reject_res == 0 {
                // 车队通过，每人+20
                for index in car {
                    self.score[index] += A;
                }
            } else {
                let mut pre_round = round - 1;
                let mut pre_car: Vec<usize> = Vec::new();
                // 循环找到上一个通过的车队
                while let Some(pre_value) = map.get(&pre_round) {
                    if pre_value.1 == 0 {
                        pre_car = pre_value.clone().0.iter().map(|index| index.id).collect();
                        break;
                    }
                    pre_round -= 1;
                }
                if !pre_car.is_empty() {
                    // 与上一个通过的车队进行diff
                    let diff: Vec<usize> = car.clone().into_iter()
                        .filter(|item| !pre_car.contains(item))
                        .collect();
                    for index in 0..self.score.len() {
                        // 上一轮车发成功，这一轮没成功，diff -20 ，剩下的 -5 * reject_res
                        self.score[index] -= if diff.contains(&index) { A } else { C * (reject_res as i32) };
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
}
#[derive(Debug)]
pub struct Merlin {}
impl Role for Merlin {
    fn proposal_for_car(&self, id: u32, car: &Vec<Player>) -> bool {
        // 策略一：梅林判断若车队有任何一个坏蛋都不同意
        car.iter().any(|player| player.role.get_role_camp() == Bad)
    }

    fn vote_with_round(&self, _round: i32) -> Vote {
        Approve
    }

    fn update_self_info(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        // do nothing!
    }


    fn get_role_camp(&self) -> Camp {
        UNKNOWN
    }
}
#[derive(Debug)]
pub struct Pai {
    score: Vec<i32>,
}
impl Pai {
    fn default(size: usize) -> Self {
        Self {
            score: Vec::with_capacity(size),
        }
    }
}
impl Role for Pai {
    fn proposal_for_car(&self, id: u32, car: &Vec<Player>) -> bool {
        // 策略一：通过对应成员的分数是否大于0进行表决
        !car.iter().any(|player| self.score[player.id] < 0)
    }

    fn vote_with_round(&self, _round: i32) -> Vote {
        Approve
    }

    fn update_self_info(&mut self, _round: i32, _map: &HashMap<i32, (Vec<Player>, u32)>) {
        // do nothing
    }

    fn get_role_camp(&self) -> Camp {
        Good
    }
}
#[derive(Debug)]
pub struct Morgana {}
impl Role for Morgana {
    fn proposal_for_car(&self, id: u32, car: &Vec<Player>) -> bool {
        // 策略一：莫甘娜判断车队没有自己，或者有自己但有任何一个坏蛋都不同意发车
        car.iter().any(|player| player.role.get_role_camp() == Bad)
    }

    fn vote_with_round(&self, round: i32) -> Vote {
        if round == 0 {
            Approve
        } else {
            Reject
        }
    }

    fn update_self_info(&mut self, round: i32, map: &HashMap<i32, (Vec<Player>, u32)>) {
        todo!()
    }

    fn get_role_camp(&self) -> Camp {
        UNKNOWN
    }
}
#[derive(Debug)]
pub struct Pawn {}
impl Role for Pawn {
    fn vote_with_round(&self, round: i32) -> Vote {
        if round == 0 {
            Approve
        } else {
            Reject
        }
    }

    fn get_role_camp(&self) -> Camp {
        Bad
    }
}
#[derive(Debug)]
pub struct Assassin {}
impl Role for Assassin {
    fn vote_with_round(&self, round: i32) -> Vote {
        if round == 0 {
            Approve
        } else {
            Reject
        }
    }

    fn get_role_camp(&self) -> Camp {
        Bad
    }
}