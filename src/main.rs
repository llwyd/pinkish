use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};

use std::sync::{Arc, RwLock};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;
use eframe::egui;
use egui::{Align,Layout,Slider,SliderOrientation};
use serde::Deserialize;
use toml::{Value, de::Error};

mod noise;
mod voss;
mod gain;
mod biquad;
mod filterbank;

use crate::gain::Gain;
use crate::noise::Noise;
use crate::biquad::Biquad;
use crate::filterbank::FilterBank;

#[derive(Deserialize, Debug)]
struct FilterConfig{
    fs: u32,
    filter0: Vec<Vec<f32>>,
    filter1: Vec<Vec<f32>>,
    filter2: Vec<Vec<f32>>,
    filter3: Vec<Vec<f32>>,
    filter4: Vec<Vec<f32>>,
    filter5: Vec<Vec<f32>>,
}

fn main() -> eframe::Result{

    let mut config_file = File::open("autogen/filters.toml").expect("Failed to open config file");
    let mut config_str = String::new();
    config_file.read_to_string(&mut config_str).expect("Failed to read TOML config");

    let fc:FilterConfig = toml::from_str(&config_str).expect("Failed to parse TOML config");
    
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
    println!("Channels: {}", num_channels);
    let bq_gain0 = Arc::new(RwLock::new(0.5));
    let bq_gain1 = Arc::new(RwLock::new(0.5));
    let bq_gain2 = Arc::new(RwLock::new(0.5));
    let bq_gain3 = Arc::new(RwLock::new(0.5));
    let bq_gain4 = Arc::new(RwLock::new(0.5));
    let bq_gain5 = Arc::new(RwLock::new(0.5));

    let mut stereo_noise = [Noise::new(),Noise::new()]; 
    let mut stereo_eq =
        [[
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter0[0][0], fc.filter0[0][1], fc.filter0[0][2]],
                    [fc.filter0[0][4], fc.filter0[0][5]])],
                    bq_gain0.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter1[0][0], fc.filter1[0][1], fc.filter1[0][2]],
                    [fc.filter1[0][4], fc.filter1[0][5]]),
                    Biquad::new(
                    [fc.filter1[1][0], fc.filter1[1][1], fc.filter1[1][2]],
                    [fc.filter1[1][4], fc.filter1[1][5]])],
                    bq_gain1.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter2[0][0], fc.filter2[0][1], fc.filter2[0][2]],
                    [fc.filter2[0][4], fc.filter2[0][5]]),
                    Biquad::new(
                    [fc.filter2[1][0], fc.filter2[1][1], fc.filter2[1][2]],
                    [fc.filter2[1][4], fc.filter2[1][5]])],
                    bq_gain2.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter3[0][0], fc.filter3[0][1], fc.filter3[0][2]],
                    [fc.filter3[0][4], fc.filter3[0][5]]),
                    Biquad::new(
                    [fc.filter3[1][0], fc.filter3[1][1], fc.filter3[1][2]],
                    [fc.filter3[1][4], fc.filter3[1][5]])],
                    bq_gain3.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter4[0][0], fc.filter4[0][1], fc.filter4[0][2]],
                    [fc.filter4[0][4], fc.filter4[0][5]]),
                    Biquad::new(
                    [fc.filter4[1][0], fc.filter4[1][1], fc.filter4[1][2]],
                    [fc.filter4[1][4], fc.filter4[1][5]])],
                    bq_gain4.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter5[0][0], fc.filter5[0][1], fc.filter5[0][2]],
                    [fc.filter5[0][4], fc.filter5[0][5]])],
                    bq_gain5.clone()), 
        ],
        [
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter0[0][0], fc.filter0[0][1], fc.filter0[0][2]],
                    [fc.filter0[0][4], fc.filter0[0][5]])],
                    bq_gain0.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter1[0][0], fc.filter1[0][1], fc.filter1[0][2]],
                    [fc.filter1[0][4], fc.filter1[0][5]]),
                    Biquad::new(
                    [fc.filter1[1][0], fc.filter1[1][1], fc.filter1[1][2]],
                    [fc.filter1[1][4], fc.filter1[1][5]])],
                    bq_gain1.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter2[0][0], fc.filter2[0][1], fc.filter2[0][2]],
                    [fc.filter2[0][4], fc.filter2[0][5]]),
                    Biquad::new(
                    [fc.filter2[1][0], fc.filter2[1][1], fc.filter2[1][2]],
                    [fc.filter2[1][4], fc.filter2[1][5]])],
                    bq_gain2.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter3[0][0], fc.filter3[0][1], fc.filter3[0][2]],
                    [fc.filter3[0][4], fc.filter3[0][5]]),
                    Biquad::new(
                    [fc.filter3[1][0], fc.filter3[1][1], fc.filter3[1][2]],
                    [fc.filter3[1][4], fc.filter3[1][5]])],
                    bq_gain3.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter4[0][0], fc.filter4[0][1], fc.filter4[0][2]],
                    [fc.filter4[0][4], fc.filter4[0][5]]),
                    Biquad::new(
                    [fc.filter4[1][0], fc.filter4[1][1], fc.filter4[1][2]],
                    [fc.filter4[1][4], fc.filter4[1][5]])],
                    bq_gain4.clone()),
            FilterBank::new(
                vec![Biquad::new(
                    [fc.filter5[0][0], fc.filter5[0][1], fc.filter5[0][2]],
                    [fc.filter5[0][4], fc.filter5[0][5]])],
                    bq_gain5.clone()), 
        ]];
        
    let value = g.clone();
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        let master_gain = value.write().unwrap().value();
        
        for frame in data.chunks_mut(num_channels)
        {
            for (idx,sample) in frame.iter_mut().enumerate()
            {
                let n = stereo_noise[idx].update();
                let mut next = 0.0;
                
                for e in &mut stereo_eq[idx]{
                    next += e.next(n);
                }
                
                *sample = next * master_gain;
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
                Ok(Box::new(PinkishApp::new(cc, 
                            g.clone(),
                            bq_gain0.clone(),
                            bq_gain1.clone(),
                            bq_gain2.clone(),
                            bq_gain3.clone(),
                            bq_gain4.clone(),
                            bq_gain5.clone()
                            )))
            }))
}

struct PinkishApp {
    gain:Arc<RwLock<Gain>>,
    bq_gain0:Arc<RwLock<f32>>,
    bq_gain1:Arc<RwLock<f32>>,
    bq_gain2:Arc<RwLock<f32>>,
    bq_gain3:Arc<RwLock<f32>>,
    bq_gain4:Arc<RwLock<f32>>,
    bq_gain5:Arc<RwLock<f32>>,
}

impl PinkishApp{
    fn new(_cc: &eframe::CreationContext<'_>,
        gain: Arc<RwLock<Gain>>,
        bq_gain0: Arc<RwLock<f32>>,
        bq_gain1: Arc<RwLock<f32>>,
        bq_gain2: Arc<RwLock<f32>>,
        bq_gain3: Arc<RwLock<f32>>,
        bq_gain4: Arc<RwLock<f32>>,
        bq_gain5: Arc<RwLock<f32>>
        ) -> Self{
        Self{
            gain: gain.clone(),
            bq_gain0: bq_gain0.clone(),
            bq_gain1: bq_gain1.clone(),
            bq_gain2: bq_gain2.clone(),
            bq_gain3: bq_gain3.clone(),
            bq_gain4: bq_gain4.clone(),
            bq_gain5: bq_gain5.clone(),
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
                    *self.bq_gain0.write().unwrap() = 0.8222;
                    *self.bq_gain1.write().unwrap() = 0.2113;
                    *self.bq_gain2.write().unwrap() = 0.0989;
                    *self.bq_gain3.write().unwrap() = 0.0507;
                }
                if ui.button("White").clicked(){
                    *self.bq_gain0.write().unwrap() = 1.0;
                    *self.bq_gain1.write().unwrap() = 0.75;
                    *self.bq_gain2.write().unwrap() = 0.75;
                    *self.bq_gain3.write().unwrap() = 1.0;
                }
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                ui.add(
                    Slider::new(&mut *self.gain.write().unwrap().ptr(), 0.0..=1.0)
                    .text("Gain")
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.1)
                    );
                ui.add(
                    Slider::new(&mut *self.bq_gain0.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
                ui.add(
                    Slider::new(&mut *self.bq_gain1.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
                ui.add(
                    Slider::new(&mut *self.bq_gain2.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
                ui.add(
                    Slider::new(&mut *self.bq_gain3.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
                ui.add(
                    Slider::new(&mut *self.bq_gain4.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
                ui.add(
                    Slider::new(&mut *self.bq_gain5.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
            });
        });
    }
}

