use crate::{
    cantidad_repeticiones::CantidadRepeticiones, 
    caracter::Caracter
};

#[derive(Debug, Clone)]
pub struct PasoRegex{
    pub(crate) caracter: Caracter,
    pub(crate) cantidad_repeticiones: CantidadRepeticiones,
}