use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};

use std::sync::{Arc, RwLock};
use eframe::egui;

mod agc;
mod audio_config;
mod audio_magic;
mod biquad;
mod crossover;
mod eq;
mod filter_coeffs_48000;
mod gain;
mod gui;
mod noise;
mod rms;
mod single_pole_lpf;

use crate::agc::AGC;
use crate::eq::Equaliser;
use crate::crossover::*;
use crate::gain::Gain;
use crate::noise::Noise;
use crate::biquad::Biquad;
use crate::rms::RMS;
use crate::audio_config::*;
use crate::gui::*;

fn load_filters() -> (CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads)
{
    let mut filter_0_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_1_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_2_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_3_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_4_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    for i in 0..filter_coeffs_48000::FILTER_0_LPF.len()
    {
        filter_0_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_0_LPF[i][0],
                    filter_coeffs_48000::FILTER_0_LPF[i][1],
                    filter_coeffs_48000::FILTER_0_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_0_LPF[i][4],
                    filter_coeffs_48000::FILTER_0_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_0_HPF.len()
    {
        filter_0_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_0_HPF[i][0],
                    filter_coeffs_48000::FILTER_0_HPF[i][1],
                    filter_coeffs_48000::FILTER_0_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_0_HPF[i][4],
                    filter_coeffs_48000::FILTER_0_HPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_1_LPF.len()
    {
        filter_1_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_1_LPF[i][0],
                    filter_coeffs_48000::FILTER_1_LPF[i][1],
                    filter_coeffs_48000::FILTER_1_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_1_LPF[i][4],
                    filter_coeffs_48000::FILTER_1_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_1_HPF.len()
    {
        filter_1_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_1_HPF[i][0],
                    filter_coeffs_48000::FILTER_1_HPF[i][1],
                    filter_coeffs_48000::FILTER_1_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_1_HPF[i][4],
                    filter_coeffs_48000::FILTER_1_HPF[i][5]
                ])
            );
    }
    
    /* Filter 2 */
    for i in 0..filter_coeffs_48000::FILTER_2_LPF.len()
    {
        filter_2_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_2_LPF[i][0],
                    filter_coeffs_48000::FILTER_2_LPF[i][1],
                    filter_coeffs_48000::FILTER_2_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_2_LPF[i][4],
                    filter_coeffs_48000::FILTER_2_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_2_HPF.len()
    {
        filter_2_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_2_HPF[i][0],
                    filter_coeffs_48000::FILTER_2_HPF[i][1],
                    filter_coeffs_48000::FILTER_2_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_2_HPF[i][4],
                    filter_coeffs_48000::FILTER_2_HPF[i][5]
                ])
            );
    }
    
    /* Filter 3 */
    for i in 0..filter_coeffs_48000::FILTER_3_LPF.len()
    {
        filter_3_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_3_LPF[i][0],
                    filter_coeffs_48000::FILTER_3_LPF[i][1],
                    filter_coeffs_48000::FILTER_3_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_3_LPF[i][4],
                    filter_coeffs_48000::FILTER_3_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_3_HPF.len()
    {
        filter_3_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_3_HPF[i][0],
                    filter_coeffs_48000::FILTER_3_HPF[i][1],
                    filter_coeffs_48000::FILTER_3_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_3_HPF[i][4],
                    filter_coeffs_48000::FILTER_3_HPF[i][5]
                ])
            );
    }

    /* Filter 4 */
    for i in 0..filter_coeffs_48000::FILTER_4_LPF.len()
    {
        filter_4_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_4_LPF[i][0],
                    filter_coeffs_48000::FILTER_4_LPF[i][1],
                    filter_coeffs_48000::FILTER_4_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_4_LPF[i][4],
                    filter_coeffs_48000::FILTER_4_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_4_HPF.len()
    {
        filter_4_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_4_HPF[i][0],
                    filter_coeffs_48000::FILTER_4_HPF[i][1],
                    filter_coeffs_48000::FILTER_4_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_4_HPF[i][4],
                    filter_coeffs_48000::FILTER_4_HPF[i][5]
                ])
            );
    }
    

    (
    filter_0_bq.clone(),
    filter_1_bq.clone(),
    filter_2_bq.clone(),
    filter_3_bq.clone(),
    filter_4_bq.clone(),
    )
}

fn main() -> eframe::Result{

    let num_bands = 6;
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

    assert!(config.sample_format() == cpal::SampleFormat::F32);

    let num_channels = config.channels() as usize;
    let fs = config.sample_rate() as f32;

    let channels:Arc<RwLock<AudioChannels>> = Arc::new(RwLock::new(AudioChannels::Stereo));

    let mut stereo_noise = [Noise::new(),Noise::new()]; 

    let band_gain: Arc<RwLock<Vec<f32>>> = Arc::new(RwLock::new(Vec::new()));
    
    for i in 0..num_bands
    {
        band_gain.write().unwrap().push(filter_coeffs_48000::PINK_GAIN[i]);
    }

    let (co0, co1, co2, co3, co4) = load_filters();
    let mut lr_eq = [
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

    let eq_gain = lr_eq[0].gain();
    
    let master_gain = Arc::new(RwLock::new(Gain::new()));
    let gain = master_gain.clone();
    *gain.write().unwrap().ptr() = 0.2;
    let chnls = channels.clone();

    let rms_freq = 1.0;
    let rms = Arc::new(RwLock::new([RMS::new(rms_freq, fs),RMS::new(rms_freq,fs)]));
    let audio_rms = rms.clone();
    let set_point = 0.1;
    let agc = Arc::new(RwLock::new([
        AGC::new(set_point),
        AGC::new(set_point)
    ]));
   
    let audio_agc = agc.clone();
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        let master_gain = gain.read().unwrap().value();
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
                let fout = lr_eq[idx].next(inp);

                // Apply gain
                let gout = fout * audio_agc.read().unwrap()[idx].gain();

                // Calculate RMS
                rms_out[idx] = audio_rms.write().unwrap()[idx].next(gout);
              
                // Update gain control
                audio_agc.write().unwrap()[idx].update(rms_out[idx]);

                let out = gout * master_gain;
                if !(out <= 1.0) || !(out >= -1.0)
                {
                    println!(" inp: {:?}", inp );
                    println!("fout: {:?}", fout );
                    println!("gout: {:?}", gout );
                    println!(" out: {:?}", out );
                    println!(" rms: {:?}", rms_out[idx] );
                    println!(" agc: {:?}", audio_agc.read().unwrap()[idx].gain() );
                    println!("gain: {:?}", master_gain );
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
                            master_gain.clone(),
                            eq_gain.clone(),
                            filter_coeffs_48000::PINK_GAIN,
                            channels.clone(),
                            rms.clone(),
                            agc.clone()
                            )))
            }))
}

