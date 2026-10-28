use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GameSource { Local, Manual, Steam, Epic, Battlenet, Riot, Gdrive }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GameStatus { Installed, NotInstalled, Downloading, Extracting }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub title: String,
    pub source: GameSource,
    pub status: GameStatus,
    pub install_path: Option<String>,
    pub executable_path: Option<String>,
    pub candidate_exes: Vec<String>,
    pub external_id: Option<String>,    // AppID Steam, AppName Epic, code Battle.net, produit Riot
    pub gdrive_file_id: Option<String>,
    pub uninstall_cmd: Option<String>,  // UninstallString du registre (Battle.net, Riot)
    pub size_bytes: Option<u64>,
    pub cover_local_path: Option<String>,
    pub playtime_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DownloadPhase { Downloading, Extracting, Completed, Cancelled, Error }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgressPayload {
    pub game_id: String,
    pub phase: DownloadPhase,
    pub downloaded_bytes: u64,
    pub total_bytes: u64,
    pub speed_bytes_per_sec: u64,
    pub error_message: Option<String>,
}