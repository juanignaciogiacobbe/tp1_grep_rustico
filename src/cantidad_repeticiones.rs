#[derive(Debug, Clone)]
pub enum CantidadRepeticiones {
    Cantidad(usize),
    Cualquiera,
    Rango{ 
        minimo: Option<usize>, 
        maximo: Option<usize> 
    }
}