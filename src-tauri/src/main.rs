// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use responses::App;
use steamworks::Client;

mod game_client;
mod responses;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn locate_games() -> Result<Vec<App>, String> {
    let result = internal_locate_games().await;
    match result {
        Ok(val) => return Ok(val),
        Err(err) => return Err(err.to_string()),
    }
}

async fn internal_locate_games() -> Result<Vec<App>, Box<dyn std::error::Error>> {
    // cache
    let resp = reqwest::get("https://dl.luismayo.com/samantha/known-achievements.json")
        .await?
        .json::<Vec<App>>()
        .await?;
    return Ok(check_ownership(resp)?);
}

fn check_ownership(games: Vec<App>) -> Result<Vec<App>, Box<dyn std::error::Error>> {
    let (client, _) = Client::init_app(480)?;
    let filtered_games = games
        .into_iter()
        .filter(|game| {
            client
                .apps()
                .is_subscribed_app(steamworks::AppId(game.appid))
        })
        .collect();
    return Ok(filtered_games);
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            locate_games,
            game_client::get_achievement_list
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
