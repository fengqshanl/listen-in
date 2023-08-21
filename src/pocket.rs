use anyhow::{Ok, Error};
use pocketsphinx;

pub fn pocket_reader() -> Result<(), Error> {
    let ps_config = pocketsphinx::CmdLn::init(true, &["pocketsphinx",
    "-hmm", "data/cmusphinx-en-us-5.2",
    "-lm", "data/cmusphinx-5.0-en-us.lm",
    "-dict", "data/turtle.dic",
    ])?;
    let ps_decoder = pocketsphinx::PsDecoder::init(ps_config);
    ps_decoder.start_utt(Some("utt_id"));
    loop {
        let input_samples: &[i16] = &Vec::new();
        ps_decoder.process_raw(input_samples, false, false);
    }
    ps_decoder.end_utt();
    match ps_decoder.get_hyp() {
        None => println!("Not recognized"),
        Some((hyp, _utt_id, _score)) => println!("Recognized: {}", hyp),
    }
    Ok(())
}
