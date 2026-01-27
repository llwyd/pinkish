use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};

use std::sync::{Arc, RwLock};
use eframe::egui;
use clap::{arg,Command, value_parser};

mod agc;
mod audio_config;
mod audio_magic;
mod biquad;
mod clip;
mod crossover;
mod eq;
mod filter_coeffs_48000;
mod filter_loader;
mod gain;
mod gui;
mod noise;
mod rms;
mod single_pole_lpf;
mod verify;

use crate::agc::AGC;
use crate::eq::Equaliser;
use crate::clip::Clipper;
use crate::crossover::*;
use crate::gain::Gain;
use crate::noise::Noise;
use crate::rms::RMS;
use crate::audio_config::*;
use crate::audio_magic::*;
use crate::gui::*;
use crate::verify::*;
use crate::filter_loader::*;

fn playback(
    mut stereo_noise:[Noise;2],
    mut eq: [Equaliser;2],
    m_gain: Arc<RwLock<Gain>>,
    rms: Arc<RwLock<[RMS;2]>>,
    agc: Arc<RwLock<[AGC;2]>>,
    ) -> eframe::Result
{
    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 240.0])
            .with_resizable(false)
            ,
        ..Default::default()
    };
    
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Host Device error");    
    let config = device.default_output_config().unwrap();


    let num_channels = config.channels() as usize;
    let fs = config.sample_rate() as f32;
    
    assert!(config.sample_format() == cpal::SampleFormat::F32);
    assert!(fs > 0.0);
    assert!(num_channels > 0);
    
    
    let channels:Arc<RwLock<AudioChannels>> = Arc::new(RwLock::new(AudioChannels::Stereo));
    let eq_gain = eq[0].gain();
    let chnls = channels.clone(); 
    let audio_agc = agc.clone();
    let audio_rms = rms.clone();
    let master_gain = m_gain.clone();
    let clipper = Clipper::new(1.0);
    
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        let gain = master_gain.read().unwrap().value();
        let c = chnls.read().unwrap();
        for frame in data.chunks_mut(num_channels)
        {
            let mut stereo_out: [f32;2] = [0.0, 0.0];
            let mut rms_out: [f32;2] = [0.0, 0.0];

            for (idx,sample) in frame.iter_mut().enumerate()
            {
                // Generate white noise
                let inp = stereo_noise[idx].update();
                
                // Filter according to EQ
                let fout = eq[idx].next(inp);

                // Apply gain
                let gout = fout * audio_agc.read().unwrap()[idx].gain();

                // Calculate RMS
                rms_out[idx] = audio_rms.write().unwrap()[idx].next(gout);
              
                // Update gain control
                audio_agc.write().unwrap()[idx].update(rms_out[idx]);

                let vout = gout * gain;
                
                let out = clipper.next(vout);
                if !(out.abs() <= 1.0)
                {
                    println!(" inp: {:?}", inp );
                    println!("fout: {:?}", fout );
                    println!("gout: {:?}", gout );
                    println!(" out: {:?}", out );
                    println!(" rms: {:?}", rms_out[idx] );
                    println!(" agc: {:?}", audio_agc.read().unwrap()[idx].gain() );
                    println!("gain: {:?}", gain );
                    panic!()
                }
                stereo_out[idx] = out;

                match *c
                {
                    AudioChannels::Mono => {*sample = stereo_out[0]},
                    AudioChannels::Stereo => {*sample = stereo_out[idx]}
                }
            }
        }
    },
    move |_err|
    {
    }, 
    None).unwrap();


    stream.play().unwrap();

    eframe::run_native(
        "Pinkish",
        options,
        Box::new(
            |cc|{
                Ok(Box::new(PinkishGUI::new(cc, 
                            m_gain.clone(),
                            eq_gain.clone(),
                            filter_coeffs_48000::PINK_GAIN,
                            channels.clone(),
                            rms.clone(),
                            agc.clone(),
                            fs,
                            )))
            }))
}

fn main() -> eframe::Result{
    let mut mode = OpMode::Playback;

    let matches = Command::new("Pinkish")
        .about("Noise generator with EQ")
        .arg(
            arg!( -v --verify <NUM_SECS> "Verify output, produce x seconds of data")
            .value_parser(value_parser!(u16)))
        .arg(
            arg!( -a --all <NUM_SECS> "Verify output with AGC enabled, produce x seconds of data")
            .value_parser(value_parser!(u16)))
        .arg(
            arg!( -f --filter <NUM_SECS> "Verify filter, produce x seconds of IR data")
            .value_parser(value_parser!(u16)))
        .get_matches();
      
    let mut output_len:u16 = 0;
    match matches.get_one::<u16>("verify")
    {
        Some(x) => 
        { 
            output_len = *x; 
            mode = OpMode::Verify;
        },
        None => {}
    }
    
    match matches.get_one::<u16>("filter")
    {
        Some(x) => 
        { 
            output_len = *x; 
            mode = OpMode::Filter;
        },
        None => {}
    }
    
    match matches.get_one::<u16>("all")
    {
        Some(x) => 
        { 
            output_len = *x; 
            mode = OpMode::VerifyAll;
        },
        None => {}
    }

    let host = cpal::default_host();
    let device = host.default_output_device().expect("Host Device error");    
    let config = device.default_output_config().unwrap();
    let fs = config.sample_rate() as f32;

    // Initialise necessary components
    let stereo_noise = [Noise::new(),Noise::new()]; 
    let band_gain: Arc<RwLock<Vec<f32>>> = Arc::new(RwLock::new(Vec::new()));
    
    for i in 0..NUM_BANDS
    {
        band_gain.write().unwrap().push(filter_coeffs_48000::PINK_GAIN[i]);
    }

    let (co0, co1, co2, co3, co4) = load_filters_48000();
    let lr_eq = [
        Equaliser::new(
                vec![
                        Crossover::new(co0.lpf.clone(), co0.hpf.clone()), 
                        Crossover::new(co1.lpf.clone(), co1.hpf.clone()), 
                        Crossover::new(co2.lpf.clone(), co2.hpf.clone()), 
                        Crossover::new(co3.lpf.clone(), co3.hpf.clone()), 
                        Crossover::new(co4.lpf.clone(), co4.hpf.clone()), 
                    ],
                    band_gain.clone(),
                    fs
                    ),
        Equaliser::new(
                vec![
                        Crossover::new(co0.lpf.clone(), co0.hpf.clone()), 
                        Crossover::new(co1.lpf.clone(), co1.hpf.clone()), 
                        Crossover::new(co2.lpf.clone(), co2.hpf.clone()), 
                        Crossover::new(co3.lpf.clone(), co3.hpf.clone()), 
                        Crossover::new(co4.lpf.clone(), co4.hpf.clone()), 
                    ],
                    band_gain.clone(),
                    fs
                )
    ];

 
    let master_gain = Arc::new(RwLock::new(Gain::new()));
    let gain = master_gain.clone();
    *gain.write().unwrap().ptr() = 0.2;

    let rms = Arc::new(RwLock::new([
            RMS::new(RMS_FREQ_HZ, fs),
            RMS::new(RMS_FREQ_HZ,fs)
    ]));

    let agc = Arc::new(RwLock::new([
            AGC::new(AGC_SET_POINT),
            AGC::new(AGC_SET_POINT)
    ]));

    let result;
    match mode
    {
        OpMode::Playback => 
        {
            result = playback(
                stereo_noise,
                lr_eq,
                master_gain.clone(),
                rms.clone(),
                agc.clone(),
                )
        },
        OpMode::Verify => 
        {
            result = verify(
                stereo_noise,
                lr_eq,
                master_gain.clone(),
                rms.clone(),
                agc.clone(),
                output_len,
                ) 
        },
        OpMode::VerifyAll => 
        {
            result = verify_all(
                stereo_noise,
                lr_eq,
                master_gain.clone(),
                rms.clone(),
                agc.clone(),
                output_len,
                ) 
        },
        OpMode::Filter => 
        {
            result = verify_impulse(
                stereo_noise,
                lr_eq,
                master_gain.clone(),
                rms.clone(),
                agc.clone(),
                output_len,
                ) 
        },
    };
    result
}

