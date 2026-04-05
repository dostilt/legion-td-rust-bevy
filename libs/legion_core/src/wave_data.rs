pub const WAVE_COUNT: [u32; 32]    = [0,36,45,40,36,36,36,30,36,45,3,54,45,45,26,36,45,35,45,36,3,36,48,36,35,45,36,36,18,30,3,15];
pub const WAVE_BOUNTY: [u32; 32]   = [0,3,3,4,5,5,5,6,6,5,51,5,6,7,12,9,8,10,8,10,86,10,9,11,11,9,12,12,23,14,123,0];
pub const WAVE_END_GOLD: [u32; 31] = [0,11,12,13,14,16,18,20,23,26,30,35,40,45,50,55,60,70,80,90,100,110,120,130,140,150,160,170,180,190,200];
pub const RECOMMEND_VALUE: [u32; 31] = [0,250,350,500,650,800,1000,1200,1450,1600,1850,2050,2400,2700,3100,3500,4000,4500,5000,5500,6000,6500,7100,7700,8500,9500,10600,11800,13000,14000,15000];
pub const SELL_PERCENT: f32        = 0.50;
pub const SELL_PERCENT_PREBATTLE: f32 = 0.90;
pub const STARTING_GOLD: u32       = 750;
pub const STARTING_LUMBER: u32     = 150;
pub const MAX_WAVE: u8             = 30;

pub fn build_timer_secs(wave: u8) -> u32 { 40 + (wave as u32 / 2) }

pub fn income_cap(wave: u8) -> u32 {
    let w = wave as f64;
    (0.025 * w.powi(3) + 0.05 * w.powi(2) + 4.0 * w + 20.0) as u32
}
