export type GameSource = "local" | "manual" | "steam" | "epic" | "battlenet" | "riot" | "gdrive";
export type GameStatus = "installed" | "not_installed" | "downloading" | "extracting";
export type DownloadPhase = "downloading" | "extracting" | "completed" | "cancelled" | "error";

export interface Game {
    id: string;
    title: string;
    source: GameSource;
    status: GameStatus;
    install_path: string | null;
    executable_path: string | null;
    candidate_exes: string[];
    external_id: string | null;
    gdrive_file_id: string | null;
    uninstall_cmd: string | null;
    size_bytes: number | null;
    cover_local_path: string | null;
    playtime_seconds: number;
}

export interface DownloadProgressPayload {
    game_id: string;
    phase: DownloadPhase;
    downloaded_bytes: number;
    total_bytes: number;
    speed_bytes_per_sec: number;
    error_message: string | null;
}

export interface GameSessionEndedPayload {
    game_id: string;
    playtime_seconds: number;
}

// Noms exacts des 3 événements Tauri (pour éviter les fautes de frappe)
export const EVENTS = {
    DOWNLOAD_PROGRESS: "download-progress",
    NETWORK_STATUS_CHANGED: "network-status-changed",
    GAME_SESSION_ENDED: "game-session-ended",
} as const;