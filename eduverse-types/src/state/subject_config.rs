#[cfg(feature = "anchor_contract")]
use anchor_lang::prelude::*;

#[cfg_attr(feature = "anchor_contract", account)]
#[derive(Default)]
pub struct SubjectConfig {
    /// The number of teachers teaching this subject
    pub count_teachers: u32,
    //TODO statistics? total lessons taught?, ...
}

impl SubjectConfig {
    pub const LEN: usize = std::mem::size_of::<SubjectConfig>() + 8;
}
