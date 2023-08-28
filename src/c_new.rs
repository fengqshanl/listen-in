use std::thread;
use vosk::{Model, SpeakerModel, Recognizer};

const MODEL_PATH: &str = "../model";
// const SPK_MODEL_PATH: &str = "path/to/spk_model";

pub fn create() {
    // 加载语音识别模型
    let model = Model::new(MODEL_PATH).expect("无法加载语音识别模型");
    // let spk_model = SpeakerModel::new(SPK_MODEL_PATH).expect("无法加载说话者识别模型");

    let audio_config: f32 = 16000.0;  // 根据实际情况设置适当的采样率
    // 启动实时语音识别
    let mut recognizer = Recognizer::new(&model, audio_config).unwrap();

    let microphone_thread = thread::spawn(move || {
      loop {
        let result = recognizer.partial_result();
        println!("识别结果: {:?}", result);
      }
    });

    // 等待用户输入，按回车即可退出程序
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("读取用户输入失败");

    microphone_thread.join().expect("麦克风线程意外终止");
}