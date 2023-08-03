mod redirect;
mod spotify;
mod util;
use spotify::{Action, Spotify};

#[tokio::main]
async fn main() {
    let mut event_handler = EventHandler::new();
    event_handler.await.recv().await;
}

use neovim_lib::{Neovim, NeovimApi, Session};

// #[derive(Default)]
struct EventHandler {
    nvim: Neovim,
    spotify: Spotify,
}

impl EventHandler {
    async fn new() -> Self {
        let session = Session::new_parent().unwrap();
        let nvim = Neovim::new(session);
        let spotify = Spotify::new();

        EventHandler { nvim, spotify }
    }

    async fn recv(&mut self) {
        let _ = self.spotify.init().await;
        let receiver = self.nvim.session.start_event_loop_channel();

        for (event, _) in receiver {
            match Cmd::from(event) {
                Cmd::Like => {
                    self.spotify.execute_command(Action::Like).await;
                    self.nvim
                        .command(&format!("echo \"Added song to favorites\""))
                        .unwrap();
                }
                Cmd::Unlike => {
                    self.spotify.execute_command(Action::Unlike).await;
                    self.nvim
                        .command(&format!("echo \"Removed song to favorites\""))
                        .unwrap();
                }
                Cmd::Unknown(uevent) => {
                    self.nvim
                        .command(&format!("echo \"Unknown command: {}\"", uevent))
                        .unwrap();
                }
            }
        }
    }
}

enum Cmd {
    Like,
    Unlike,
    Unknown(String),
}

impl From<String> for Cmd {
    fn from(event: String) -> Self {
        match &event[..] {
            "like" => Cmd::Like,
            "unlike" => Cmd::Unlike,
            _ => Cmd::Unknown(event),
        }
    }
}
