use crate::{custom_error::CustomError, expresion::{Expresion, ExpresionNormal}};

const PERIOD: char = '.';
const CARET: char = '^';
const ASTERISK: char = '*';

pub fn validar_metacaracter(regular_expression: &str, caracter: char) -> bool {
    if let Some(indice) = regular_expression.chars().position(|c| c == caracter) {
        if indice > 0 {
             if let Some(caracter_anterior) = regular_expression.chars().nth(indice - 1) {
                if caracter_anterior == '\\' {
                    println!("NO soy un metacaracter, me precede un backslash {}", caracter_anterior);
                    return false;
                }
             }
        }
    }

    true
}

//la idea es que le ingresemos la regex, y que sea
//capaz de conformar el filtro.
//ademas, le pasaremos las lecturas del archivo
//y sera el encargado de aplicar el filtro.
#[derive(Debug)]
pub struct RegexProcessor {
    regular_expression: String,
    //expresiones: Vec<Box<dyn Expresion>> 
}

impl RegexProcessor {
    //para procesar la regex dada: busco en ella 
    //metacaracteres. Si encuentro metacaracteres,
    //tengo que actualizar el filtro.
    pub fn build(regular_expression: &str) -> Result<RegexProcessor, CustomError> {
        
        //Este vector contendra una separacion de la regex en pequenias
        //expresiones.
        let mut expresiones: Vec<String> = vec!["".to_string()];
        let mut index_expresiones: usize = 1;

        let largo_regex: usize = regular_expression.len();

        let mut i: usize = 0;
        let caracteres_regex: Vec<char> = regular_expression.chars().collect();

        while i < largo_regex {
            if caracteres_regex[i] == ASTERISK {
                //index_expresiones += 1;
                if index_expresiones > 0 {
                    let mut repetition = String::new();
                    let caracter = caracteres_regex[i - 1];
                   // expresiones[index_expresiones - 1].remove(expresiones[index_expresiones - 1].len() - 1);

                    repetition.push(caracter);
                    repetition.push(caracteres_regex[i]);

                    expresiones.push(repetition);

                    if index_expresiones != 1 {
                        index_expresiones += 1;
                    }
                }
            } else {
                if index_expresiones == 1 {
                    expresiones[index_expresiones - 1].push(caracteres_regex[i]);
                } else {
                    expresiones[index_expresiones].push(caracteres_regex[i]);
                }
            }
            i += 1;
        }

        println!("{:?}", expresiones);
        
        


       // for caracter in regular_expression.chars() {
         //   match caracter {
                //PERIOD => validar_metacaracter(regular_expression, caracter),
                // CARET => validar_metacaracter(regular_expression, caracter),
           //     ASTERISK => println!("Soy un asterisk!"),
             //   _ => println!("Encontre un caracter normal")
            //}
        //}

        //si mi vector de expresiones termina vacio
        //quiere decir que no encontre ningun metacaracter :D
       // if separacion_expresiones.len() == 0 {
         //   separacion_expresiones.push(Box::new(ExpresionNormal::new(regular_expression)));
        //}

        Ok(RegexProcessor {
            regular_expression: regular_expression.to_string(), 
        //    expresiones: expresiones
        })
    }

   // pub fn filtrar_lecturas(&self, lineas: Vec<String>) -> Vec<String> {
     //   let mut resultado_filtro: Vec<String> = vec![];
                
       // for linea in lineas {
         //   for i in 0..self.separacion_expresiones.len() {
           //     if self.separacion_expresiones[i].filtrar_linea(&linea) {
             //       resultado_filtro.push(linea.to_string());
               // }
            //}   
        //}

        //println!("{:?}", resultado_filtro);
        //resultado_filtro
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_filtro_dada_una_regex_compuesta_de_caracteres_normales() {
        let regex_processor = RegexProcessor::build("abcd");

     //   assert_eq!(regex_processor.unwrap().filtrar_lecturas(vec!["abcdefg".to_string(), "fedcba".to_string()]), vec!["abcdefg".to_string()]);
    }

    #[test]
    fn test_02_filtro_dada_una_regex_con_un_metacaracter() {
        let regex_processor = RegexProcessor::build("ab.cd");

        //assert_eq!(regex_processor.unwrap().filtrar_lecturas(vec!["juan dice el abecedario: abccdefg".to_string(), "fedcba".to_string()]), vec!["juan dice el abecedario: abccdefg".to_string()]);
    }

    #[test]
    fn test_03_filtro_dada_una_regex_con_caret_y_asterisk() {
        let regex_processor = RegexProcessor::build("^ab*cd");

        //assert_eq!(regex_processor.unwrap().filtrar_lecturas(vec!["abccdefg juan".to_string(), "fedcba".to_string()]), vec!["abccdefg juan".to_string()]);
    }
}
