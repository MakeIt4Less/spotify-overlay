use std::{collections::HashMap, time::Instant};

use base64::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_LENGTH, CONTENT_TYPE};
use tinyjson::JsonValue;
static REDIRECT_URL:&str = "http://localhost:6969";
pub struct SongInfo{
    pub title: String,
    pub artist: String,
    pub album: String,
    pub url: String,
    pub progress_ms: usize,
    pub duration_ms: usize
}

pub struct SpotifyController{
    pub access_token: String,
    pub refresh_token: String,
    pub refresh_timer: Instant,
    pub refresh_timeout: u64,
    pub current_vol: Option<u8>,
    pub can_control_vol: bool,
    pub is_playing: bool,
    pub shuffle_state: bool,
    pub currently_playing_id: String,
}

impl SpotifyController {
    pub fn new(code:String) -> SpotifyController {
        println!("Initiating Spotify");
        let (access_token,refresh_token,refresh_timeout) = post_spotify(&code);

        SpotifyController{
            access_token,
            refresh_token,
            refresh_timer:Instant::now(),
            refresh_timeout,
            current_vol:None,
            can_control_vol:true,
            is_playing: true,
            shuffle_state: false,
            currently_playing_id: String::new()
        }
    }

    pub fn get_current_song(&self) -> String{
        let auth = format!("Bearer {}",self.access_token.trim_matches('"'));
        let mut headers = HeaderMap::new();
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
        let url = "https://api.spotify.com/v1/me/player";
        let client = reqwest::blocking::Client::new();
        let response = client.get(url)
        .headers(headers)
        .send().unwrap();
        response.text().unwrap()
    }
}

fn post_spotify(code: &str) -> (String,String,u64) {
    let payload = format!("grant_type=authorization_code&code={code}&redirect_uri={REDIRECT_URL}");
    let content_length_header = format!("{}", payload.len());
    let client_id = env!("client_id");
    let client_secret = env!("client_secret");
    let auth = format!("Basic {}",BASE64_STANDARD.encode(format!("{}:{}",client_id,client_secret)));
    let mut headers = HeaderMap::new();
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&auth).unwrap());
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    let url = "https://accounts.spotify.com/api/token";
    let client = reqwest::blocking::Client::new();
    let response = client.post(url)
    .headers(headers)
    .body(payload)
    .send().unwrap();
    let request_data: JsonValue = response.text().unwrap().parse().unwrap();
    let request_data : &HashMap<_, _> = request_data.get().unwrap();
    (request_data["access_token"].format().unwrap(),request_data["refresh_token"].format().unwrap(),request_data["expires_in"].format().unwrap().parse().unwrap())
}

