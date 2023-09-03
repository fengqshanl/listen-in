use std::thread;
use std::path::PathBuf;
use std::io; 
use vosk::{ Model, Recognizer };  
use portaudio as pa;

const INTERLEAVED: bool = true;
const LATENCY: pa::Time = 0.0; // Ignored by PortAudio::is_*_format_supported.
const STANDARD_SAMPLE_RATES: [f64; 13] = [
    8000.0, 9600.0, 11025.0, 12000.0, 16000.0, 22050.0, 24000.0, 32000.0, 44100.0, 48000.0,
    88200.0, 96000.0, 192000.0,
];

pub fn create() {
  let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
  config_path.push("model");

  // 加载模型  
  let model = Model::new(config_path.to_str().unwrap()).unwrap();  
  let mut voice = Recognizer::new(&model, 16000.0).unwrap();  

  let pa = pa::PortAudio::new().unwrap();

    let num_devices = pa.device_count().unwrap();
    println!("Number of devices = {}", num_devices);

    println!("Defualt input device: {:?}", pa.default_input_device());
    println!("Defualt output device: {:?}", pa.default_output_device());

    println!("All devices:");
    for device in pa.devices().unwrap() {
        let (idx, info) = device.unwrap();
        println!("--------------------------------------- {:?}", idx);
        println!("{:#?}", &info);

        let in_channels = info.max_input_channels;
        let input_params = pa::StreamParameters::<i16>::new(idx, in_channels, INTERLEAVED, LATENCY);
        let out_channels = info.max_output_channels;
        let output_params =
            pa::StreamParameters::<i16>::new(idx, out_channels, INTERLEAVED, LATENCY);

        println!(
            "Supported standard sample rates for half-duplex 16-bit {} channel input:",
            in_channels
        );
        for &sample_rate in &STANDARD_SAMPLE_RATES {
            if pa
                .is_input_format_supported(input_params, sample_rate)
                .is_ok()
            {
                println!("\t{}hz", sample_rate);
            }
        }

        println!(
            "Supported standard sample rates for half-duplex 16-bit {} channel output:",
            out_channels
        );
        for &sample_rate in &STANDARD_SAMPLE_RATES {
            if pa
                .is_output_format_supported(output_params, sample_rate)
                .is_ok()
            {
                println!("\t{}hz", sample_rate);
            }
        }

        println!("Supported standard sample rates for full-duplex 16-bit {} channel input, {} channel output:",
                 in_channels, out_channels);
        for &sample_rate in &STANDARD_SAMPLE_RATES {
            if pa
                .is_duplex_format_supported(input_params, output_params, sample_rate)
                .is_ok()
            {
                println!("\t{}hz", sample_rate);
            }
        }
    }

  //   println!("请开始说话...");  
  // loop {
  //   stream.read(&mut buffer).unwrap();
  //   voice.accept_waveform(buffer.as_slice());  
  //   let result = voice.result();  
  //   if let Some(text) = result {  
  //       println!("翻译: {}", text);  
  //   }  
  // }
}