pub mod pocket;
pub mod tts;
pub mod c_new;
// pub mod macro_log;

fn main() -> Result<(), anyhow::Error> {
    // tts::tts_speak();
    // pocket::pocket_reader();
    c_new::create();
    Ok(())
}
