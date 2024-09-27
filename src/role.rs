use std::fmt::Debug;
use std::rc::Rc;
use std::sync::Arc;
use crate::role::Camp::{Bad, Good};
use crate::role::Vote::{Approve, Reject};

pub type RoleImpl = Arc<Box<dyn Role>>;

#[derive(Debug)]
pub enum Vote {
    Approve,
    Reject,
}

#[derive(Debug)]
pub enum Camp {
    Good,
    Bad,
}

#[derive(Debug)]
pub enum Description{
    LoyalOfficial,
    Pai,
    Merlin,
    Morgana,
    Pawn,
    Assassin,
}

pub struct Player {
    pub(crate) role:RoleImpl,
}

impl Player {
    pub fn new(role:RoleImpl) -> Self {
        Self { role }
    }
}

pub trait Role: Debug + Send + Sync {
    fn vote_with_round(&self, round: u32) -> Vote;

    fn get_role_camp(&self) -> Camp;
}

#[derive(Debug)]
pub struct LoyalOfficial {}
impl Role for LoyalOfficial {
    fn vote_with_round(&self, round: u32) -> Vote {
        Approve
    }

    fn get_role_camp(&self) -> Camp {
        Good
    }
    
}
#[derive(Debug)]
pub struct Merlin {}
impl Role for Merlin {
    fn vote_with_round(&self, round: u32) -> Vote {
        Approve
    }

    fn get_role_camp(&self) -> Camp {
        Good
    }
}
#[derive(Debug)]
pub struct Pai {}
impl Role for Pai {
    fn vote_with_round(&self, round: u32) -> Vote {
        Approve
    }

    fn get_role_camp(&self) -> Camp {
        Good
    }
}
#[derive(Debug)]
pub struct Morgana {}
impl Role for Morgana {
    fn vote_with_round(&self, round: u32) -> Vote {
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
pub struct Pawn {}
impl Role for Pawn {
    fn vote_with_round(&self, round: u32) -> Vote {
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
    fn vote_with_round(&self, round: u32) -> Vote {
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