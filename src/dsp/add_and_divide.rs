use std::{collections::HashSet, sync::Arc};

use crate::system::parameter::Parameter;
use super::highpass::HighPassFilter;

pub struct AddAndDivide {
    pub hpf: HighPassFilter,
    pub volumes: HashSet<String, Arc<Parameter>>
}

impl AddAndDivide {

}