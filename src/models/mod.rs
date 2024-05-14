use std::sync::{Arc, Mutex};

use bus::BusReader;
pub mod app_settings;
pub mod steamid;

use crate::{appbus::AppBus, tf2::lobby::Lobby};

use self::{app_settings::AppSettings, steamid::SteamID};

pub struct AppWin {
    pub lobby: Lobby,
    pub lobby_report_bus_rx: BusReader<Lobby>,
    pub self_steamid: SteamID,
    pub swap_team_colors: bool,
    pub show_crits: bool,
}

impl AppWin {
    pub fn new(settings: &AppSettings, bus: &Arc<Mutex<AppBus>>) -> Self {
        Self {
            lobby: Lobby::new(),
            lobby_report_bus_rx: bus.lock().unwrap().lobby_report_bus.add_rx(),
            swap_team_colors: false,
            show_crits: true,
            self_steamid: settings.self_steamid64,
        }
    }

    pub fn process_bus(&mut self) {
        while let Ok(lobby) = self.lobby_report_bus_rx.try_recv() {
            self.lobby = lobby;
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlayerFlags {
    Cheater,
    Bot,
    Sus,
    New,
    Racist,
    Exploiter,
}

pub fn flag_shortname(flag: PlayerFlags) -> &'static str {
    match flag {
        PlayerFlags::Cheater => "C",
        PlayerFlags::Bot => "B",
        PlayerFlags::Sus => "S",
        PlayerFlags::New => "N",
        PlayerFlags::Racist => "R",
        PlayerFlags::Exploiter => "E",
    }
}

pub fn flag_description(flag: PlayerFlags) -> &'static str {
    match flag {
        PlayerFlags::Cheater => "Cheater",
        PlayerFlags::Bot => "Bot",
        PlayerFlags::Sus => "Suspicious",
        PlayerFlags::New => "New Account",
        PlayerFlags::Racist => "Racist",
        PlayerFlags::Exploiter => "Exploiter",
    }
}