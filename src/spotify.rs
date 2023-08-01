use env_logger;
use rspotify::model::{AdditionalType, CurrentlyPlayingContext, FullTrack, PlayableItem};
use rspotify::{prelude::*, ClientError, ClientResult, Config};
use rspotify::{scopes, AuthCodeSpotify, Credentials, OAuth};

use crate::redirect::redirect_uri_web_server;
use crate::util::*;

const PORT: u16 = 8000;

pub struct Spotify {
    spotify: AuthCodeSpotify,
}
pub enum Action {
    Like,
    Unlike,
}

#[tokio::main]
pub async fn main() {
    env_logger::init();
    let spotify = Spotify::new();
    let _ = spotify.init().await;
}

impl Spotify {
    pub fn new() -> Self {
        let config = Config {
            token_cached: true,
            ..Default::default()
        };
        let creds = Credentials::new(
            "6bc8c82d87194bfbb89e60fdaeb08e47",
            "9352bc2da70941b3b3fc917a81386700",
        );
        let oauth = OAuth {
            redirect_uri: "http://localhost:8000".to_string(),
            scopes: scopes!(
                "user-library-modify",
                "user-library-read",
                "user-modify-private",
                "user-modify-playback-state",
                "user-read-currently-playing"
            ),
            ..Default::default()
        };
        let spotify = AuthCodeSpotify::with_config(creds, oauth, config);

        Spotify { spotify }
    }

    pub async fn authenticate(&self) {
        let req = redirect_uri_web_server(&self.spotify, PORT);
        println!(" >>> redirect func returns ::: {:?}", req);
        match req {
            Ok(mut r) => match parse_for_code(&mut r) {
                Some(c) => {
                    println!(" >>> Parse redirect uri into code ::: {}", c);
                    match self.spotify.request_token(&c).await {
                        Ok(_) => println!("Success?"),
                        Err(e) => println!("{:?}", e),
                    };
                }
                None => println!("Error: code was not parsed!"),
            },
            Err(_) => {}
        }
    }

    pub async fn init(&self) -> ClientResult<()> {
        match self.spotify.read_token_cache(true).await {
            Ok(Some(new_token)) => {
                let expired = new_token.is_expired();

                *self.spotify.token.lock().await.unwrap() = Some(new_token);

                if expired {
                    match self.spotify.refetch_token().await? {
                        Some(refreshed_token) => {
                            *self.spotify.get_token().lock().await.unwrap() = Some(refreshed_token)
                        }
                        None => self.authenticate().await,
                    }
                }
            }
            _ => self.authenticate().await,
        }

        Ok(())
    }
    pub async fn execute_command(&self, cmd: Action) {
        match cmd {
            Action::Like => current_add_fav(&self.spotify).await,
            Action::Unlike => current_remove_fav(&self.spotify).await,
        }
    }
}
async fn in_favorites(spotify: &AuthCodeSpotify, track: &FullTrack) -> Result<bool, ClientError> {
    let id = &track.id.clone().unwrap();
    let in_fav = spotify
        .current_user_saved_tracks_contains([id.clone()])
        .await?;

    if in_fav[0] {
        Ok(true)
    } else {
        Ok(false)
    }
}

async fn current_add_fav(spotify: &AuthCodeSpotify) {
    if let Some(track) = get_current(&spotify).await {
        let id = track.id.clone().unwrap();
        let in_favorites = in_favorites(&spotify, &track).await;
        if let Ok(in_favorites) = in_favorites {
            println!("Success?");
            if !in_favorites {
                match spotify.current_user_saved_tracks_add([id.clone()]).await {
                    Ok(_) => {
                        println!("Song {} successfully added to favorites", track.name);
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            } else {
                println!("In favorites");
            }
        } else {
            println!("No success..");
        }
    } else {
        println!("A currently playing song could not be retrieved")
    }
}

async fn current_remove_fav(spotify: &AuthCodeSpotify) {
    if let Some(track) = get_current(&spotify).await {
        let id = track.id.clone().unwrap();
        let in_favorites = in_favorites(&spotify, &track).await;
        if let Ok(in_favorites) = in_favorites {
            if in_favorites {
                match spotify.current_user_saved_tracks_delete([id.clone()]).await {
                    Ok(_) => {
                        println!("Song {} successfully removed from favorites", track.name);
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            }
        }
    } else {
        println!("A currently playing song could not be retrieved")
    }
}
async fn get_current(spotify: &AuthCodeSpotify) -> Option<FullTrack> {
    let ctx: Option<CurrentlyPlayingContext> = spotify
        .current_playing(None, Some([&AdditionalType::Track]))
        .await
        .unwrap();

    if let Some(ctx) = ctx {
        let curr_item = ctx.item.unwrap();
        match curr_item {
            PlayableItem::Track(track) => {
                println!(
                    "Currently playing: {}... Paused: {}",
                    track.name, !ctx.is_playing
                );
                Some(track)
            }
            PlayableItem::Episode(_episode) => {
                println!("currently playing a podcast");
                None
            }
        }
    } else {
        println!("User is currently not playing anything");
        None
    }
}
