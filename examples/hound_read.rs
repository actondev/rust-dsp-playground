use dsp_playground::biquad;
use hound;
use std::i16;

fn main() {
    let mut reader = hound::WavReader::open("tests/assets/white_noise_mono.wav").unwrap();
    let samples = reader.samples::<i16>();
    let mut samples_vec: Vec<i16> = Vec::new();

    let mut count = 0;
    {
        let sampels_vec2 = &mut samples_vec;
        samples.for_each(|s| {
            count += 1;
            sampels_vec2.push(s.unwrap());
        });
    }

    println!("count vec {}", samples_vec.len());

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = hound::WavWriter::create("tests/assets/white.wav", spec).unwrap();
    for s in &samples_vec {
        writer.write_sample((*s as f32 * 1.0) as i16).unwrap();
    }
    writer.finalize().unwrap();

    println!("count {}", count);

    filter_file(&samples_vec);
    write_file(&samples_vec);
}

fn filter_file(samples: &Vec<i16>) {
    let biquad_params = biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6;
    let mut biquad_process = biquad::Process::new(biquad_params);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("tests/assets/filtered.wav", spec).unwrap();

    for s in samples {
        let sout = biquad_process.process(s);
        writer.write_sample(sout).unwrap();
    }
}

fn write_file(samples: &Vec<i16>) {

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("tests/assets/white_noise2.wav", spec).unwrap();

    for s in samples {
        writer.write_sample(*s).unwrap();
    }
}