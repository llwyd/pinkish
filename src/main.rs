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

mod noise;
mod voss;

use crate::noise::Noise;
use crate::voss::Pink;

fn main() {
    println!("Hello, world!");

    let host = cpal::default_host();

    let device = host.default_output_device().expect("Host Device error");
    
    let config = device.default_output_config().unwrap();
    println!("Default Config: {config:?}");

    assert!(config.sample_format() == cpal::SampleFormat::F32);
    assert!(config.sample_rate() == 44100);

    let num_channels = config.channels() as usize;
    println!("Channels: {}", num_channels);
    let mut p = Pink::new();
    let mut stereo = [Pink::new(),Pink::new()];
    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        for frame in data.chunks_mut(num_channels)
        {
            let next = p.update();
            for (idx,sample) in frame.iter_mut().enumerate()
            {
                *sample = stereo[idx].update();
            }
        }
    },
    move |err|
    {
    }, 
    None).unwrap();


    stream.play().unwrap();

    loop {}
}
