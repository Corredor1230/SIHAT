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
    overtone::process(unit, &sihat_settings.o_settings);
}