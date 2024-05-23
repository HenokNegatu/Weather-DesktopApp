// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use reqwest;
use dotenv::dotenv;

fn get_env_var(api_key:&str) -> String{
    dotenv().ok();
    dotenv::var(api_key).unwrap()
}


#[tauri::command]
async fn fetch_data(address:String, key:String) -> Result<String, String> {
    println!("{}",key);
    // let result = get_env_var("VITE_APIKEY");
    let url = format!("http://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", key, address);
    let client = reqwest::Client::new()
    .get(url)
    .send()
    .await.map_err(|e| e.to_string())?
    .text()
    .await.map_err(|e| e.to_string())?;
    // let weather_data:Response = serde_json::from_str(&client).unwrap();
    // println!("{:#?}", weather_data);
    // println!("{}", client);
    Ok(client)
}
