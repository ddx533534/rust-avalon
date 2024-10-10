use crate::role::Player;

struct CarInfo {
    // 车次
    round: i32,
    // 车上乘客
    car: Vec<Player>,
    // 车上人数
    car_size: i32,
    // 反对票数
    reject_count: i32,

}
impl CarInfo {
    pub fn new(round: i32, car: Vec<Player>, car_size: i32, reject_count: i32) -> Self {
        Self {
            round,
            car,
            car_size,
            reject_count,
        }
    }
}
struct GameInfo {
    // 发车信息
    cars: Vec<CarInfo>,
}
impl GameInfo {
    pub fn default() -> Self {
        Self {
            cars: Vec::new(),
        }
    }
}