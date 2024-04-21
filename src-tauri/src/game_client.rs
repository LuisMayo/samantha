use std::{io::Cursor, thread, time::Duration};

use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE},
    Engine,
};
use image::{Rgb, Rgba};
use serde::{Deserialize, Serialize};
use steamworks::{stats::AchievementHelper, Client};
use tauri::Error;

use crate::{
    callback_runner::Callback_Runner,
    utils::{request_current_stats, request_icon},
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievement {
    name: String,
    unlocked: bool,
    screen_name: Option<String>,
    description: Option<String>,
    hidden: bool,
    image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AchRequest {
    name: String,
    unlock: bool,
}

fn get_image_data(data: Vec<u8>) -> String {
    let img_result = image::ImageBuffer::<Rgba<u8>, Vec<u8>>::from_vec(64, 64, data);
    // let img_result = image::ImageBuffer::<Rgb<u8>, Vec<u8>>::from_vec(48, 48, data);
    // let mut img_result = image::ImageBuffer::from_vec(48, 48, data);
    match img_result {
        Some(img) => {
            let mut png_data = Vec::<u8>::new();
            let mut cursor = Cursor::new(&mut png_data);
            img.write_to(&mut cursor, image::ImageFormat::Png)
                .unwrap_or_default();
            return STANDARD.encode(png_data);
        }
        None => {
            // println!("{}", err.to_string());
            return "".to_string();
        }
    }
}

#[tauri::command]
pub fn set_achievements(appid: u32, achievement_list: Vec<AchRequest>) -> Result<(), String> {
    match set_achievements_internal(appid, achievement_list) {
        Ok(_) => return Ok(()),
        Err(err) => return Err(err.to_string()),
    };
}

fn set_achievements_internal(
    appid: u32,
    achievement_list: Vec<AchRequest>,
) -> Result<(), Box<dyn std::error::Error>> {
    let init = Client::init_app(appid)?;
    let mut result = true;
    achievement_list.into_iter().for_each(|achievement| {
        result = init
            .0
            .user_stats()
            .achievement(&achievement.name)
            .set()
            .is_ok()
            && result;
    });
    if result {
        return Ok(());
    } else {
        return Err("There were problems setting some achievements")?;
    };
}

#[tauri::command]
pub async fn get_achievement_list(appid: u32) -> Result<Vec<Achievement>, String> {
    let init = Client::init_app(appid);
    match init {
        Ok((client, single_client)) => {
            let _runner = Callback_Runner::new(single_client);
            request_current_stats(&client).await;
            let ach_array: Vec<Achievement> = client
                .user_stats()
                .get_achievement_names()
                .unwrap_or(vec![])
                .into_iter()
                .map(|ach_name| {
                    let stats = client.user_stats();
                    let achievement = stats.achievement(&ach_name);
                    // Upstream needs a more permanent solution
                    return Achievement {
                        name: ach_name,
                        unlocked: achievement.get().unwrap_or(false),
                        image: get_image_data(request_icon(&client, &achievement)),
                        screen_name: if let Ok(scr_name) =
                            achievement.get_achievement_display_attribute("name")
                        {
                            Some(scr_name.to_string())
                        } else {
                            None
                        },
                        description: if let Ok(desc) =
                            achievement.get_achievement_display_attribute("desc")
                        {
                            Some(desc.to_string())
                        } else {
                            None
                        },
                        hidden: achievement
                            .get_achievement_display_attribute("hidden")
                            .unwrap_or("0")
                            == "1",
                    };
                })
                .collect();
            return Ok(ach_array);
        }
        Err(e) => return Err(e.to_string()),
    }
}
