use crate::models::AudioInfo;
use crate::results::SihatResults;
use crate::models::SihatSettings;

use super::harmonic;
use super::overtone;
use super::period;
use super::transient;

pub fn analyze(unit: &AudioInfo, sihat_settings: &SihatSettings)
{
    let mut sihat_results: SihatResults = Default::default();
    sihat_results.o_results = overtone::process(unit, &sihat_settings.o_settings);
    sihat_results.t_results = transient::process(unit, &sihat_settings.t_settings);
}