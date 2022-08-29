use iced::pure::{button, column, container, row, slider, text, Element};
use iced::{Alignment, Command, Length};

use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings, MainPlaybackState};
use kira::sound::streaming::{StreamingSoundData, StreamingSoundSettings};
use kira::tween::Tween;

#[derive(Debug, Clone, Copy)]
pub enum AudioStatus {
    Playing,
    Paused,
    None,
}

impl std::fmt::Display for AudioStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Play,
    Pause,
    Stop,
    VolumeChanged(f64),
}

pub struct Player {
    manager: Option<AudioManager<CpalBackend>>,
    volume: f64,
    status: AudioStatus,
    music: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            manager: None,
            volume: 100.0,
            status: AudioStatus::None,
            music: String::new(),
        }
    }
}

impl Player {
    pub fn set_music(&mut self, music: &str) {
        self.music = music.to_string();
    }

    pub fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Play => self.play(),
            Message::Pause => self.pause(),
            Message::VolumeChanged(value) => self.set_volume(value),
            Message::Stop => self.stop(),
        }
        Command::none()
    }

    pub fn view(&self) -> Element<Message> {
        let play_button = button("Play").on_press(Message::Play);
        let pause_button = button("Pause").on_press(Message::Pause);
        let stop_button = button("Stop").on_press(Message::Stop);

        let buttons = row()
            .push(play_button)
            .push(pause_button)
            .push(stop_button)
            .align_items(Alignment::Center);

        let status = text(self.status.to_string());
        let info = row().push(status).align_items(Alignment::Center);

        let slider = slider(0.0..=100.0, self.volume, Message::VolumeChanged)
            .step(1.0)
            .width(Length::Units(150));
        let volume_text = text(self.volume.to_string());
        let volume = row()
            .push(slider)
            .spacing(5)
            .push(volume_text)
            .align_items(Alignment::Center);

        let content = column()
            .push(buttons)
            .push(info)
            .push(volume)
            .align_items(Alignment::Center);

        container(content).center_x().center_y().into()
    }

    fn play(&mut self) {
        if let Some(manager) = self.manager.as_mut() {
            match manager.state() {
                MainPlaybackState::Playing => {}
                MainPlaybackState::Pausing | MainPlaybackState::Paused => {
                    manager.resume(Tween::default());

                    let mut sound = manager.main_track();
                    sound.set_volume(self.volume / 100.0, Tween::default());

                    self.status = AudioStatus::Playing;
                }
            }
        } else {
            // Create sound data
            if let Ok(sound_data) =
                StreamingSoundData::from_file(&self.music, StreamingSoundSettings::default())
            {
                // Initialize audio manager
                self.manager = Some(
                    AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap(),
                );

                // Play sound
                if let Some(manager) = self.manager.as_mut() {
                    manager.play(sound_data);

                    let mut sound = manager.main_track();
                    sound.set_volume(self.volume / 100.0, Tween::default());

                    self.status = AudioStatus::Playing;
                }
            }
        }
    }

    fn pause(&mut self) {
        if let Some(manager) = self.manager.as_mut() {
            if matches!(manager.state(), MainPlaybackState::Playing) {
                manager.pause(Tween::default());
                self.status = AudioStatus::Paused;
            }
        }
    }

    fn set_volume(&mut self, volume: f64) {
        self.volume = volume;

        if let Some(manager) = self.manager.as_mut() {
            if matches!(manager.state(), MainPlaybackState::Playing) {
                let mut sound = manager.main_track();
                sound.set_volume(self.volume / 100.0, Tween::default());
            }
        }
    }

    fn stop(&mut self) {
        // Reset manager
        self.manager = None;
        self.status = AudioStatus::None;
    }
}
