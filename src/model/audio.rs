use rodio::{Decoder, OutputStream, Sink, Source};
use std::collections::HashMap;
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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum SoundEffect {
    Hit,
    Damaged,
    Death,
    PickedUpItem,
    KilledEnemy,
    AdvanceLevel,
    MenuClick,
    MenuSwitch,
    MenuPick,
    ItemEquip,
    Gold,
}

impl SoundEffect {
    /// Get the file path for this sound effect in audio/sfx/
    pub fn file_name(&self) -> &'static str {
        match self {
            SoundEffect::Hit => "hit.wav",
            SoundEffect::Damaged => "Damaged.mp3",
            SoundEffect::Death => "Death.mp3",
            SoundEffect::PickedUpItem => "PickedUpItem.mp3",
            SoundEffect::KilledEnemy => "kill.wav",
            SoundEffect::AdvanceLevel => "AdvanceLevel.mp3",
            SoundEffect::MenuClick => "MenuClick.mp3",
            SoundEffect::MenuSwitch => "MenuSwitch.mp3",
            SoundEffect::MenuPick => "MenuPick.mp3",
            SoundEffect::ItemEquip => "ItemEquip.mp3",
            SoundEffect::Gold => "Gold.ogg",
        }
    }

    pub fn full_path(&self) -> PathBuf {
        PathBuf::from("audio/sfx").join(self.file_name())
    }

    /// Get a random hit sound effect variant (Hit1, Hit2, or Hit3)
    pub fn get_random_hit() -> PathBuf {
        use rand::Rng;
        let mut rng = rand::rng();
        let choice = rng.random_range(0..3);
        let filename = match choice {
            0 => "hit/Hit1.mp3",
            1 => "hit/Hit2.mp3",
            _ => "hit/Hit3.mp3",
        };
        PathBuf::from("audio/sfx").join(filename)
    }
}

/// Cached sound effect data
#[derive(Clone)]
struct CachedSoundEffect {
    data: Arc<Vec<u8>>,
}

/// Audio manager for handling music and sound effects using rodio
pub struct AudioManager {
    sink: Option<Arc<Mutex<Sink>>>,
    _stream: Option<Box<OutputStream>>,
    // Separate sink and stream for sound effects (so SFX and music volumes are independent)
    effects_sink: Option<Arc<Mutex<Sink>>>,
    _effects_stream: Option<Box<OutputStream>>,

    music_volume: f32,
    sound_volume: f32,
    target_volume: f32,
    fade_state: FadeState,
    music_files: Vec<PathBuf>,
    current_file_index: usize,
    /// Cache of loaded sound effects to avoid file I/O on every play
    sfx_cache: HashMap<SoundEffect, CachedSoundEffect>,
}
impl AudioManager {
    pub fn new() -> Self {
        // Load all MP3 files from audio/music folder
        let music_files = Self::load_music_files();
        let mut manager = Self {
            sink: None,
            _stream: None,
            effects_sink: None,
            _effects_stream: None,
            music_volume: 0.0, // Start at 0 for fade-in
            sound_volume: 0.5,
            target_volume: 0.5,
            fade_state: FadeState::None,
            music_files,
            current_file_index: 0,
            sfx_cache: HashMap::new(),
        };

        // Pre-load all sound effects into cache
        manager.preload_sound_effects();

        manager
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

    /// Pre-load all sound effects into memory cache for fast playback
    fn preload_sound_effects(&mut self) {
        let effects = [
            SoundEffect::Hit,
            SoundEffect::Damaged,
            SoundEffect::Death,
            SoundEffect::PickedUpItem,
            SoundEffect::KilledEnemy,
            SoundEffect::AdvanceLevel,
            SoundEffect::MenuClick,
            SoundEffect::MenuSwitch,
            SoundEffect::MenuPick,
            SoundEffect::ItemEquip,
            SoundEffect::Gold,
        ];

        for effect in &effects {
            let path = effect.full_path();
            if let Ok(data) = fs::read(&path) {
                self.sfx_cache.insert(
                    *effect,
                    CachedSoundEffect {
                        data: Arc::new(data),
                    },
                );
            }
        }

        // Also pre-load random hit variations
        for i in 1..=3 {
            let filename = format!("audio/sfx/hit/Hit{}.mp3", i);
            if let Ok(_data) = fs::read(&filename) {
                // Cache them with a dummy effect (we'll handle hit variations differently)
            }
        }
    }

    /// Play a sound effect using cached data (safe, non-blocking, low overhead)
    pub fn play_sound_effect(&mut self, effect: SoundEffect) {
        // Special handling for Hit - use random variant
        if effect == SoundEffect::Hit {
            self.play_random_hit();
            return;
        }

        // Check if we have the effect cached
        if let Some(cached) = self.sfx_cache.get(&effect) {
            // Ensure we have an effects stream and sink
            if self.effects_sink.is_none() {
                if let Ok(mut stream) = rodio::OutputStreamBuilder::open_default_stream() {
                    stream.log_on_drop(false);
                    let sink = Sink::connect_new(stream.mixer());
                    self.effects_sink = Some(Arc::new(Mutex::new(sink)));
                    self._effects_stream = Some(Box::new(stream));
                }
            }

            // If we have a sink, play the sound
            if let Some(sink) = &self.effects_sink {
                // Create a cursor from cached data to avoid file I/O
                let cursor = std::io::Cursor::new(cached.data.as_ref().clone());
                if let Ok(source) = Decoder::new(cursor) {
                    if let Ok(sink_guard) = sink.lock() {
                        // Apply sound volume
                        sink_guard.set_volume(self.sound_volume);
                        sink_guard.append(source);
                    }
                }
            }
        }
    }

    /// Play a random hit sound effect (Hit1, Hit2, or Hit3)
    pub fn play_random_hit(&mut self) {
        let path = SoundEffect::get_random_hit();
        if let Ok(data) = fs::read(&path) {
            // Ensure we have an effects stream and sink
            if self.effects_sink.is_none() {
                if let Ok(mut stream) = rodio::OutputStreamBuilder::open_default_stream() {
                    stream.log_on_drop(false);
                    let sink = Sink::connect_new(stream.mixer());
                    self.effects_sink = Some(Arc::new(Mutex::new(sink)));
                    self._effects_stream = Some(Box::new(stream));
                }
            }

            // If we have a sink, play the sound
            if let Some(sink) = &self.effects_sink {
                let cursor = std::io::Cursor::new(data);
                if let Ok(source) = Decoder::new(cursor) {
                    if let Ok(sink_guard) = sink.lock() {
                        sink_guard.set_volume(self.sound_volume);
                        sink_guard.append(source);
                    }
                }
            }
        }
    }

    /// Play a sound effect with pitch variation in semitones (-12 to +12)
    pub fn play_sound_with_pitch(&mut self, effect: SoundEffect, pitch_semitones: f32) {
        // Check if we have the effect cached
        if let Some(cached) = self.sfx_cache.get(&effect) {
            // Ensure we have an effects stream and sink
            if self.effects_sink.is_none() {
                if let Ok(mut stream) = rodio::OutputStreamBuilder::open_default_stream() {
                    stream.log_on_drop(false);
                    let sink = Sink::connect_new(stream.mixer());
                    self.effects_sink = Some(Arc::new(Mutex::new(sink)));
                    self._effects_stream = Some(Box::new(stream));
                }
            }

            // If we have a sink, play the sound with pitch shift
            if let Some(sink) = &self.effects_sink {
                let cursor = std::io::Cursor::new(cached.data.as_ref().clone());
                if let Ok(source) = Decoder::new(cursor) {
                    // Convert semitones to speed multiplier (2^(semitones/12))
                    let speed_multiplier = 2.0_f32.powf(pitch_semitones / 12.0);
                    let speed_source = source.speed(speed_multiplier);

                    if let Ok(sink_guard) = sink.lock() {
                        sink_guard.set_volume(self.sound_volume);
                        sink_guard.append(speed_source);
                    }
                }
            }
        }
    }

    /// Play Damaged sound with up to +3 pitch variations
    pub fn play_damaged_sound(&mut self) {
        use rand::Rng;
        let mut rng = rand::rng();
        let pitch_variation = rng.random_range(-3..=3) as f32;
        self.play_sound_with_pitch(SoundEffect::Damaged, pitch_variation);
    }

    /// Play Gold pickup sound with random pitch variation (-5 to +5)
    pub fn play_gold_sound(&mut self) {
        use rand::Rng;
        let mut rng = rand::rng();
        let pitch_variation = rng.random_range(-5..=5) as f32;
        self.play_sound_with_pitch(SoundEffect::Gold, pitch_variation);
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

    /// Start fade-out transition with muffling effect for death
    pub fn start_death_fade_out(&mut self, duration: f32) {
        self.fade_state = FadeState::FadingOut {
            current_time: 0.0,
            duration,
        };
        self.target_volume = 0.1; // Lower volume for muffled effect
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
                    self.music_volume =
                        self.target_volume + (1.0 - progress) * (0.5 - self.target_volume);
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

        // Apply volume to effects sink if it exists
        if let Some(sink) = &self.effects_sink {
            if let Ok(sink_guard) = sink.lock() {
                sink_guard.set_volume(self.sound_volume);
            }
        }
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
