use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::{self, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// Fade state for music transitions
#[derive(Clone, Copy, Debug)]
pub enum FadeState {
    None,
    FadingIn { current_time: f32, duration: f32 },
    FadingOut { current_time: f32, duration: f32 },
}

/// Audio manager for handling music and sound effects using rodio
pub struct AudioManager {
    sink: Option<Arc<Mutex<Sink>>>,
    _stream: Option<Box<OutputStream>>,
    music_volume: f32,
    sound_volume: f32,
    target_volume: f32,
    fade_state: FadeState,
    music_files: Vec<PathBuf>,
    current_file_index: usize,
}

impl AudioManager {
    pub fn new() -> Self {
        // Load all MP3 files from audio/music folder
        let music_files = Self::load_music_files();

        Self {
            sink: None,
            _stream: None,
            music_volume: 0.0,  // Start at 0 for fade-in
            sound_volume: 0.5,
            target_volume: 0.5,
            fade_state: FadeState::None,
            music_files,
            current_file_index: 0,
        }
    }

    /// Load all MP3 files from audio/music directory
    fn load_music_files() -> Vec<PathBuf> {
        let music_dir = PathBuf::from("audio/music");
        let mut files = Vec::new();

        if let Ok(entries) = fs::read_dir(&music_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| {
                    ext.eq_ignore_ascii_case("mp3")
                        || ext.eq_ignore_ascii_case("wav")
                        || ext.eq_ignore_ascii_case("flac")
                        || ext.eq_ignore_ascii_case("ogg")
                }) {
                    files.push(path);
                }
            }
        }

        if files.is_empty() {
            // Fallback to darkhelm.mp3 if no music folder
            files.push(PathBuf::from("audio/darkhelm.mp3"));
        }

        files
    }

    /// Start playing music with fade-in
    pub fn start_music_with_fade_in(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.music_files.is_empty() {
            return Err("No music files found".into());
        }

        // Get current music file
        let path = &self.music_files[self.current_file_index];

        // Create output stream using default device
        let mut stream = rodio::OutputStreamBuilder::open_default_stream()?;
        stream.log_on_drop(false);

        // Create sink from the mixer
        let sink = Sink::connect_new(stream.mixer());

        // Open file
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        // Decode audio file
        let source = Decoder::new(reader)?;

        // Loop infinitely
        let looped_source = source.repeat_infinite();

        // Add to sink
        sink.append(looped_source);

        // Start at 0 volume for fade-in
        sink.set_volume(0.0);
        sink.play();

        // Store sink for volume control
        self.sink = Some(Arc::new(Mutex::new(sink)));
        self._stream = Some(Box::new(stream));

        // Start fade-in (3 second duration)
        self.fade_state = FadeState::FadingIn {
            current_time: 0.0,
            duration: 3.0,
        };
        self.target_volume = 0.5;

        Ok(())
    }

    /// Start fade-out transition
    pub fn start_fade_out(&mut self, duration: f32) {
        self.fade_state = FadeState::FadingOut {
            current_time: 0.0,
            duration,
        };
        self.target_volume = 0.1;  // Lower target for muffled effect
    }

    /// Start fade-in transition
    pub fn start_fade_in(&mut self, duration: f32, target: f32) {
        self.fade_state = FadeState::FadingIn {
            current_time: 0.0,
            duration,
        };
        self.target_volume = target;
    }

    /// Update fade transitions (call every frame)
    pub fn update(&mut self, delta_time: f32) {
        match self.fade_state {
            FadeState::FadingIn {
                mut current_time,
                duration,
            } => {
                current_time += delta_time;
                if current_time >= duration {
                    // Fade complete
                    self.music_volume = self.target_volume;
                    self.fade_state = FadeState::None;
                } else {
                    // Interpolate volume
                    let progress = current_time / duration;
                    self.music_volume = progress * self.target_volume;
                }

                // Apply current volume
                if let Some(sink) = &self.sink {
                    if let Ok(sink_guard) = sink.lock() {
                        sink_guard.set_volume(self.music_volume);
                    }
                }

                // Update state
                self.fade_state = FadeState::FadingIn {
                    current_time,
                    duration,
                };
            }
            FadeState::FadingOut {
                mut current_time,
                duration,
            } => {
                current_time += delta_time;
                if current_time >= duration {
                    // Fade complete
                    self.music_volume = self.target_volume;
                    self.fade_state = FadeState::None;
                } else {
                    // Interpolate volume (fade out from current to target)
                    let progress = current_time / duration;
                    self.music_volume = self.target_volume + (1.0 - progress) * (0.5 - self.target_volume);
                }

                // Apply current volume
                if let Some(sink) = &self.sink {
                    if let Ok(sink_guard) = sink.lock() {
                        sink_guard.set_volume(self.music_volume);
                    }
                }

                // Update state
                self.fade_state = FadeState::FadingOut {
                    current_time,
                    duration,
                };
            }
            FadeState::None => {}
        }
    }

    /// Set music volume (0.0 to 1.0) - changes volume of currently playing track
    pub fn set_music_volume(&mut self, volume: f32) {
        let volume = volume.clamp(0.0, 1.0);
        self.music_volume = volume;
        self.target_volume = volume;
        self.fade_state = FadeState::None;

        // Apply volume to current sink if playing
        if let Some(sink) = &self.sink {
            if let Ok(sink_guard) = sink.lock() {
                sink_guard.set_volume(volume);
            }
        }
    }

    /// Get current music volume
    pub fn get_music_volume(&self) -> f32 {
        self.music_volume
    }

    /// Set sound effects volume (0.0 to 1.0)
    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume.clamp(0.0, 1.0);
    }

    /// Get current sound effects volume
    pub fn get_sound_volume(&self) -> f32 {
        self.sound_volume
    }

    /// Stop background music
    pub fn stop_music(&mut self) {
        if let Some(sink) = &self.sink {
            if let Ok(sink_guard) = sink.lock() {
                sink_guard.stop();
            }
        }
        self.sink = None;
        self._stream = None;
        self.fade_state = FadeState::None;
    }

    /// Check if music is playing
    pub fn is_music_playing(&self) -> bool {
        if let Some(sink) = &self.sink {
            if let Ok(sink_guard) = sink.lock() {
                return !sink_guard.is_paused();
            }
        }
        false
    }

    /// Pause music
    pub fn pause_music(&mut self) {
        if let Some(sink) = &self.sink {
            if let Ok(sink_guard) = sink.lock() {
                sink_guard.pause();
            }
        }
    }

    /// Resume music
    pub fn resume_music(&mut self) {
        if let Some(sink) = &self.sink {
            if let Ok(sink_guard) = sink.lock() {
                sink_guard.play();
            }
        }
    }
}

impl Default for AudioManager {
    fn default() -> Self {
        Self::new()
    }
}
