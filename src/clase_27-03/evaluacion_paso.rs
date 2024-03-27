use crate::paso_regex::PasoRegex;

pub struct EvaluacionPaso {
    pub(crate) paso: PasoRegex,
    pub(crate) match_size: usize,
    pub(crate) backtrackeable: bool
}