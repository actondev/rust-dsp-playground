#[macro_use]
extern crate more_asserts;

mod helper;
mod common;

use dsp_playground::biquad;

const PATH_WHITE_NOISE: &str = "tests/assets/white_noise_mono.wav";
const PATH_SNAPSHOT_LOWPASS: &str = "tests/assets/snapshot_lowpass_fc_1000_Q_0.7071_gain_6.wav";

#[test]
fn self_rmse_is_0() {
    let white_noise : Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let rmse = helper::rmse(&white_noise, &white_noise);
    assert_eq!(rmse, 0.0);
}

#[test]
fn not_self_rmse() {
    let white_noise : Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered : Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);
    let rmse = helper::rmse(&white_noise, &white_noise_filtered);
    assert_gt!(rmse, 4_000.0);
}

#[test]
/**
 * Self correlation index should be 1
 * 
 * This serves as a basis for our following tests
 */
fn self_cci_is_1() {
    let white_noise : Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let cci = helper::cross_correlation_index(&white_noise, &white_noise);
    // println!("cci {}", cci);
    assert_eq!(cci, 1.0);
}

#[test]
fn not_self_cci_less_than_1() {
    let white_noise : Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered : Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);
    let cci = helper::cross_correlation_index(&white_noise, &white_noise_filtered);
    // println!("cci {}", cci);
    assert_lt!(cci, 0.1);
}

#[test]
fn low_pass_snaphost() {
    let white_noise : Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered_snapshot : Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);

    let biquad_params = biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6;
    let mut biquad_process = biquad::Process::new(biquad_params);

    let mut filtered: Vec<i16> = Vec::new();

    for s in &white_noise {
        let sout = biquad_process.process(s);
        filtered.push(sout);
    }

    let cci = helper::cross_correlation_index(&white_noise_filtered_snapshot, &filtered);
    assert_gt!(cci, 0.9999);

    let rmse = helper::rmse(&white_noise_filtered_snapshot, &filtered);
    assert_lt!(rmse, 1.0);
}
#[test]
fn write_low_pass_filtered_file() {
    common::cleanup_temp_files();
    let white_noise : Vec<i16> = helper::audio_file_samples(PATH_WHITE_NOISE);
    let white_noise_filtered_snapshot : Vec<i16> = helper::audio_file_samples(PATH_SNAPSHOT_LOWPASS);

    let biquad_params = biquad::LOWPASS_FC_1000_Q_0_7071_GAIN_6;
    let mut biquad_process = biquad::Process::new(biquad_params);

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let filtered_out_path = "tests/assets/temp_lowpass_fc_1000_Q_0.7071_gain_6.wav";
    let mut writer = hound::WavWriter::create(filtered_out_path, spec).unwrap();

    for s in &white_noise {
        let sout = biquad_process.process(s);
        writer.write_sample(sout).unwrap();
    }

    writer.finalize().unwrap();

    let filtered : Vec<i16> = helper::audio_file_samples(filtered_out_path);

    let cci = helper::cross_correlation_index(&white_noise_filtered_snapshot, &filtered);
    // println!("cci {}", cci);
    assert_gt!(cci, 0.9999);
}