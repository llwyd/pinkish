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

fn write_samples(data: &mut [f32], _:&cpal::OutputCallbackInfo)
{
    for sample in data.iter_mut()
    {
        *sample = 0.0;
    }
}

fn audio_loop(device: &cpal::Device, config: &cpal::StreamConfig)
{
}

fn main() {
    println!("Hello, world!");

    let host = cpal::default_host();

    let device = host.default_output_device().expect("Host Device error");
    
    let config = device.default_output_config().unwrap();
    println!("Default Config: {config:?}");

    assert!(config.sample_format() == cpal::SampleFormat::F32);
    assert!(config.sample_rate() == 44100);

    //audio_loop(&device, &config.into());

    let err_fn = |err| eprintln!("Error {}", err);

    let stream = device.build_output_stream(&config.into(),write_samples,err_fn, None).unwrap();


    stream.play().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(1000));
}
