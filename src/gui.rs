use egui::{Align,Layout,Slider,SliderOrientation};
use std::sync::{Arc, RwLock};
use crate::gain::Gain;
use crate::rms::RMS;
use crate::audio_config::*;
use crate::audio_magic::*;

pub struct PinkishGUI {
    gain:Arc<RwLock<Gain>>,
    eq_gain:Arc<RwLock<Vec<f32>>>,
    pink_gain: [f32;6],
    channels: Arc<RwLock<AudioChannels>>,
    rms: Arc<RwLock<[RMS;2]>>,
}

impl PinkishGUI{
    pub fn new(_cc: &eframe::CreationContext<'_>,
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

impl eframe::App for PinkishGUI{
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
                if ui.button("White").clicked(){

                    let white_gain = DEFAULT_CROSSOVER_GAIN;
                    self.eq_gain.write().unwrap()[0] = white_gain;
                    self.eq_gain.write().unwrap()[1] = white_gain;
                    self.eq_gain.write().unwrap()[2] = white_gain;
                    self.eq_gain.write().unwrap()[3] = white_gain;
                    self.eq_gain.write().unwrap()[4] = white_gain;
                    self.eq_gain.write().unwrap()[5] = white_gain;
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
                    .step_by(0.01)
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
