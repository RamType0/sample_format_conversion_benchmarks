#![feature(test)]
extern crate test;
mod samples_format_mine;
use samples_format_mine::Sample as MySample;
use cpal::Sample;

#[cfg(test)]
mod tests{
    use std::f32::consts::PI;

    use super::*;
    use test::Bencher;

    fn sine_wave_f32()-> Vec<f32>{
        let length = 512;
        let mut vec = vec![Default::default();length];
        for i in 0..length {
            vec[i] = (i as f32 / length as f32 * PI * 2.0).sin();
        }
        vec
    }

    fn sine_wave_i16() -> Vec<i16>{
        sine_wave_f32().iter().map(|v| v.to_i16()).collect::<Vec<i16>>()
    }

    #[bench]
    fn bench_current_f32_to_i16(b: &mut Bencher){
        let input = sine_wave_f32();
        let mut output = vec![Default::default();input.len()];
        b.iter(move || {
            for (i,x) in input.iter().enumerate() {
                output[i] = x.to_i16();
            }
        });
    }
    #[bench]
    fn bench_my_f32_to_i16(b: &mut Bencher){
        let input = sine_wave_f32();
        let mut output = vec![Default::default();input.len()];
        b.iter(move || {
            for (i,x) in input.iter().enumerate() {
                output[i] = x.to_i16_mine();
            }
        });
    }
    #[bench]
    fn bench_current_f32_to_u16(b: &mut Bencher){
        let input = sine_wave_f32();
        let mut output = vec![Default::default();input.len()];
        b.iter(move || {
            for (i,x) in input.iter().enumerate() {
                output[i] = x.to_u16();
            }
        });
    }
    #[bench]
    fn bench_my_f32_to_u16(b: &mut Bencher){
        let input = sine_wave_f32();
        let mut output = vec![Default::default();input.len()];
        b.iter(move || {
            for (i,x) in input.iter().enumerate() {
                output[i] = x.to_u16_mine();
            }
        });
    }
    #[bench]
    fn bench_current_i16_to_f32(b: &mut Bencher){
        let input = sine_wave_i16();
        let mut output = vec![Default::default();input.len()];
        b.iter(move || {
            for (i,x) in input.iter().enumerate() {
                output[i] = x.to_f32();
            }
        });
    }
    #[bench]
    fn bench_my_i16_to_f32(b: &mut Bencher){
        let input = sine_wave_i16();
        let mut output = vec![Default::default();input.len()];
        b.iter(move || {
            for (i,x) in input.iter().enumerate() {
                output[i] = x.to_f32_mine();
            }
        });
    }
}