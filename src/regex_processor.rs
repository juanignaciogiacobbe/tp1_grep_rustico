use crate::{
    custom_error::CustomError, 
    expresion::{Expresion, ResultadoFiltro}, 
    expression_builder::ExpressionBuilder
};

//const PERIOD: char = '.';
const CARET: char = '^';
const ASTERISK: char = '*';
const PLUS: char = '+';
const QUESTION_SIGN: char = '?';
const OPEN_CURVY_BRACKET: char = '{';
const CLOSE_CURVY_BRACKET: char = '}';
const OPEN_BRACKET: char = '[';
const CLOSE_BRACKET: char = ']';

enum ResultadoFiltroLectura {
    Encontrada,
    NoEncontrada
}


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
    expresiones: Vec<Box<dyn Expresion>> 
}

impl RegexProcessor {
    //para procesar la regex dada: busco en ella 
    //metacaracteres. Si encuentro metacaracteres,
    //tengo que actualizar el filtro.
    pub fn build(regular_expression: &str) -> Result<RegexProcessor, CustomError> {
        let expresiones: Vec<Box<dyn Expresion>> = RegexProcessor::procesar_expresion(regular_expression);

        Ok(RegexProcessor {
          expresiones
        })
    }

    fn procesar_expresion(regular_expression: &str) -> Vec<Box<dyn Expresion>> {
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
                
            } else if caracteres_regex[i] == OPEN_BRACKET {
                let mut contenido_dentro_del_bracket:String = String::new();

                while caracteres_regex[i] != CLOSE_BRACKET && i < caracteres_regex.len() - 1 {
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

        //elimino aquellas expresiones que contengan un string vacio ""
        if expresiones.len() > 1 {
            expresiones.retain(|expresion| expresion != &"".to_string());
        }

        Self::construir_expresiones(expresiones)
        
    }
    
    fn construir_expresiones(expresiones: Vec<String>) -> Vec<Box<dyn Expresion>> { 
        ExpressionBuilder::construir_expresiones(expresiones)
    
    }


    pub fn filtrar_lecturas(&self, lecturas: Vec<String>) -> Vec<String> {
        let mut resultado_filtro: Vec<String> = vec![];
                
        for lectura in lecturas {

            match Self::filtrar_lectura(lectura, self.expresiones) {
                ResultadoFiltroLectura::Encontrada => resultado_filtro.push(lectura),
                ResultadoFiltroLectura::NoEncontrada => ()
            };
        }

        println!("{:?}", resultado_filtro);
        resultado_filtro
    }

    fn filtrar_lectura(lectura: String, expresiones: Vec<Box<dyn Expresion>>) -> ResultadoFiltroLectura {
        //if expresiones.len() == 1 {
            match expresiones[0].filtrar_linea(&lectura) {
                ResultadoFiltro::PosicionFinalEnLinea(index_linea) => ResultadoFiltroLectura::Encontrada,
                ResultadoFiltro::NoEncontrado(index_linea) => ResultadoFiltroLectura::NoEncontrada
            }
        //}
        // let mut index_lectura = 0;

        // let mut index_expresion = 0;

        // while index_lectura < lectura.len() {
        //     match expresiones[index_expresion].filtrar_linea(&lectura) {
        //         ResultadoFiltro::PosicionFinalEnLinea(index_linea) => index_lectura = index_linea,
        //         ResultadoFiltro::NoEncontrado(index_linea) => {index_lectura = index_linea; index_expresion = 0;}
        //     }
        // }


    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_filtro_dada_una_regex_compuesta_de_caracteres_normales() {
        let regex_processor = RegexProcessor::build("abcd");

        assert_eq!(regex_processor.unwrap().filtrar_lecturas(vec!["abcdefg".to_string(), "fedcba".to_string()]), vec!["abcdefg".to_string()]);
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
