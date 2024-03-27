use crate::{
    clase_regex::ClaseRegex, 
    linea::Linea, 
    resultado_validacion::ResultadoValidacion
};

#[derive(Debug)]
pub enum Valor {
    Literal { 
        valor: char, 
        clase: ClaseRegex 
    },
    Period,
}

impl Valor {
    pub fn validar_coincidencia(&self, linea: &Linea) -> ResultadoValidacion {
        if let Some(caracter_a_comparar) = linea.siguiente_caracter() {
            match self {
                Valor::Literal { valor, clase } => {
                    if caracter_a_comparar == *valor {
                        return ResultadoValidacion::Encontrado { avance: caracter_a_comparar.len_utf8() };
                    } else {
                        return ResultadoValidacion::NoEncontrado { avance: caracter_a_comparar.len_utf8() };
                    }
                },
                Valor::Period => {
                    return ResultadoValidacion::Encontrado { avance: caracter_a_comparar.len_utf8() };
                },
            }
        }

        return ResultadoValidacion::NoEncontrado { avance: 3 };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_valido_coincidencia_entre_dos_caracteres() {
        let valor = Valor::Literal { valor: 'a', clase: ClaseRegex::Alpha };

        let linea: Linea = Linea::new("abcd").unwrap();

        assert_eq!(valor.validar_coincidencia(&linea), ResultadoValidacion::Encontrado { avance: 1 });

    }
}
