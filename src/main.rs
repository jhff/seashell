mod explorer;
mod player;

use iced::pure::{column, container, Application, Element};
use iced::{executor, Alignment};
use iced::{Command, Settings};

pub fn main() -> iced::Result {
    Seashell::run(Settings::default())
}

struct Seashell {
    player: player::Player,
    explorer: explorer::Explorer,
}

#[derive(Debug, Clone)]
enum Message {
    Explorer(explorer::Message),
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
                explorer: explorer::Explorer::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Seashell")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Explorer(msg) => {
                match &msg {
                    explorer::Message::Selected(selected) => match selected {
                        explorer::Selected::File(file) => self.player.set_music(file),
                        explorer::Selected::Dir(_) => {}
                    },
                    explorer::Message::InputChanged(_) | explorer::Message::BackDir => {}
                }

                self.explorer.update(msg).map(Message::Explorer)
            }
            Message::Player(msg) => self.player.update(msg).map(Message::Player),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column()
            .push(self.player.view().map(Message::Player))
            .push(self.explorer.view().map(Message::Explorer))
            .align_items(Alignment::Center);

        container(content).center_x().center_y().into()
    }
}
