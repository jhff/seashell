use iced::pure::{button, column, container, row, slider, text, Element};
use iced::{Alignment, Command, Length};

use kira::manager::backend::cpal::CpalBackend;
use kira::manager::{AudioManager, AudioManagerSettings, MainPlaybackState};
use kira::sound::static_sound::PlaybackState;
use kira::sound::streaming::{StreamingSoundData, StreamingSoundHandle, StreamingSoundSettings};
use kira::sound::FromFileError;
use kira::tween::Tween;

#[derive(Debug, Clone, Copy)]
pub enum AudioStatus {
    Playing,
    Pausing,
    Paused,
    Stopping,
    Stopped,
    None,
}

impl Default for AudioStatus {
    fn default() -> Self {
        AudioStatus::None
    }
}

impl From<PlaybackState> for AudioStatus {
    fn from(state: PlaybackState) -> Self {
        match state {
            PlaybackState::Playing => AudioStatus::Playing,
            PlaybackState::Pausing => AudioStatus::Pausing,
            PlaybackState::Paused => AudioStatus::Paused,
            PlaybackState::Stopping => AudioStatus::Stopping,
            PlaybackState::Stopped => AudioStatus::Stopped,
        }
    }
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
    handle: Option<StreamingSoundHandle<FromFileError>>,
    volume: f64,
    status: AudioStatus, // TODO
    music: String,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            manager: None,
            handle: None,
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

        let status = text(self.state().to_string());
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
        match self.state() {
            AudioStatus::Playing => {}
            AudioStatus::Pausing | AudioStatus::Paused => {
                if let Some(handle) = self.handle.as_mut() {
                    handle.resume(Tween::default());
                }
            }
            AudioStatus::Stopping | AudioStatus::Stopped | AudioStatus::None => {
                if self.manager.is_none() {
                    // Initialize audio manager
                    self.manager = Some(
                        AudioManager::<CpalBackend>::new(AudioManagerSettings::default()).unwrap(),
                    );
                }

                // Create sound data
                if let Ok(sound_data) =
                    StreamingSoundData::from_file(&self.music, StreamingSoundSettings::default())
                {
                    if let Some(manager) = self.manager.as_mut() {
                        // Play sound
                        if let Ok(mut handle) = manager.play(sound_data) {
                            handle.set_volume(self.volume / 100.0, Tween::default());
                            self.handle = Some(handle);
                        }
                    }
                }
            }
        }
    }

    fn pause(&mut self) {
        if let Some(handle) = self.handle.as_mut() {
            handle.pause(Tween::default());
        }
    }

    fn set_volume(&mut self, volume: f64) {
        self.volume = volume;

        if let Some(handle) = self.handle.as_mut() {
            handle.set_volume(self.volume / 100.0, Tween::default());
        }
    }

    fn stop(&mut self) {
        if let Some(handle) = self.handle.as_mut() {
            handle.stop(Tween::default());
        }
    }

    fn state(&self) -> AudioStatus {
        self.handle
            .as_ref()
            .map(|handle| handle.state().into())
            .unwrap_or_default()
    }
}
