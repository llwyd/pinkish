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
    let mut p = Pink::new();

    let stream = device.build_output_stream(&config.into(),
    move |data: &mut [f32], _: &cpal::OutputCallbackInfo|
    {
        for sample in data.iter_mut()
        {
            *sample = p.update();
        }
    },
    move |err|
    {
    }, 
    None).unwrap();


    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(4000));
}
