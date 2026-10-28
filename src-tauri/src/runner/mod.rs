pub mod disk;
pub mod uninstall;
pub mod utils;

use crate::models::{Game, GameSource, GameStatus};

/// Applique exactement le tableau de routage de INIT-03 (Page 4 du PDF)
/// Le Dev 2 remplira les appels système réels dans DEV2-09.
pub fn execute_game_action(game: &Game) -> Result<(), String> {
    match (&game.source, &game.status) {
        // 1. JEUX LOCAUX, MANUELS OU GOOGLE DRIVE DÉJÀ INSTALLÉS
        (GameSource::Local | GameSource::Manual | GameSource::Gdrive, GameStatus::Installed) => {
            let _exe = game.executable_path.as_deref()
                .ok_or("Aucun exécutable (.exe) sélectionné pour ce jeu")?;
            let _dir = game.install_path.as_deref()
                .ok_or("Dossier d'installation introuvable")?;
            // DEV2-09 : Command::new(exe).current_dir(dir).spawn()
            // + Démarrer la boucle sysinfo en 2 phases sur install_path
            Ok(())
        }

        // 2. STEAM (Jouer vs Installer)
        (GameSource::Steam, GameStatus::Installed) => {
            let appid = game.external_id.as_deref().ok_or("AppID Steam manquant")?;
            let _uri = format!("steam://rungameid/{appid}");
            // DEV2-09 : open::that(&uri) + boucle sysinfo sur install_path
            Ok(())
        }
        (GameSource::Steam, GameStatus::NotInstalled) => {
            let appid = game.external_id.as_deref().ok_or("AppID Steam manquant")?;
            let _uri = format!("steam://install/{appid}");
            // DEV2-09 : open::that(&uri)
            Ok(())
        }

        // 3. EPIC GAMES (Jouer vs Installer)
        (GameSource::Epic, GameStatus::Installed) => {
            let app_name = game.external_id.as_deref().ok_or("AppName Epic manquant")?;
            let _uri = format!("com.epicgames.launcher://apps/{app_name}?action=launch&silent=true");
            // DEV2-09 : open::that(&uri) + boucle sysinfo sur install_path
            Ok(())
        }
        (GameSource::Epic, GameStatus::NotInstalled) => {
            // DEV2-09 : open::that("com.epicgames.launcher://")
            Ok(())
        }

        // 4. RIOT GAMES (Local uniquement)
        (GameSource::Riot, GameStatus::Installed) => {
            let _product = game.external_id.as_deref().ok_or("Produit Riot manquant")?;
            // DEV2-09 : Command::new(riot_client).args(["--launch-product=...", "--launch-patchline=live"])
            // + boucle sysinfo sur install_path
            Ok(())
        }

        // 5. BATTLE.NET (Local uniquement)
        (GameSource::Battlenet, GameStatus::Installed) => {
            let code = game.external_id.as_deref().ok_or("Code Battle.net manquant")?;
            let _arg = format!("--exec=launch {code}");
            // DEV2-09 : Command::new(bnet_exe).arg(_arg) + boucle sysinfo sur install_path
            Ok(())
        }

        _ => Err("Action non supportée pour cet état du jeu".to_string()),
    }
}
