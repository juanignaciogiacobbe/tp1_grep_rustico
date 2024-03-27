use crate::clase_regex::ClaseRegex;

#[derive(Debug, Clone)]
pub enum Caracter {
    Literal(char),
    Comodin,
    Clase(ClaseRegex)
}

impl Caracter {
    pub fn validar_coincidencia(&self, linea: &str) -> usize {
        match self {
            Caracter::Literal(caracter) => {
                if linea.chars().next() == Some(*caracter) {
                    caracter.len_utf8()
                } else {
                    0
                }
            },
            Caracter::Comodin => {
                if let Some(caracter) = linea.chars().next() {
                    caracter.len_utf8()
                } else {
                    0
                }
            },
            Caracter::Clase(_) => 0,
        }
    }
}