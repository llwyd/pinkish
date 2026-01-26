#[derive(Clone,Debug,PartialEq)]
pub enum AudioChannels
{
    Mono,
    Stereo,
}

#[derive(Clone,Debug,PartialEq)]
pub enum OpMode
{
    Playback,
    Verify,
    Filter,
}

