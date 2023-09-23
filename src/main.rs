use std::{fs::File, sync::Arc};

use itertools::Itertools;
use rustysynth::{MidiFile, MidiFileSequencer, SoundFont, Synthesizer, SynthesizerSettings};
use tinyaudio::{run_output_device, OutputDeviceParameters};

fn main() {
    let params = OutputDeviceParameters {
        channels_count: 2,
        sample_rate: 44100,
        channel_sample_count: 4410,
    };

    // Buffer for the audio output.
    let mut left: Vec<f32> = vec![0_f32; params.channel_sample_count];
    let mut right: Vec<f32> = vec![0_f32; params.channel_sample_count];

    // Load the SoundFont.
    let mut sf2 = File::open("SHS-10.sf2").unwrap();
    let sound_font = Arc::new(SoundFont::new(&mut sf2).unwrap());

    // Load the MIDI file.
    let mut mid = File::open("test.mid").unwrap();
    let midi_file = Arc::new(MidiFile::new(&mut mid).unwrap());

    // Create the MIDI file sequencer.
    let settings = SynthesizerSettings::new(params.sample_rate as i32);
    let mut synthesizer = Synthesizer::new(&sound_font, &settings).unwrap();
    // let mut sequencer = MidiFileSequencer::new(synthesizer);

    synthesizer.note_on(0, 50, 100);
    synthesizer.note_on(0, 48, 100);
    synthesizer.note_on(0, 46, 100);

    // // Play the MIDI file.
    // sequencer.play(&midi_file, false);

    // Start the audio output.
    let _device = run_output_device(params, {
        move |data| {
            synthesizer.render(&mut left[..], &mut right[..]);
            for (i, value) in left.iter().interleave(right.iter()).enumerate() {
                data[i] = *value;
            }
        }
    })
    .unwrap();

    // Wait for 10 seconds.
    std::thread::sleep(std::time::Duration::from_secs(200000));
}
