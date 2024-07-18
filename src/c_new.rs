use std::sync::{Arc, Mutex};
use std::path::PathBuf;
use std::io;
// use crate::macro_log::console_log;
use vosk::{ Model, Recognizer };  
use std::collections::VecDeque;
use weresocool_portaudio::{ PortAudio, DuplexStreamSettings, Stream  };
use std::i16;
use std::slice;
use std::time::SystemTime;

const SAMPLE_RATE: f64 = 16000.0;
const CHANNELS: i32 = 1;
const FRAMES: u32 = 1280;
const STANDARD_SAMPLE_RATES: [f64; 13] = [
    8000.0, 9600.0, 11025.0, 12000.0, 16000.0, 22050.0, 24000.0, 32000.0, 44100.0, 48000.0,
    88200.0, 96000.0, 192000.0,
];

pub fn create() {
  let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  config_path.push("model");
  println!("config_path: {:?}", config_path.to_str().unwrap());
  // 加载模型  
  let model = Model::new(config_path.to_str().unwrap()).unwrap();  
  let mut voice = Recognizer::new(&model, 16000.0).unwrap();  
  
  let audio_self = PortAudio::new().unwrap();
  
  let settings = PortAudio::default_input_stream_settings::<i16>(
        &audio_self,
        CHANNELS,
        SAMPLE_RATE,            
        FRAMES,              
    ).unwrap();

  // 打开音频输入流
  let mut stream = PortAudio::open_blocking_stream(&audio_self, settings).unwrap();

  stream.start().unwrap();
  loop {
    let buf = stream.read(16).unwrap();
    voice.accept_waveform(buf);  
    let result = voice.result().single().unwrap();
    if result.text.len() > 0 {
        // console_log!("Hello {}!", "world");
      println!("text: {}", result.text); 
    } 
  }
}