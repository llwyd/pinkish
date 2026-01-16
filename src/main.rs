use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};

use std::sync::{Arc, RwLock};
use eframe::egui;
use egui::{Align,Layout,Slider,SliderOrientation};

mod eq;
mod noise;
mod voss;
mod gain;
mod biquad;
mod filterbank;
mod crossover;
mod filter_coeffs_48000;
mod rms;

use crate::eq::Equaliser;
use crate::crossover::*;
use crate::gain::Gain;
use crate::noise::Noise;
use crate::biquad::Biquad;
use crate::rms::RMS;
//const DEFAULT_CROSSOVER_GAIN:f32 = 0.70795; 

const VOLUME_SLIDER_MAX:f32 = 0.70795;
const NOISE_PRESCALAR:f32 = 1.0; //0.707;
const DEFAULT_CROSSOVER_GAIN:f32 = 1.0;
const GAIN_SLIDER_MAX:f32 = DEFAULT_CROSSOVER_GAIN;
const GAIN_SLIDER_MIN:f32 = 0.001;
const GAIN_SLIDER_INC:f64 = 0.00001;

#[derive(Clone,Debug,PartialEq)]
enum AudioChannels
{
    Mono,
    Stereo,
}

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
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let g = Arc::new(RwLock::new(Gain::new()));
    
    let host = cpal::default_host();
    let device = host.default_output_device().expect("Host Device error");    
    let config = device.default_output_config().unwrap();

    assert!(config.sample_format() == cpal::SampleFormat::F32);

    let num_channels = config.channels() as usize;
    let fs = config.sample_rate() as f32;
    println!("Channels: {}", num_channels);
    println!("fs: {fs}");

    let channels:Arc<RwLock<AudioChannels>> = Arc::new(RwLock::new(AudioChannels::Stereo));

    let mut stereo_noise = [Noise::new(),Noise::new()]; 

    let band_gain: Arc<RwLock<Vec<f32>>> = Arc::new(RwLock::new(Vec::new()));
    
    for _i in 0..num_bands
    {
        band_gain.write().unwrap().push(DEFAULT_CROSSOVER_GAIN);
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
                    band_gain.clone()
                ),
        Equaliser::new(
                vec![
                        Crossover::new(co0.lpf.clone(), co0.hpf.clone()), 
                        Crossover::new(co1.lpf.clone(), co1.hpf.clone()), 
                        Crossover::new(co2.lpf.clone(), co2.hpf.clone()), 
                        Crossover::new(co3.lpf.clone(), co3.hpf.clone()), 
                        Crossover::new(co4.lpf.clone(), co4.hpf.clone()), 
                    ],
                    band_gain.clone()
                )
    ];

    let eq_gain = lr_eq[0].gain();
    /*
    eq_gain.write().unwrap()[0] = filter_coeffs_48000::PINK_GAIN[0];
    eq_gain.write().unwrap()[1] = filter_coeffs_48000::PINK_GAIN[1];
    eq_gain.write().unwrap()[2] = filter_coeffs_48000::PINK_GAIN[2];
    eq_gain.write().unwrap()[3] = filter_coeffs_48000::PINK_GAIN[3];
    eq_gain.write().unwrap()[4] = filter_coeffs_48000::PINK_GAIN[4];
    eq_gain.write().unwrap()[5] = filter_coeffs_48000::PINK_GAIN[5];
*/
    let value = g.clone();
    *value.write().unwrap().ptr() = 0.0;
    let chnls = channels.clone();

    let rms_freq = 1000.0;
    let mut rms = Arc::new(RwLock::new([RMS::new(rms_freq, fs),RMS::new(rms_freq,fs)]));

    let audio_rms = rms.clone();
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        let master_gain = value.read().unwrap().value();
        let c = chnls.read().unwrap();
        for frame in data.chunks_mut(num_channels)
        {
        //    let n = noise.update();

        //    let next = lr_eq.next(n);
            let mut stereo_out: [f32;2] = [0.0, 0.0];
            let mut rms_out: [f32;2] = [0.0, 0.0];
            for (idx,sample) in frame.iter_mut().enumerate()
            {
                
                /* Scale the noise source to avoid overflow due to float
                 * maths
                 */
                let inp = stereo_noise[idx].update() * NOISE_PRESCALAR;
                
                let fout = lr_eq[idx].next(inp);
              
                let out = fout * master_gain;
                if !(fout <= 1.0) || !(fout >= -1.0)
                {
                    println!(" inp: {:?}", inp );
                    println!("fout: {:?}", fout );
                    println!(" out: {:?}", out );
                    println!("gain: {:?}", master_gain );
                    panic!()
                }
                stereo_out[idx] = out;

                rms_out[idx] = audio_rms.write().unwrap()[idx].next(fout);
                match *c
                {
                    AudioChannels::Mono => {*sample = stereo_out[0]},
                    AudioChannels::Stereo => {*sample = stereo_out[idx]}
                }
            }
            //println!("RMS: ({:?}, {:?})", rms_out[0],rms_out[1]);
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
                Ok(Box::new(PinkishApp::new(cc, 
                            g.clone(),
                            eq_gain.clone(),
                            filter_coeffs_48000::PINK_GAIN,
                            channels.clone(),
                            rms.clone()
                            )))
            }))
}

struct PinkishApp {
    gain:Arc<RwLock<Gain>>,
    eq_gain:Arc<RwLock<Vec<f32>>>,
    pink_gain: [f32;6],
    channels: Arc<RwLock<AudioChannels>>,
    rms: Arc<RwLock<[RMS;2]>>,
}

impl PinkishApp{
    fn new(_cc: &eframe::CreationContext<'_>,
        gain: Arc<RwLock<Gain>>,
        eq_gain:Arc<RwLock<Vec<f32>>>,
        pink_gain: [f32;6],
        channels: Arc<RwLock<AudioChannels>>,
        rms: Arc<RwLock<[RMS;2]>>,
        ) -> Self{
        Self{
            gain: gain.clone(),
            eq_gain: eq_gain.clone(),
            pink_gain,
            channels,
            rms: rms.clone(),
        }
    }
}

impl eframe::App for PinkishApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Pink-ish");
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                if ui.button("Stop").clicked(){
                    self.gain.write().unwrap().silence();
                }
                if ui.button("Start").clicked(){
                    self.gain.write().unwrap().resume();
                }
                if ui.button("Pink").clicked(){
                    self.eq_gain.write().unwrap()[0] = self.pink_gain[0];
                    self.eq_gain.write().unwrap()[1] = self.pink_gain[1];
                    self.eq_gain.write().unwrap()[2] = self.pink_gain[2];
                    self.eq_gain.write().unwrap()[3] = self.pink_gain[3];
                    self.eq_gain.write().unwrap()[4] = self.pink_gain[4];
                    self.eq_gain.write().unwrap()[5] = self.pink_gain[5];
                }
                ui.radio_value(&mut *self.channels.write().unwrap(), AudioChannels::Mono, "Mono");
                ui.radio_value(&mut *self.channels.write().unwrap(), AudioChannels::Stereo, "Stereo");
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                ui.add(
                    Slider::new(&mut *self.gain.write().unwrap().ptr(), 0.0..=VOLUME_SLIDER_MAX)
                    .text("Gain")
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.1)
                    .show_value(false)
                    );
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[0], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    );
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[1], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    );
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[2], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    );
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[3], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    );
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[4], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    );
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[5], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    );
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                ui.label(format!("L-RMS: {}", self.rms.read().unwrap()[0].value()));
                ui.label(format!("L-RMS: {}", self.rms.read().unwrap()[1].value()));
            });
        });
    }
}

