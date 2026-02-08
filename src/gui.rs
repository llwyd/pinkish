use egui::{Align,Layout,Slider,SliderOrientation,special_emojis};
use std::sync::{Arc, RwLock};
use crate::gain::Gain;
use crate::rms::RMS;
use crate::agc::AGC;
use crate::audio_config::*;
use crate::audio_magic::*;

pub struct PinkishGUI {
    gain:Arc<RwLock<Gain>>,
    eq_gain:Arc<RwLock<Vec<f32>>>,
    pink_gain: [f32;6],
    channels: Arc<RwLock<AudioChannels>>,
    _rms: Arc<RwLock<[RMS;2]>>,
    agc: Arc<RwLock<[AGC;2]>>,
    fs: f32,
}

impl PinkishGUI{
    pub fn new(_cc: &eframe::CreationContext<'_>,
        gain: Arc<RwLock<Gain>>,
        eq_gain:Arc<RwLock<Vec<f32>>>,
        pink_gain: [f32;6],
        channels: Arc<RwLock<AudioChannels>>,
        rms: Arc<RwLock<[RMS;2]>>,
        agc: Arc<RwLock<[AGC;2]>>,
        fs: f32,
        ) -> Self{
        Self{
            gain: gain.clone(),
            eq_gain: eq_gain.clone(),
            pink_gain,
            channels,
            _rms: rms.clone(),
            agc: agc.clone(),
            fs,
        }
    }
}

impl eframe::App for PinkishGUI{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::TopBottomPanel::top("").show(ctx, |ui|{
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                ui.label("Noise generator with 6 band graphic EQ");
            });
        });
        egui::TopBottomPanel::bottom("info").show(ctx, |ui|{
            ui.with_layout(Layout::with_main_align(Layout::left_to_right(Align::BOTTOM),Align::Center), |ui|
            {
                ui.hyperlink_to(format!("GitHub {}",special_emojis::GITHUB), "https://github.com/llwyd/pinkish");
                ui.hyperlink_to(format!("llwyd.io"), "https://llwyd.io");
                ui.label(format!("{} Hz", self.fs));
                ui.label(format!("v{}",env!("CARGO_PKG_VERSION")));
            });
        
        });
        egui::CentralPanel::default().show(ctx, |ui|
        {
            ui.horizontal( |ui|
            {
                ui.add_space(30.0);
                ui.style_mut().spacing.slider_width = 120.0;
                ui.style_mut().spacing.slider_rail_height = 10.0;
                ui.spacing_mut().item_spacing.x = 25.0;
                ui.add(
                    Slider::new(&mut self.eq_gain.write().unwrap()[0], GAIN_SLIDER_MIN..=GAIN_SLIDER_MAX)
                    .orientation(SliderOrientation::Vertical)
                    .step_by(GAIN_SLIDER_INC)
                    .logarithmic(true)
                    .show_value(false)
                    .text("Low")
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
                    .text("High")
                    );
                });
            ui.horizontal( |ui|
            {
                ui.add_space(50.0);
                ui.style_mut().spacing.slider_width = 120.0;
                ui.style_mut().spacing.slider_rail_height = 10.0;
                ui.spacing_mut().item_spacing.x = 15.0;
                if ui.button("🔈").clicked(){
                    self.gain.write().unwrap().decrement();
                }
                ui.add(
                    Slider::new(&mut *self.gain.write().unwrap().ptr(), 0.0..=VOLUME_SLIDER_MAX)
                    .orientation(SliderOrientation::Horizontal)
                    .step_by(0.01)
                    .show_value(false)
                    );
                if ui.button("🔊").clicked(){
                    self.gain.write().unwrap().increment();
                }
            });
                    ui.separator();
                ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
                {
                    ui.radio_value(&mut *self.channels.write().unwrap(), AudioChannels::Mono, "Mono");
                    ui.radio_value(&mut *self.channels.write().unwrap(), AudioChannels::Stereo, "Stereo");
                    ui.label("Presets:");
                    if ui.button("Pink").clicked(){
                        self.eq_gain.write().unwrap()[0] = self.pink_gain[0];
                        self.eq_gain.write().unwrap()[1] = self.pink_gain[1];
                        self.eq_gain.write().unwrap()[2] = self.pink_gain[2];
                        self.eq_gain.write().unwrap()[3] = self.pink_gain[3];
                        self.eq_gain.write().unwrap()[4] = self.pink_gain[4];
                        self.eq_gain.write().unwrap()[5] = self.pink_gain[5];
                    }
                    if ui.button("White").clicked(){

                        self.agc.write().unwrap()[0].reset();
                        self.agc.write().unwrap()[1].reset();
                        let white_gain = DEFAULT_CROSSOVER_GAIN;
                        self.eq_gain.write().unwrap()[0] = white_gain;
                        self.eq_gain.write().unwrap()[1] = white_gain;
                        self.eq_gain.write().unwrap()[2] = white_gain;
                        self.eq_gain.write().unwrap()[3] = white_gain;
                        self.eq_gain.write().unwrap()[4] = white_gain;
                        self.eq_gain.write().unwrap()[5] = white_gain;
                    }
                });
                ui.allocate_space(ui.available_size());
        });
    }
}

pub struct PinkishWeb
{
    fs: f32
}

impl PinkishWeb
{
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self
    {
        Self
        {
            fs: 48000.0,
        }
    }
}

impl eframe::App for PinkishWeb{
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame){
        egui::CentralPanel::default().show(ctx, |ui|{
            ui.with_layout(Layout::left_to_right(Align::TOP), |ui|
            {
                ui.label("Noise generator with 6 band graphic EQ");
            });
        });

    }
}
