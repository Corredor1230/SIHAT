use crate::dsp::stft::FftContext;
use crate::models::AudioInfo;
use crate::results::SihatResults;
use crate::models::SihatSettings;

use super::harmonic;
use super::overtone;
use super::transient;

pub fn analyze(unit: &AudioInfo, sihat_settings: &SihatSettings, context: &mut FftContext)
{
    
    let mut sihat_results: SihatResults = Default::default();
    sihat_results.t_results = transient::process(unit, &sihat_settings.t_settings, unit.meta_pitch, context);
    sihat_results.o_results = overtone::process(unit, &sihat_settings.o_settings, context);
    sihat_results.h_results = harmonic::process(unit, &sihat_settings.h_settings, context, &sihat_results.o_results, sihat_results.t_results.range.end);
    
}