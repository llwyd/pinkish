use cpal::traits::{
    DeviceTrait,
    HostTrait,
    StreamTrait,
};
use cpal::
{
    Sample,
    SizedSample,
};

use eframe::egui;
use egui::{Key,ScrollArea};
mod noise;
mod voss;
mod resonator;
mod gain;

use crate::gain::Gain;
use crate::noise::Noise;
use crate::voss::Pink;
use crate::resonator::Resonator;

fn main() -> eframe::Result{

    let options = eframe::NativeOptions
    {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let host = cpal::default_host();

    let device = host.default_output_device().expect("Host Device error");
    
    let config = device.default_output_config().unwrap();
    println!("Default Config: {config:?}");

    assert!(config.sample_format() == cpal::SampleFormat::F32);
    //assert!(config.sample_rate() == 44100);

    let num_channels = config.channels() as usize;
    println!("Channels: {}", num_channels);
    let mut stereo = [Pink::new(),Pink::new()];
    let mut gain = 1.0;
    let mut g = Gain::new();

    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        //println!("{gain}");
        for frame in data.chunks_mut(num_channels)
        {
            for (idx,sample) in frame.iter_mut().enumerate()
            {
                *sample = stereo[idx].update() * g.value();
            }
        }
    },
    move |err|
    {
    }, 
    None).unwrap();


    stream.play().unwrap();

    eframe::run_native(
        "Pinkish",
        options,
        Box::new(
            |cc|{
                Ok(Box::new(PinkishApp::new(cc, &mut g)))
            }))
}

struct PinkishApp<'a> {
    gain: &'a mut Gain,
}

impl <'a>PinkishApp<'a>{
    fn new(_cc: &eframe::CreationContext<'_>, gain: &'a mut Gain) -> Self{
        Self{
            gain: gain,
        }
    }
}

impl eframe::App for PinkishApp<'_>{
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.heading("Hello Pinkish");
            if ui.button("Stop").clicked(){
                self.gain.silence();
                println!("button pressed {}", self.gain.value());
            }       
        });
    }
}


