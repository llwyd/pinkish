use crate::crossover::*;
use crate::biquad::Biquad;
use crate::filter_coeffs_8000;
use crate::filter_coeffs_16000;
use crate::filter_coeffs_32000;
use crate::filter_coeffs_44100;
use crate::filter_coeffs_48000;

pub fn load_filters_48000() -> (CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,[f32;6])
{
    let mut filter_0_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_1_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_2_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_3_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_4_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    for i in 0..filter_coeffs_48000::FILTER_0_LPF.len()
    {
        filter_0_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_0_LPF[i][0],
                    filter_coeffs_48000::FILTER_0_LPF[i][1],
                    filter_coeffs_48000::FILTER_0_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_0_LPF[i][4],
                    filter_coeffs_48000::FILTER_0_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_0_HPF.len()
    {
        filter_0_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_0_HPF[i][0],
                    filter_coeffs_48000::FILTER_0_HPF[i][1],
                    filter_coeffs_48000::FILTER_0_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_0_HPF[i][4],
                    filter_coeffs_48000::FILTER_0_HPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_1_LPF.len()
    {
        filter_1_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_1_LPF[i][0],
                    filter_coeffs_48000::FILTER_1_LPF[i][1],
                    filter_coeffs_48000::FILTER_1_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_1_LPF[i][4],
                    filter_coeffs_48000::FILTER_1_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_1_HPF.len()
    {
        filter_1_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_1_HPF[i][0],
                    filter_coeffs_48000::FILTER_1_HPF[i][1],
                    filter_coeffs_48000::FILTER_1_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_1_HPF[i][4],
                    filter_coeffs_48000::FILTER_1_HPF[i][5]
                ])
            );
    }
    
    /* Filter 2 */
    for i in 0..filter_coeffs_48000::FILTER_2_LPF.len()
    {
        filter_2_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_2_LPF[i][0],
                    filter_coeffs_48000::FILTER_2_LPF[i][1],
                    filter_coeffs_48000::FILTER_2_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_2_LPF[i][4],
                    filter_coeffs_48000::FILTER_2_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_2_HPF.len()
    {
        filter_2_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_2_HPF[i][0],
                    filter_coeffs_48000::FILTER_2_HPF[i][1],
                    filter_coeffs_48000::FILTER_2_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_2_HPF[i][4],
                    filter_coeffs_48000::FILTER_2_HPF[i][5]
                ])
            );
    }
    
    /* Filter 3 */
    for i in 0..filter_coeffs_48000::FILTER_3_LPF.len()
    {
        filter_3_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_3_LPF[i][0],
                    filter_coeffs_48000::FILTER_3_LPF[i][1],
                    filter_coeffs_48000::FILTER_3_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_3_LPF[i][4],
                    filter_coeffs_48000::FILTER_3_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_3_HPF.len()
    {
        filter_3_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_3_HPF[i][0],
                    filter_coeffs_48000::FILTER_3_HPF[i][1],
                    filter_coeffs_48000::FILTER_3_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_3_HPF[i][4],
                    filter_coeffs_48000::FILTER_3_HPF[i][5]
                ])
            );
    }

    /* Filter 4 */
    for i in 0..filter_coeffs_48000::FILTER_4_LPF.len()
    {
        filter_4_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_4_LPF[i][0],
                    filter_coeffs_48000::FILTER_4_LPF[i][1],
                    filter_coeffs_48000::FILTER_4_LPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_4_LPF[i][4],
                    filter_coeffs_48000::FILTER_4_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_48000::FILTER_4_HPF.len()
    {
        filter_4_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_48000::FILTER_4_HPF[i][0],
                    filter_coeffs_48000::FILTER_4_HPF[i][1],
                    filter_coeffs_48000::FILTER_4_HPF[i][2]
                ],
                [
                    filter_coeffs_48000::FILTER_4_HPF[i][4],
                    filter_coeffs_48000::FILTER_4_HPF[i][5]
                ])
            );
    }
    

    (
    filter_0_bq.clone(),
    filter_1_bq.clone(),
    filter_2_bq.clone(),
    filter_3_bq.clone(),
    filter_4_bq.clone(),
    filter_coeffs_48000::PINK_GAIN,
    )
}

pub fn load_filters_44100() -> (CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,[f32;6])
{
    let mut filter_0_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_1_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_2_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_3_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_4_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    for i in 0..filter_coeffs_44100::FILTER_0_LPF.len()
    {
        filter_0_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_0_LPF[i][0],
                    filter_coeffs_44100::FILTER_0_LPF[i][1],
                    filter_coeffs_44100::FILTER_0_LPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_0_LPF[i][4],
                    filter_coeffs_44100::FILTER_0_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_44100::FILTER_0_HPF.len()
    {
        filter_0_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_0_HPF[i][0],
                    filter_coeffs_44100::FILTER_0_HPF[i][1],
                    filter_coeffs_44100::FILTER_0_HPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_0_HPF[i][4],
                    filter_coeffs_44100::FILTER_0_HPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_44100::FILTER_1_LPF.len()
    {
        filter_1_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_1_LPF[i][0],
                    filter_coeffs_44100::FILTER_1_LPF[i][1],
                    filter_coeffs_44100::FILTER_1_LPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_1_LPF[i][4],
                    filter_coeffs_44100::FILTER_1_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_44100::FILTER_1_HPF.len()
    {
        filter_1_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_1_HPF[i][0],
                    filter_coeffs_44100::FILTER_1_HPF[i][1],
                    filter_coeffs_44100::FILTER_1_HPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_1_HPF[i][4],
                    filter_coeffs_44100::FILTER_1_HPF[i][5]
                ])
            );
    }
    
    /* Filter 2 */
    for i in 0..filter_coeffs_44100::FILTER_2_LPF.len()
    {
        filter_2_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_2_LPF[i][0],
                    filter_coeffs_44100::FILTER_2_LPF[i][1],
                    filter_coeffs_44100::FILTER_2_LPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_2_LPF[i][4],
                    filter_coeffs_44100::FILTER_2_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_44100::FILTER_2_HPF.len()
    {
        filter_2_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_2_HPF[i][0],
                    filter_coeffs_44100::FILTER_2_HPF[i][1],
                    filter_coeffs_44100::FILTER_2_HPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_2_HPF[i][4],
                    filter_coeffs_44100::FILTER_2_HPF[i][5]
                ])
            );
    }
    
    /* Filter 3 */
    for i in 0..filter_coeffs_44100::FILTER_3_LPF.len()
    {
        filter_3_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_3_LPF[i][0],
                    filter_coeffs_44100::FILTER_3_LPF[i][1],
                    filter_coeffs_44100::FILTER_3_LPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_3_LPF[i][4],
                    filter_coeffs_44100::FILTER_3_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_44100::FILTER_3_HPF.len()
    {
        filter_3_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_3_HPF[i][0],
                    filter_coeffs_44100::FILTER_3_HPF[i][1],
                    filter_coeffs_44100::FILTER_3_HPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_3_HPF[i][4],
                    filter_coeffs_44100::FILTER_3_HPF[i][5]
                ])
            );
    }

    /* Filter 4 */
    for i in 0..filter_coeffs_44100::FILTER_4_LPF.len()
    {
        filter_4_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_4_LPF[i][0],
                    filter_coeffs_44100::FILTER_4_LPF[i][1],
                    filter_coeffs_44100::FILTER_4_LPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_4_LPF[i][4],
                    filter_coeffs_44100::FILTER_4_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_44100::FILTER_4_HPF.len()
    {
        filter_4_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_44100::FILTER_4_HPF[i][0],
                    filter_coeffs_44100::FILTER_4_HPF[i][1],
                    filter_coeffs_44100::FILTER_4_HPF[i][2]
                ],
                [
                    filter_coeffs_44100::FILTER_4_HPF[i][4],
                    filter_coeffs_44100::FILTER_4_HPF[i][5]
                ])
            );
    }
    

    (
    filter_0_bq.clone(),
    filter_1_bq.clone(),
    filter_2_bq.clone(),
    filter_3_bq.clone(),
    filter_4_bq.clone(),
    filter_coeffs_44100::PINK_GAIN,
    )
}

pub fn load_filters_32000() -> (CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,[f32;6])
{
    let mut filter_0_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_1_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_2_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_3_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_4_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    for i in 0..filter_coeffs_32000::FILTER_0_LPF.len()
    {
        filter_0_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_0_LPF[i][0],
                    filter_coeffs_32000::FILTER_0_LPF[i][1],
                    filter_coeffs_32000::FILTER_0_LPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_0_LPF[i][4],
                    filter_coeffs_32000::FILTER_0_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_32000::FILTER_0_HPF.len()
    {
        filter_0_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_0_HPF[i][0],
                    filter_coeffs_32000::FILTER_0_HPF[i][1],
                    filter_coeffs_32000::FILTER_0_HPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_0_HPF[i][4],
                    filter_coeffs_32000::FILTER_0_HPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_32000::FILTER_1_LPF.len()
    {
        filter_1_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_1_LPF[i][0],
                    filter_coeffs_32000::FILTER_1_LPF[i][1],
                    filter_coeffs_32000::FILTER_1_LPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_1_LPF[i][4],
                    filter_coeffs_32000::FILTER_1_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_32000::FILTER_1_HPF.len()
    {
        filter_1_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_1_HPF[i][0],
                    filter_coeffs_32000::FILTER_1_HPF[i][1],
                    filter_coeffs_32000::FILTER_1_HPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_1_HPF[i][4],
                    filter_coeffs_32000::FILTER_1_HPF[i][5]
                ])
            );
    }
    
    /* Filter 2 */
    for i in 0..filter_coeffs_32000::FILTER_2_LPF.len()
    {
        filter_2_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_2_LPF[i][0],
                    filter_coeffs_32000::FILTER_2_LPF[i][1],
                    filter_coeffs_32000::FILTER_2_LPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_2_LPF[i][4],
                    filter_coeffs_32000::FILTER_2_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_32000::FILTER_2_HPF.len()
    {
        filter_2_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_2_HPF[i][0],
                    filter_coeffs_32000::FILTER_2_HPF[i][1],
                    filter_coeffs_32000::FILTER_2_HPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_2_HPF[i][4],
                    filter_coeffs_32000::FILTER_2_HPF[i][5]
                ])
            );
    }
    
    /* Filter 3 */
    for i in 0..filter_coeffs_32000::FILTER_3_LPF.len()
    {
        filter_3_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_3_LPF[i][0],
                    filter_coeffs_32000::FILTER_3_LPF[i][1],
                    filter_coeffs_32000::FILTER_3_LPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_3_LPF[i][4],
                    filter_coeffs_32000::FILTER_3_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_32000::FILTER_3_HPF.len()
    {
        filter_3_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_3_HPF[i][0],
                    filter_coeffs_32000::FILTER_3_HPF[i][1],
                    filter_coeffs_32000::FILTER_3_HPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_3_HPF[i][4],
                    filter_coeffs_32000::FILTER_3_HPF[i][5]
                ])
            );
    }

    /* Filter 4 */
    for i in 0..filter_coeffs_32000::FILTER_4_LPF.len()
    {
        filter_4_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_4_LPF[i][0],
                    filter_coeffs_32000::FILTER_4_LPF[i][1],
                    filter_coeffs_32000::FILTER_4_LPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_4_LPF[i][4],
                    filter_coeffs_32000::FILTER_4_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_32000::FILTER_4_HPF.len()
    {
        filter_4_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_32000::FILTER_4_HPF[i][0],
                    filter_coeffs_32000::FILTER_4_HPF[i][1],
                    filter_coeffs_32000::FILTER_4_HPF[i][2]
                ],
                [
                    filter_coeffs_32000::FILTER_4_HPF[i][4],
                    filter_coeffs_32000::FILTER_4_HPF[i][5]
                ])
            );
    }
    

    (
    filter_0_bq.clone(),
    filter_1_bq.clone(),
    filter_2_bq.clone(),
    filter_3_bq.clone(),
    filter_4_bq.clone(),
    filter_coeffs_32000::PINK_GAIN,
    )
}

pub fn load_filters_16000() -> (CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,[f32;6])
{
    let mut filter_0_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_1_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_2_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_3_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_4_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    for i in 0..filter_coeffs_16000::FILTER_0_LPF.len()
    {
        filter_0_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_0_LPF[i][0],
                    filter_coeffs_16000::FILTER_0_LPF[i][1],
                    filter_coeffs_16000::FILTER_0_LPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_0_LPF[i][4],
                    filter_coeffs_16000::FILTER_0_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_16000::FILTER_0_HPF.len()
    {
        filter_0_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_0_HPF[i][0],
                    filter_coeffs_16000::FILTER_0_HPF[i][1],
                    filter_coeffs_16000::FILTER_0_HPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_0_HPF[i][4],
                    filter_coeffs_16000::FILTER_0_HPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_16000::FILTER_1_LPF.len()
    {
        filter_1_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_1_LPF[i][0],
                    filter_coeffs_16000::FILTER_1_LPF[i][1],
                    filter_coeffs_16000::FILTER_1_LPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_1_LPF[i][4],
                    filter_coeffs_16000::FILTER_1_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_16000::FILTER_1_HPF.len()
    {
        filter_1_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_1_HPF[i][0],
                    filter_coeffs_16000::FILTER_1_HPF[i][1],
                    filter_coeffs_16000::FILTER_1_HPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_1_HPF[i][4],
                    filter_coeffs_16000::FILTER_1_HPF[i][5]
                ])
            );
    }
    
    /* Filter 2 */
    for i in 0..filter_coeffs_16000::FILTER_2_LPF.len()
    {
        filter_2_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_2_LPF[i][0],
                    filter_coeffs_16000::FILTER_2_LPF[i][1],
                    filter_coeffs_16000::FILTER_2_LPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_2_LPF[i][4],
                    filter_coeffs_16000::FILTER_2_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_16000::FILTER_2_HPF.len()
    {
        filter_2_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_2_HPF[i][0],
                    filter_coeffs_16000::FILTER_2_HPF[i][1],
                    filter_coeffs_16000::FILTER_2_HPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_2_HPF[i][4],
                    filter_coeffs_16000::FILTER_2_HPF[i][5]
                ])
            );
    }
    
    /* Filter 3 */
    for i in 0..filter_coeffs_16000::FILTER_3_LPF.len()
    {
        filter_3_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_3_LPF[i][0],
                    filter_coeffs_16000::FILTER_3_LPF[i][1],
                    filter_coeffs_16000::FILTER_3_LPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_3_LPF[i][4],
                    filter_coeffs_16000::FILTER_3_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_16000::FILTER_3_HPF.len()
    {
        filter_3_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_3_HPF[i][0],
                    filter_coeffs_16000::FILTER_3_HPF[i][1],
                    filter_coeffs_16000::FILTER_3_HPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_3_HPF[i][4],
                    filter_coeffs_16000::FILTER_3_HPF[i][5]
                ])
            );
    }

    /* Filter 4 */
    for i in 0..filter_coeffs_16000::FILTER_4_LPF.len()
    {
        filter_4_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_4_LPF[i][0],
                    filter_coeffs_16000::FILTER_4_LPF[i][1],
                    filter_coeffs_16000::FILTER_4_LPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_4_LPF[i][4],
                    filter_coeffs_16000::FILTER_4_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_16000::FILTER_4_HPF.len()
    {
        filter_4_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_16000::FILTER_4_HPF[i][0],
                    filter_coeffs_16000::FILTER_4_HPF[i][1],
                    filter_coeffs_16000::FILTER_4_HPF[i][2]
                ],
                [
                    filter_coeffs_16000::FILTER_4_HPF[i][4],
                    filter_coeffs_16000::FILTER_4_HPF[i][5]
                ])
            );
    }
    

    (
    filter_0_bq.clone(),
    filter_1_bq.clone(),
    filter_2_bq.clone(),
    filter_3_bq.clone(),
    filter_4_bq.clone(),
    filter_coeffs_16000::PINK_GAIN,
    )
}

pub fn load_filters_8000() -> (CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,CrossoverBiquads,[f32;6])
{
    let mut filter_0_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_1_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_2_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_3_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    let mut filter_4_bq = CrossoverBiquads{
        lpf: Vec::new(),
        hpf: Vec::new()
    };
    for i in 0..filter_coeffs_8000::FILTER_0_LPF.len()
    {
        filter_0_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_0_LPF[i][0],
                    filter_coeffs_8000::FILTER_0_LPF[i][1],
                    filter_coeffs_8000::FILTER_0_LPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_0_LPF[i][4],
                    filter_coeffs_8000::FILTER_0_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_8000::FILTER_0_HPF.len()
    {
        filter_0_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_0_HPF[i][0],
                    filter_coeffs_8000::FILTER_0_HPF[i][1],
                    filter_coeffs_8000::FILTER_0_HPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_0_HPF[i][4],
                    filter_coeffs_8000::FILTER_0_HPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_8000::FILTER_1_LPF.len()
    {
        filter_1_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_1_LPF[i][0],
                    filter_coeffs_8000::FILTER_1_LPF[i][1],
                    filter_coeffs_8000::FILTER_1_LPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_1_LPF[i][4],
                    filter_coeffs_8000::FILTER_1_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_8000::FILTER_1_HPF.len()
    {
        filter_1_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_1_HPF[i][0],
                    filter_coeffs_8000::FILTER_1_HPF[i][1],
                    filter_coeffs_8000::FILTER_1_HPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_1_HPF[i][4],
                    filter_coeffs_8000::FILTER_1_HPF[i][5]
                ])
            );
    }
    
    /* Filter 2 */
    for i in 0..filter_coeffs_8000::FILTER_2_LPF.len()
    {
        filter_2_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_2_LPF[i][0],
                    filter_coeffs_8000::FILTER_2_LPF[i][1],
                    filter_coeffs_8000::FILTER_2_LPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_2_LPF[i][4],
                    filter_coeffs_8000::FILTER_2_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_8000::FILTER_2_HPF.len()
    {
        filter_2_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_2_HPF[i][0],
                    filter_coeffs_8000::FILTER_2_HPF[i][1],
                    filter_coeffs_8000::FILTER_2_HPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_2_HPF[i][4],
                    filter_coeffs_8000::FILTER_2_HPF[i][5]
                ])
            );
    }
    
    /* Filter 3 */
    for i in 0..filter_coeffs_8000::FILTER_3_LPF.len()
    {
        filter_3_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_3_LPF[i][0],
                    filter_coeffs_8000::FILTER_3_LPF[i][1],
                    filter_coeffs_8000::FILTER_3_LPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_3_LPF[i][4],
                    filter_coeffs_8000::FILTER_3_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_8000::FILTER_3_HPF.len()
    {
        filter_3_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_3_HPF[i][0],
                    filter_coeffs_8000::FILTER_3_HPF[i][1],
                    filter_coeffs_8000::FILTER_3_HPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_3_HPF[i][4],
                    filter_coeffs_8000::FILTER_3_HPF[i][5]
                ])
            );
    }

    /* Filter 4 */
    for i in 0..filter_coeffs_8000::FILTER_4_LPF.len()
    {
        filter_4_bq.lpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_4_LPF[i][0],
                    filter_coeffs_8000::FILTER_4_LPF[i][1],
                    filter_coeffs_8000::FILTER_4_LPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_4_LPF[i][4],
                    filter_coeffs_8000::FILTER_4_LPF[i][5]
                ])
            );
    }
    for i in 0..filter_coeffs_8000::FILTER_4_HPF.len()
    {
        filter_4_bq.hpf.push(
            Biquad::new(
                [
                    filter_coeffs_8000::FILTER_4_HPF[i][0],
                    filter_coeffs_8000::FILTER_4_HPF[i][1],
                    filter_coeffs_8000::FILTER_4_HPF[i][2]
                ],
                [
                    filter_coeffs_8000::FILTER_4_HPF[i][4],
                    filter_coeffs_8000::FILTER_4_HPF[i][5]
                ])
            );
    }
    

    (
    filter_0_bq.clone(),
    filter_1_bq.clone(),
    filter_2_bq.clone(),
    filter_3_bq.clone(),
    filter_4_bq.clone(),
    filter_coeffs_8000::PINK_GAIN,
    )
}
