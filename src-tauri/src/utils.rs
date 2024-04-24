use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use image::{ImageBuffer, Rgba};
use steamworks::{stats::AchievementHelper, Client, SingleClient, UserAchievementIconFetched, UserAchievementStored, UserStatsReceived};
use tauri::async_runtime::{block_on, channel};

pub async fn request_current_stats(client: &Client) {
    let (sender, mut receiver) = channel::<bool>(1);
    let _ = client.register_callback(move |val: UserStatsReceived| {
        block_on(sender.send(val.result.is_ok())).expect("Doesn't work");
    });
    client.user_stats().request_current_stats();
    receiver.recv().await;
    return;
}

pub fn request_icon(client: &Client, achievement: &AchievementHelper<'_, steamworks::ClientManager>) -> ImageBuffer::<Rgba<u8>, Vec<u8>>  {
    // TODO missing callback definition in Steamworks crate: https://github.com/Noxime/steamworks-rs/issues/171
    let (sender, mut receiver) = std::sync::mpsc::channel::<()>();
    let _ = client.register_callback(move |val: UserAchievementIconFetched| {
        sender.send(()).unwrap_or_default();
    });
    let mut icon = achievement.get_achievement_icon_v2();
    if icon.is_none() {
        receiver.recv();
        icon = achievement.get_achievement_icon_v2();
    }

    return icon.unwrap_or_default();
}
