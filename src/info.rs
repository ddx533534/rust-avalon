use crate::role::Player;

pub struct CarInfo {
    // 车次
    pub round: usize,
    // 车上乘客
    pub car: Vec<Player>,
    // 车上人数
    pub car_size: usize,
    // 反对票数
    pub reject_count: usize,

}
impl CarInfo {
    pub fn new(round: usize, car: Vec<Player>, car_size: usize, reject_count: usize) -> Self {
        Self {
            round,
            car,
            car_size,
            reject_count,
        }
    }
}
pub struct GameInfo {
    // 发车信息
    pub cars: Vec<CarInfo>,
}
impl GameInfo {
    pub fn default() -> Self {
        Self {
            cars: Vec::new(),
        }
    }
}