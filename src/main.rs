mod player;

use iced::executor;
use iced::pure::{Application, Element};
use iced::{Command, Settings};

pub fn main() -> iced::Result {
    Seashell::run(Settings::default())
}

struct Seashell {
    player: player::Player,
}

#[derive(Debug, Clone)]
enum Message {
    Player(player::Message),
}

impl Application for Seashell {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, Command<Message>) {
        (
            Seashell {
                player: player::Player::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Seashell")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Player(msg) => self.player.update(msg).map(Message::Player),
        }
    }

    fn view(&self) -> Element<Message> {
        self.player.view().map(Message::Player)
    }
}
