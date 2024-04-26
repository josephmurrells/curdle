use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use std::env;
use reqwest::Error;

#[derive(Deserialize, Serialize, Debug)]
struct WordJson {
    word: String,
    id: i16
}

pub async fn words_from_json() -> Vec<String> {
    let path = env::current_dir().expect("couldnt get path").join("words.json");
    if !path.try_exists().unwrap()
    {
        get_json(&path).await.expect("Failed to get json");
    }
    let file =  fs::read_to_string(path).expect("Failed to open file");
    let words:Vec<WordJson> = serde_json::from_str(&file).expect("Failed to parse json");
    let words_array:Vec<String> = words.iter().map(|w| w.word.clone()).collect();
    return words_array;
}

async fn get_json(path:&PathBuf) -> Result<(), Error> {
    println!("Downloading dictionary file...");
    let response = reqwest::get("https://wordle-api.cyclic.app/words").await?;
    let words: Vec<WordJson> = response.json().await?;
    let json_string = serde_json::to_string(&words).unwrap();
    fs::write(path, json_string).expect("Unable to write file");
    println!("Dictionary file successfully downloaded!");
    println!("----------------------------------------------------------------");

    Ok(())
}
