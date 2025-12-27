use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};
/*
use cpal::
{
    Sample,
    SizedSample,
};
*/
use std::sync::{Arc, RwLock};
use eframe::egui;
use egui::{Align,Layout,Slider,SliderOrientation};
mod noise;
mod voss;
mod gain;
mod biquad;
mod filterbank;

use crate::gain::Gain;
use crate::noise::Noise;
use crate::biquad::Biquad;
use crate::filterbank::FilterBank;

fn main() -> eframe::Result{

    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    let g = Arc::new(RwLock::new(Gain::new()));
    
    let host = cpal::default_host();

    let device = host.default_output_device().expect("Host Device error");
    
    let config = device.default_output_config().unwrap();
    println!("Default Config: {config:?}");

    assert!(config.sample_format() == cpal::SampleFormat::F32);
    //assert!(config.sample_rate() == 44100);

    let num_channels = config.channels() as usize;
    println!("Channels: {}", num_channels);
    let mut stereo = [Noise::new(),Noise::new()];
    let biquad_gain = Arc::new(RwLock::new(1.0));
    let mut filter = [
        Biquad::new([0.00764556, 0.00764556, 0.0],[-0.98470888,0.0]),
        Biquad::new([0.00764556, 0.00764556, 0.0],[-0.98470888,0.0])
    ];

    let noise = Noise::new();
    let value = g.clone();
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        let master_gain = value.write().unwrap().value();
        
        for frame in data.chunks_mut(num_channels)
        {
            for (idx,sample) in frame.iter_mut().enumerate()
            {
                let noise = stereo[idx].update();
                let next = filter[idx].next(noise);
                
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
                            biquad_gain.clone())))
            }))
}

struct PinkishApp {
    gain:Arc<RwLock<Gain>>,
    biquad_gain:Arc<RwLock<f32>>,
    log_gain:f32,
    slider_gain: f32,
}

impl PinkishApp{
    fn new(_cc: &eframe::CreationContext<'_>,
        gain: Arc<RwLock<Gain>>,
        biquad_gain: Arc<RwLock<f32>>) -> Self{
        Self{
            gain: gain.clone(),
            biquad_gain: biquad_gain.clone(),
            log_gain: 1.0,
            slider_gain: 0.0,
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
                    self.gain.write().unwrap().silence();
                }
                if ui.button("White").clicked(){
                    self.gain.write().unwrap().silence();
                }
            });
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                ui.add(
                    Slider::new(&mut *self.gain.write().unwrap().ptr(), 0.0..=1.0)
                    .text("Master Gain")
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.1)
                    );
                ui.add(
                    Slider::new(&mut *self.biquad_gain.write().unwrap(), 0.00001..=1.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.001)
                    .logarithmic(true)
                    );
                ui.add(
                    Slider::new(&mut self.slider_gain, -100.0..=0.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.1)
                    );
                ui.add(
                    Slider::new(&mut self.slider_gain, -100.0..=0.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.1)
                    );
                ui.add(
                    Slider::new(&mut self.slider_gain, -100.0..=0.0)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(0.1)
                    );
            });
        });
    }
}

