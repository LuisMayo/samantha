pub async fn get_pic_for_game(appid: u32) -> Result<String, Box<dyn std::error::Error>> {
    let resp = reqwest::get(format!("https://apps.luismayo.com/griddb/{}", appid))
    .await?
    .text()
    .await?;
return Ok(resp);
}