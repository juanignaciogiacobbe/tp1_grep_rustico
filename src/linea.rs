use crate::custom_error::CustomError;

#[derive(Debug, PartialEq)]
pub struct Linea {
    caracteres: Vec<char>,
    pub(crate) index_actual: usize
}

impl Linea {
    pub fn new(contenido: &str) -> Result<Self, CustomError> {
        if !contenido.is_ascii() {
            return Err(CustomError::LineaNoAscii)
        }

        let caracteres = contenido.chars().collect();
        let index_actual = 0;

        Ok(Self {
            caracteres,
            index_actual
        })
    }

    pub fn actualizar_index(&mut self, avance: usize) {
        self.index_actual += avance;
    }

    pub fn siguiente_caracter(&self) -> Option<char> {
        if self.index_actual == self.caracteres.len() {
            None
        } else {
            Some(self.caracteres[self.index_actual])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_creo_una_linea() {
        let mut linea: &mut Linea = &mut Linea::new("abcd").unwrap();
        let avance: usize = 4;

        
        match &linea.siguiente_caracter() {
            Some(caracter) => println!("mi caracter actual es {}", caracter),
            None => println!("parece que se acabo la linea :("),
        };
        linea.actualizar_index(avance);

        match &linea.siguiente_caracter() {
            Some(caracter) => println!("mi caracter actual es {}", caracter),
            None => println!("parece que se acabo la linea :("),
        };
    }
}