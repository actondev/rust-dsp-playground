use hound;

fn main() {
    let mut reader = hound::WavReader::open("assets/white_noise_mono.wav").unwrap();
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

    let mut writer = hound::WavWriter::create("assets/write.wav", spec).unwrap();
    for s in samples_vec {
        writer.write_sample((s as f32 * 0.1) as i16).unwrap();
    }
    writer.finalize().unwrap();

    println!("count {}", count);
}
