use crate::{
    custom_error::CustomError, 
    expresion::{self, Expresion, ExpresionNormal}
};

//const PERIOD: char = '.';
const CARET: char = '^';
const ASTERISK: char = '*';
const PLUS: char = '+';
const QUESTION_SIGN: char = '?';
const OPEN_CURVY_BRACKET: char = '{';
const CLOSE_CURVY_BRACKET: char = '}';



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
        RegexProcessor::procesar_expresion(regular_expression);

        Ok(RegexProcessor {
            regular_expression: regular_expression.to_string(), 
        //    expresiones: expresiones
        })
    }

    fn procesar_expresion(regular_expression: &str) {
        //Este vector contendra una separacion de la regex en pequenias
        //expresiones.
        let mut expresiones: Vec<String> = vec!["".to_string()];
        let mut index_expresiones: usize = 0;

        let caracteres_regex: Vec<char> = regular_expression.chars().collect();
        let mut i: usize = 0;

        while i < caracteres_regex.len() {
            if caracteres_regex[i] == ASTERISK || caracteres_regex[i] == PLUS || caracteres_regex[i] == QUESTION_SIGN {
                let mut expresion_anterior_modificada = expresiones[index_expresiones].to_string();
                expresion_anterior_modificada = expresion_anterior_modificada[0..expresion_anterior_modificada.len() - 1].to_string();

                expresiones[index_expresiones] = expresion_anterior_modificada;
                
                let mut nueva_expresion = String::new();
                
                nueva_expresion.push(caracteres_regex[i - 1]);
                nueva_expresion.push(caracteres_regex[i]);
                
                expresiones.push(nueva_expresion);
                expresiones.push("".to_string());
                index_expresiones += 2;

            } else if caracteres_regex[i] == OPEN_CURVY_BRACKET {
                let mut contenido_dentro_del_bracket:String = String::new();

                while caracteres_regex[i] != CLOSE_CURVY_BRACKET && i < caracteres_regex.len() - 1 {
                    contenido_dentro_del_bracket.push(caracteres_regex[i]);
                    i += 1;
                }

                contenido_dentro_del_bracket.push(caracteres_regex[i]);
                
                expresiones.push(contenido_dentro_del_bracket);
                expresiones.push("".to_string());
                index_expresiones += 2;
                
            } else {
                expresiones[index_expresiones].push(caracteres_regex[i]);
            }
            i += 1;
        }

        println!("{:?}", expresiones);

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
        let regex_processor = RegexProcessor::build("a+hola*{abc}{holi}");

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
