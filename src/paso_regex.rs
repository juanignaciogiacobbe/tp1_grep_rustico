use crate::{
    repeticiones::Repeticiones, 
    valor::Valor
};

#[derive(Debug)]
pub struct PasoRegex {
    pub(crate) valor: Valor,
    pub(crate) repeticiones: Repeticiones
}