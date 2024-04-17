use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use steamworks::{stats::AchievementHelper, Client, SingleClient, UserAchievementStored, UserStatsReceived};
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

pub fn request_icon(_client: &Client, achievement: &AchievementHelper<'_, steamworks::ClientManager>) -> Vec<u8> {
    // TODO missing callback definition in Steamworks crate: https://github.com/Noxime/steamworks-rs/issues/171
    // let (sender, mut receiver) = channel::<bool>(1);
    // let _ = client.register_callback(move |val: TODO-CALLBACK| {
    //     block_on(sender.send(val.result.is_ok())).expect("Doesn't work");
    // });
    // client.user_stats().request_current_stats();
    // receiver.recv().await;
    let mut i: u8 = 0;
    let mut icon: Option<Vec<u8>> = None;
    while i < 3 && icon.is_none() {
        icon = achievement.get_achievement_icon();
        i += 1;
        thread::sleep(Duration::from_millis(300));
    };

    return icon.unwrap_or_default();
    // thread::sleep(Duration::from_millis(300));
    // let achi_icon = achievement.get_achievement_icon().unwrap_or_default();
}
