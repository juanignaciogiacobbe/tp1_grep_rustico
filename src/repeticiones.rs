#[derive(Debug)]
pub enum Repeticiones {
    CantidadExacta(usize),
    Indefinida,
    Rango {
        minimo: Option<usize>,
        maximo: Option<usize>
    }
}