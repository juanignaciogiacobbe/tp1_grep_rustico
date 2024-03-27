use std::collections::VecDeque;

use crate::{
    cantidad_repeticiones::{self, CantidadRepeticiones}, caracter::Caracter, custom_error::CustomError, evaluacion_paso::EvaluacionPaso, paso_regex::PasoRegex
};

#[derive(Debug)]
pub struct RegularExpression {
    pasos_regex: Vec<PasoRegex>
}

impl RegularExpression {
    pub fn new(expression: &str) -> Result<Self, CustomError> {
        let mut pasos_regex: Vec<PasoRegex> = Vec::new();

        let mut caracteres_iter = expression.chars();

        while let Some(c) = caracteres_iter.next() {
            let paso = match c {
                '.' => Some(PasoRegex { 
                    caracter: Caracter::Comodin, 
                    cantidad_repeticiones: CantidadRepeticiones::Rango { minimo: Some(0), maximo: Some(1) } 
                }),
                'a'..='z' => Some(PasoRegex {
                    caracter: Caracter::Literal(c),
                    cantidad_repeticiones: CantidadRepeticiones::Rango { minimo: Some(0), maximo: Some(1) }
                }),
                '*' => {
                    if let Some(anterior) = pasos_regex.last_mut() {
                        anterior.cantidad_repeticiones = CantidadRepeticiones::Cualquiera;
                    } else {
                        return Err(CustomError::UsoErroneoDelAsterisk)
                    }

                    None
                },
                '+' => {
                    if let Some(anterior) = pasos_regex.last_mut() {
                        anterior.cantidad_repeticiones = CantidadRepeticiones::Rango { minimo: Some(1), maximo: None };
                    } else {
                        return Err(CustomError::UsoErroneoDelAsterisk)
                    }

                    None
                }
                '?' => {
                    if let Some(anterior) = pasos_regex.last_mut() {
                        anterior.cantidad_repeticiones = CantidadRepeticiones::Rango { minimo: Some(0), maximo: Some(1) };
                    } else {
                        return Err(CustomError::UsoErroneoDelQuestionSign)
                    }

                    None
                }
                '\\' => { match caracteres_iter.next() {
                    Some(literal) => {
                        Some(PasoRegex { 
                            caracter: Caracter::Literal(literal), 
                            cantidad_repeticiones: CantidadRepeticiones::Cantidad(1)  
                        })

                    },
                    None => return Err(CustomError::CaracterInesperado) 
                    }
                },
                _ => return Err(CustomError::CaracterInesperado) 
            };

            if let Some(paso) = paso {
                pasos_regex.push(paso);
            }

        }

        Ok(Self { pasos_regex })

    }

    pub fn validar_linea(self, linea: &str) -> Result<bool, CustomError> {
        if !linea.is_ascii() {
            return Err(CustomError::LineaNoAscii)
        }

        let mut queue: VecDeque<PasoRegex> = VecDeque::from(self.pasos_regex);
        let mut stack: Vec<EvaluacionPaso> = Vec::new();
        let mut index = 0;

        
        'pasos: while let Some(paso) = queue.pop_front() {
            match paso.cantidad_repeticiones {
                CantidadRepeticiones::Cantidad(n) => {
                    let mut match_size = 0;
                    
                    for _ in 0..n {
                        let avance = paso.caracter.validar_coincidencia(&linea[index..]);

                        if avance == 0 { //puedo encontrar la coincidencia mas adelante
                            match backtrack(paso, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    
                                    continue 'pasos;
                                }
                                None => {
                                    //index += avance;
                                    return Ok(false)
                                }
                            };
                        } else {  
                            match_size += avance;
                            index += avance; 
                        }
                        
                        // if avance == 0 && index == linea.len() { //aca ya me quede sin caracteres en la linea, no pude matchear
                        //     return Ok(false)
                        // }
                    }

                    stack.push(EvaluacionPaso {
                        paso: paso,
                        match_size,
                        backtrackeable: false
                    })
                },
                CantidadRepeticiones::Cualquiera => {
                    let mut sigo_avanzando = true;
                    while sigo_avanzando {  
                        let avance = paso.caracter.validar_coincidencia(&linea[index..]);
                        
                        if avance != 0 {
                            index += avance;
                            stack.push(EvaluacionPaso {
                                paso: paso.clone(),
                                match_size: avance,
                                backtrackeable: true
                            })
                        } else {
                            sigo_avanzando = false;
                        }

                    }
                },
                CantidadRepeticiones::Rango { minimo, maximo } => {
                    let min = match minimo {
                        Some(min) => min,
                        None => 0,
                    };

                    let max = match maximo {
                        Some(max) => max,
                        None => linea.len(),
                    };

                    let mut match_size = 0;
                    
                    for _ in min..max {
                        let avance = paso.caracter.validar_coincidencia(&linea[index..]);

                        if avance == 0 { //puedo encontrar la coincidencia mas adelante
                            match backtrack(paso, &mut stack, &mut queue) {
                                Some(size) => {
                                    index -= size;
                                    
                                    continue 'pasos;
                                }
                                None => {
                                    //index += avance;
                                    return Ok(false)
                                }
                            };
                        } else {  
                            match_size += avance;
                            index += avance; 
                        }
                        
                        // if avance == 0 && index == linea.len() { //aca ya me quede sin caracteres en la linea, no pude matchear
                        //     return Ok(false)
                        // }
                    }

                    stack.push(EvaluacionPaso {
                        paso: paso,
                        match_size,
                        backtrackeable: false
                    })
                },
            }
        }

        Ok(true)
    }

}


fn backtrack(paso_actual: PasoRegex, evaluados: &mut Vec<EvaluacionPaso>, siguiente: &mut VecDeque<PasoRegex>) -> Option<usize> {
    let mut size_anterior: usize = 0;
    siguiente.push_front(paso_actual);

    while let Some(e) = evaluados.pop() {
        size_anterior += e.match_size;

        if e.backtrackeable {
            return Some(size_anterior);
        } else {
            siguiente.push_front(e.paso);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_evaluo_una_regex_creada_con_asterisk() {
        let regex = RegularExpression::new("ab*cd");

        assert_eq!(regex.unwrap().validar_linea("abbbbbcd").unwrap(), true);
        //assert_eq!(regex.unwrap().validar_linea("abbbbbcd").unwrap(), true);
       // assert_eq!(regex.unwrap().validar_linea("aacd").unwrap(), false);
    }

    #[test]
    fn test_02_evaluo_una_regex_creada_con_question_sign() {
        let regex = RegularExpression::new("ab?cd");

        assert_eq!(regex.unwrap().validar_linea("abbcd").unwrap(), true);
       // assert_eq!(regex.unwrap().validar_linea("acd").unwrap(), true);

    }

    #[test]
    fn test_03_evaluo_una_regex_creada_con_plus() {
        let regex = RegularExpression::new("ab+cd");

        //assert_eq!(regex.unwrap().validar_linea("abcd").unwrap(), true);
        assert_eq!(regex.unwrap().validar_linea("acd").unwrap(), true);

    }
}