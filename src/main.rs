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
use egui::{Slider,SliderOrientation};
mod noise;
mod voss;
mod gain;

use crate::gain::Gain;
use crate::voss::Pink;

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
    let mut stereo = [Pink::new(),Pink::new()];

    let value = g.clone();
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        let gain = value.write().unwrap().value();
        for frame in data.chunks_mut(num_channels)
        {
            for (idx,sample) in frame.iter_mut().enumerate()
            {
                *sample = stereo[idx].update() * gain;
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
                Ok(Box::new(PinkishApp::new(cc, g.clone())))
            }))
}

struct PinkishApp {
    gain:Arc<RwLock<Gain>>,
    slider_gain:f32,
}

impl PinkishApp{
    fn new(_cc: &eframe::CreationContext<'_>, gain: Arc<RwLock<Gain>>) -> Self{
        Self{
            gain: gain.clone(),
            slider_gain: 0.0,
        }
    }
}

impl eframe::App for PinkishApp{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Hello Pinkish");
            if ui.button("Stop").clicked(){
                self.gain.write().unwrap().silence();
            }
            ui.add(
                Slider::new(&mut *self.gain.write().unwrap().ptr(), 0.0..=1.0)
                .text("Master Gain")
                .orientation(SliderOrientation::Vertical)
                .step_by(0.1)
                );
            /*
            ui.add(
                Slider::new(&mut self.slider_gain, -100.0..=0.0)
                .orientation(SliderOrientation::Vertical)
                .step_by(0.1)
                );
            */
        });
    }
}

