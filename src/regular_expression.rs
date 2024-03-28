use std::str::Chars;

use crate::{
    clase_regex::ClaseRegex, custom_error::CustomError, linea::Linea, paso_regex::PasoRegex, repeticiones::Repeticiones, resultado_validacion::ResultadoValidacion, valor::Valor
};

const PERIOD: char = '.';
const CARET: char = '^';
const DOLLAR_SIGN: char = '$';
const ASTERISK: char = '*';
const PLUS: char = '+';
const QUESTION_SIGN: char = '?';
const OPEN_CURVY_BRACKET: char = '{';
const CLOSE_CURVY_BRACKET: char = '}';
const OPEN_BRACKET: char = '[';
const CLOSE_BRACKET: char = ']';

pub struct RegularExpression {
    pasos: Vec<PasoRegex>
}

pub fn matchear_por_cantidad_de_repeticiones(pasos: &mut Vec<PasoRegex>, mut index_pasos: usize, mut linea: Linea, lectura: &str) -> Result<bool, CustomError> {
    while let Some(caracter) = linea.siguiente_caracter() {
        
        let paso = &pasos[index_pasos];

        match paso.repeticiones {
            Repeticiones::CantidadExacta(n) => {
                let mut cantidad_repeticiones = 0;

                for _ in 0..n {
                    match paso.valor.validar_coincidencia(&linea) {
                        ResultadoValidacion::Encontrado { avance } => {
                            cantidad_repeticiones += 1;
                            linea.actualizar_index(avance);
                        },
                        ResultadoValidacion::NoEncontrado { avance } => {
                            linea.actualizar_index(avance);
                            index_pasos = 0; //Reinicio el index porque no encontre coincidencia.
                            break; //No me interesa seguir iterando en este punto(es mas, podria romper el matching completo :D).
                        },
                    }
                }

                if cantidad_repeticiones == n {
                    index_pasos += 1;
                }

                if index_pasos == pasos.len() {
                    return Ok(true)
                }
            },

            Repeticiones::Indefinida => {1;},
            Repeticiones::Rango { minimo, maximo } => {
                let min = match minimo {
                    Some(min) => min,
                    None => 0, //Si no viene definido lo tengo que tomar como 0.
                };
                
                let max = match maximo {
                    Some(max) => max,
                    None => lectura.len() - linea.index_actual, //Si no viene definido, asumo que puede repetirse la cantidad de veces que
                                                                //el resto de la linea me deje.
                };

                let mut cantidad_repeticiones = 0;

                match &paso.valor {
                    Valor::Literal { valor, clase } => {
                        for _ in min..max {
                            match paso.valor.validar_coincidencia(&linea) {
                                ResultadoValidacion::Encontrado { avance } => {
                                    cantidad_repeticiones += 1;
                                    linea.actualizar_index(avance);
                                },
                                ResultadoValidacion::NoEncontrado { avance } => {
                                    break; //No me interesa seguir iterando en este punto(es mas, podria romper el matching completo :D).
                                },
                            }
                        }
    
                        if cantidad_repeticiones >= min && cantidad_repeticiones <= max {
                            index_pasos += 1;
                        } else {
                            index_pasos = 0;
                        }
    
                        if index_pasos == pasos.len() {
                            return Ok(true)
                        }
                    },
                    Valor::Period => {
                        if index_pasos + 1 == pasos.len() {
                            return Ok(true) //si el ultimo paso es un .*, SIEMPRE voy a matchear
                        }

                        if max == lectura.len() - linea.index_actual {
                            let resto_linea = match Linea::new(&lectura[linea.index_actual..]) {
                                Ok(resto_linea) => resto_linea,
                                Err(err) => return Err(err),
                            };

                            let pasos_restantes: &mut Vec<PasoRegex> = pasos;

                            let mut i = 0;

                            while i < index_pasos + 1 {
                                pasos_restantes.remove(0);
                                i += 1;
                            }

                            let resultado_resto = matchear_por_cantidad_de_repeticiones(pasos_restantes, 0, resto_linea, &lectura[linea.index_actual..]);

                            let resultado = match resultado_resto {
                                Ok(resultado) => resultado,
                                Err(err) => return Err(err),
                            };

                            match resultado {
                                true => return Ok(true),
                                false => return Ok(false),
                            }
                        }
                    },
                }
            },
        };

    }
return Ok(false);
}

pub fn definir_bracket_expression(caracteres_iter: &mut Chars<'_>, bracket_cierre: char) -> Result<Vec<char>, CustomError> {
    let mut contenido_bracket = Vec::new();

    while let Some(caracter) = caracteres_iter.next() {
        if caracter == bracket_cierre {
            return Ok(contenido_bracket);
        }
        contenido_bracket.push(caracter);
    }

    return Err(CustomError::UsoErroneoDelCurvyBracket)
}

impl RegularExpression {
    pub fn new(expression: &str) -> Result<RegularExpression, CustomError>{
        let mut pasos: Vec<PasoRegex> = Vec::new();
        let mut caracteres_iter = expression.chars();
        
        while let Some(caracter) = caracteres_iter.next() {
            let paso = match caracter {
                'a'..='z' => Some(PasoRegex {
                    valor: Valor::Literal{
                        valor: caracter,
                        clase: ClaseRegex::Alpha
                    },
                    repeticiones: Repeticiones::CantidadExacta(1)

                }),
                'A'..='Z' => Some(PasoRegex {
                    valor: Valor::Literal{
                        valor: caracter,
                        clase: ClaseRegex::Alpha
                    },
                    repeticiones: Repeticiones::CantidadExacta(1)
                
                }),
                PERIOD => Some(PasoRegex {
                    valor: Valor::Period,
                    repeticiones: Repeticiones::CantidadExacta(1)
                
                }),
                ASTERISK => {
                    if let Some(anterior) = pasos.last_mut() {
                        anterior.repeticiones = Repeticiones::Rango { minimo: Some(0), maximo: None }
                    } else {
                        return Err(CustomError::UsoErroneoDelAsterisk)
                    }

                    None
                
                },
                PLUS => {
                    if let Some(anterior) = pasos.last_mut() {
                        anterior.repeticiones = Repeticiones::Rango { minimo: Some(1), maximo: None }
                    } else {
                        return Err(CustomError::UsoErroneoDelPlus)
                    }

                    None
                
                },
                QUESTION_SIGN => {
                    if let Some(anterior) = pasos.last_mut() {
                        anterior.repeticiones = Repeticiones::Rango { minimo: Some(0), maximo: Some(1) }
                    } else {
                        return Err(CustomError::UsoErroneoDelQuestionSign)
                    }

                    None                
                },
                // CARET => {
                //         if let Some(_) = pasos.last() {
                //             return Err(CustomError::UsoErroneoDelCaret)
                //         } else {
                //             println!("soy un anchoring!");
                //         }
    
                //         None
                // },
                // DOLLAR_SIGN => {
                //     if let Some(_) = caracteres_iter.next() {
                //         return Err(CustomError::UsoErroneoDelDollarSign)
                //     } else {
                //         println!("soy un dollar sign!");
                //     }

                //     None
                // },
                // OPEN_CURVY_BRACKET => {
                //     if let Some(anterior) = pasos.last_mut() {
                //         match definir_bracket_expression(&mut caracteres_iter, CLOSE_CURVY_BRACKET) {
                //             Ok(contenido_bracket) => println!("{:?}", contenido_bracket),
                //             Err(err) => return Err(err),
                //         }
                //     } else {
                //         return Err(CustomError::UsoErroneoDelCurvyBracket)
                //     }

                //     None
                // },
                // OPEN_BRACKET => {
                //     if let Some(anterior) = pasos.last_mut() {
                //         match definir_bracket_expression(&mut caracteres_iter, CLOSE_BRACKET) {
                //             Ok(contenido_bracket) => println!("{:?}", contenido_bracket),
                //             Err(err) => return Err(err),
                //         }
                //     } else {
                //         return Err(CustomError::UsoErroneoDelPlus)
                //     }

                //     None
                // }
                _ => return Err(CustomError::CaracterInesperado)
            };

            if let Some(paso) = paso {
                pasos.push(paso);
            }
        }

        println!("{:?}", pasos);
        Ok(Self { pasos })
    }

    pub fn filtrar_lectura(&mut self, lectura: &str) -> Result<bool, CustomError> {
        let linea = match Linea::new(lectura) {
            Ok(linea) => linea,
            Err(err) => return Err(err),
        };

        let index_pasos: usize = 0; //Voy a iterar sobre el vector de pasos de la regex.
        let resultado = match matchear_por_cantidad_de_repeticiones(&mut self.pasos, index_pasos, linea, lectura) {
            Ok(resultado) => resultado,
            Err(err) => return Err(err),
        };

        match resultado {
            true => Ok(true),
            false => Ok(false),
        }

        // if index_pasos == self.pasos.len() {
        //     return Ok(true);
        // } pal dollar sign :D
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_regex_con_caracteres_normales() {
        let regex = RegularExpression::new("abcd");

        assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abcdefg").unwrap(), true);
    }

    #[test]
    fn test_02_regex_con_caracteres_normales_y_period() {
        let regex = RegularExpression::new("ab.cd");

        assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abzcdefg").unwrap(), true);
    }

    #[test]
    fn test_03_regex_con_caracteres_normales_y_asterisk() {
        let regex = RegularExpression::new("ab*cd");

        assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abcde").unwrap(), true);
        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: acde").unwrap(), true);
        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abbbbbcde").unwrap(), true);
    }

    #[test]
    fn test_04_regex_con_caracteres_normales_y_plus() {
        let regex = RegularExpression::new("ab+cd");

        assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abcde").unwrap(), true);
        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: acde").unwrap(), false);
        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abbbbbcde").unwrap(), true);
    }

    #[test]
    fn test_05_regex_con_caracteres_normales_y_question_sign() {
        let regex = RegularExpression::new("ab?cd");

        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abcde").unwrap(), true);
        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: acde").unwrap(), true);
        assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abbbbbcde").unwrap(), false);
    }

    #[test]
    fn test_06_regex_con_combinacion_punto_asterisco() {
        let regex = RegularExpression::new("ab.*cd");

        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abcde").unwrap(), true);
        //assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: acde").unwrap(), true);
        assert_eq!(regex.unwrap().filtrar_lectura("juan dice el abecedario: abacsdkahsdjahscdefgg").unwrap(), true);
    }
}