use alto::{Alto, Stereo, StreamingSource};
use lewton::inside_ogg::OggStreamReader;
use std::fs::File;

use crate::constants;

pub fn init() -> StreamingSource {
    let mut source = OggStreamReader::new(File::open("data/djbLUETOOTH.ogg").unwrap()).unwrap();
    let alto = Alto::load_default().unwrap();
    let device = alto.open(None).unwrap();
    let audio_context = device.new_context(None).unwrap();
    let mut stream = audio_context.new_streaming_source().unwrap();
    let sample_rate = source.ident_hdr.audio_sample_rate as i32;

    while let Ok(Some(mut samples)) = source.read_dec_packet_itl() {
        samples = samples
            .into_iter()
            .map(|s| (s as f32 * constants::VOLUME) as i16)
            .collect();
        let audio_buffer = audio_context
            .new_buffer::<Stereo<i16>, _>(&samples, sample_rate)
            .unwrap();
        stream.queue_buffer(audio_buffer).unwrap();
    }

    stream
}
