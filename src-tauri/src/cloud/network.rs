use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tauri::{AppHandle, Emitter};

/// État réseau global mis en cache (accessible instantanément par tout le backend Rust).
/// Initialisé à `true` par défaut avant le premier test.
static IS_ONLINE: AtomicBool = AtomicBool::new(true);

/// URL ultra-légère de Google (conçue pour les tests de connectivité d'Android/Chrome, répond 204 sans corps).
const CONNECTIVITY_URL: &str = "https://clients3.google.com/generate_204";

/// Lecture synchrone et instantanée de l'état réseau pour les autres modules Rust (ex: DEV3-02 SteamGridDB).
pub fn is_online() -> bool {
    IS_ONLINE.load(Ordering::Relaxed)
}

/// Exécute une requête HEAD avec un timeout strict de 3 secondes.
async fn check_connection(client: &reqwest::Client) -> bool {
    match client.head(CONNECTIVITY_URL).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false, // Timeout dépassé ou DNS/réseau coupé
    }
}

/// Commande Tauri appelée par le Frontend au démarrage (DEV1-04).
/// Fait un test réel immédiat pour éviter tout faux positif au lancement de l'application.
pub async fn get_network_status() -> Result<bool, String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .map_err(|e| format!("Erreur création client HTTP: {}", e))?;

    let current_status = check_connection(&client).await;
    IS_ONLINE.store(current_status, Ordering::Relaxed);

    Ok(current_status)
}

/// Lance la boucle de surveillance périodique en tâche de fond.
/// Émet "network-status-changed" uniquement quand l'état bascule.
pub fn start_network_monitor(app_handle: AppHandle) {
    tauri::async_runtime::spawn(async move {
        // On construit le client HTTP une seule fois hors de la boucle pour réutiliser le pool de connexions.
        // .unwrap_or_default() est utilisé ici par sécurité au lieu de .unwrap() pour ne jamais crasher.
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(3))
            .build()
            .unwrap_or_default();

        // 1. Premier check au démarrage pour initialiser proprement IS_ONLINE
        let initial_status = check_connection(&client).await;
        let previous = IS_ONLINE.swap(initial_status, Ordering::Relaxed);
        if previous != initial_status {
            let _ = app_handle.emit("network-status-changed", initial_status);
        }

        // 2. Boucle périodique toutes les 10 secondes
        loop {
            tokio::time::sleep(Duration::from_secs(10)).await;

            let current_status = check_connection(&client).await;
            let previous_status = IS_ONLINE.swap(current_status, Ordering::Relaxed);

            // On émet l'événement UNIQUEMENT si l'état a changé (En ligne <-> Hors-ligne)
            if current_status != previous_status {
                let _ = app_handle.emit("network-status-changed", current_status);
            }
        }
    });
}