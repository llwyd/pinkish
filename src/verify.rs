use std::sync::{Arc, RwLock};
use crate::agc::AGC;
use crate::eq::Equaliser;
use crate::gain::Gain;
use crate::noise::Noise;
use crate::rms::RMS;

pub fn verify(
    mut stereo_noise:[Noise;2],
    mut eq: [Equaliser;2],
    _m_gain: Arc<RwLock<Gain>>,
    _rms: Arc<RwLock<[RMS;2]>>,
    _agc: Arc<RwLock<[AGC;2]>>,
    output_len:u16
    ) -> eframe::Result{

   
    let out_len = output_len as u32;
    for _i in 0..(48000 * out_len){
        let inp = stereo_noise[0].update();
        let out = eq[0].next(inp);

        println!("{:?}", out);
    }
    Result::Ok(())
}

pub fn verify_all(
    mut stereo_noise:[Noise;2],
    mut eq: [Equaliser;2],
    m_gain: Arc<RwLock<Gain>>,
    rms: Arc<RwLock<[RMS;2]>>,
    agc: Arc<RwLock<[AGC;2]>>,
    output_len:u16
    ) -> eframe::Result{

  
    let out_len = output_len as u32;
    let mut rms_out;
    for _i in 0..(48000 * out_len){
        let inp = stereo_noise[0].update();
        let fout = eq[0].next(inp);
        let gout = fout * agc.read().unwrap()[0].gain();
        rms_out = rms.write().unwrap()[0].next(gout);
        agc.write().unwrap()[0].update(rms_out);
        let out = gout * m_gain.read().unwrap().value();

        println!("{:?}", out);
    }
    Result::Ok(())
}

pub fn verify_impulse(
    mut _stereo_noise:[Noise;2],
    mut eq: [Equaliser;2],
    _m_gain: Arc<RwLock<Gain>>,
    _rms: Arc<RwLock<[RMS;2]>>,
    _agc: Arc<RwLock<[AGC;2]>>,
    output_len:u16
    ) -> eframe::Result{

   
    let out_len = output_len as u32;
    for i in 0..(48000 * out_len){
        let inp;
        if i == 0
        {
            inp = 1.0
        }
        else
        {
            inp = 0.0
        }
        let out = eq[0].next(inp);

        println!("{:?}", out);
    }
    Result::Ok(())
}
