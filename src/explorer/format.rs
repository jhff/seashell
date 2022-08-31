use std::ffi::OsStr;

#[derive(Debug, Clone, Copy)]
#[allow(clippy::upper_case_acronyms)]
pub enum SupportedFormat {
    AAC,
    FLAC,
    MP3,
    OGG,
    WAV,
}

impl SupportedFormat {
    const ALL: &[SupportedFormat] = &[
        SupportedFormat::AAC,
        SupportedFormat::FLAC,
        SupportedFormat::MP3,
        SupportedFormat::OGG,
        SupportedFormat::WAV,
    ];
}

impl ToString for SupportedFormat {
    fn to_string(&self) -> String {
        match self {
            SupportedFormat::AAC => "aac".to_string(),
            SupportedFormat::FLAC => "flac".to_string(),
            SupportedFormat::MP3 => "mp3".to_string(),
            SupportedFormat::OGG => "ogg".to_string(),
            SupportedFormat::WAV => "wav".to_string(),
        }
    }
}

impl TryFrom<&OsStr> for SupportedFormat {
    type Error = ();

    fn try_from(path: &OsStr) -> Result<Self, Self::Error> {
        SupportedFormat::ALL
            .iter()
            .find_map(|supported| {
                path.eq_ignore_ascii_case(supported.to_string())
                    .then_some(*supported)
            })
            .ok_or(())
    }
}
