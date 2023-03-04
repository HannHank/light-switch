use core::fmt;
use std::net::Ipv4Addr;

use serde::{Serialize, Deserialize};
use derive_getters::Getters;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct wifi_sta{
    pub connected: bool,
    pub ssid: String,
    pub ip: String,
    pub rssi: i32
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct cloud {
    pub enabled: bool,
    pub connected: bool
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct mqtt {
    pub connected: bool
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct actions_stats {
    pub skipped: i32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct relay {
    pub ison: bool,
    pub has_timer: bool,
    pub timer_started: i32,
    pub timer_duration: i32,
    pub timer_remaining: i32,
    pub overpower: bool,
    pub overtemperature: Option<bool>,
    pub is_valid: Option<bool>,
    pub source: String
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Turn {
    On,
    Off,
    Toggle
}
impl fmt::Display for Turn {
    fn fmt (&self,f: &mut fmt::Formatter<'_>)->fmt::Result{
       match self{
           Turn::On=>write!(f,"on"),
           Turn::Off=>write!(f,"off"),
           Turn::Toggle=>write!(f,"toggle")
       }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct set_relay{
    pub Turn:Turn
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct meter {
    pub power: f32,
    pub overpower: f32,
    pub is_valid: Option<bool>,
    pub timestamp: u64,
    pub counters: (f32,f32,f32),
    pub total: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct input {
    pub input: i32,
    pub event: String,
    pub event_cnt: u32,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct tmp {
    pub tC: f64,
    pub tF: f64,
    pub is_valid: Option<bool>
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct update {
    pub status: String,
    pub has_update: bool,
    pub new_version: String,
    pub old_version: String
}
#[derive(Debug, Clone, Serialize, Deserialize, Getters)]
pub struct status {
    pub wifi_sta: wifi_sta,
    pub cloud: cloud,
    pub mqtt: mqtt,
    pub time:String,
    pub unixtime: u64,
    pub serial: i32,
    pub has_update: bool,
    pub mac: String,
    pub cfg_changed_cnt: i32,
    pub actions_stats: actions_stats,
    pub relays: Box<[relay]>,
    pub meters: Box<[meter]>,
    pub inputs: Option<Box<[input]>>,
    pub temperature: Option<f32>,
    pub overtemperature: Option<bool>,
    pub tmp: tmp,
    pub temperature_status: Option<String>,
    pub update: update,
    pub ram_total: i32,
    pub ram_free: i32,
    pub fs_size: i32,
    pub fs_free: i32,
    pub voltage: Option<f64>,
    pub uptime: i64
}
#[derive(Debug,Eq,Hash, PartialEq, Clone)]
pub struct Device {
    pub ip: Ipv4Addr,
    pub name: Option<String>,
    pub relay_names: Vec<String>
}
impl Device {
    pub fn get_ip(&self)->Ipv4Addr{
       self.ip.clone() 
    }
}
