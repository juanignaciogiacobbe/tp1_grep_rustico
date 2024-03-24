use crate::{custom_error::CustomError, expresion::{Expresion, ExpresionNormal}};

pub fn validar_metacaracter(regular_expression: &str, caracter: char) {
    if let Some(indice) = regular_expression.chars().position(|c| c == caracter) {
        if indice > 0 {
             if let Some(caracter_anterior) = regular_expression.chars().nth(indice - 1) {
                if caracter_anterior == '\\' {
                    println!("NO soy un metacaracter, me precede un backslash {}", caracter_anterior);
                }
             }
        }
    }
}

//la idea es que le ingresemos la regex, y que sea
//capaz de conformar el filtro.
//ademas, le pasaremos las lecturas del archivo
//y sera el encargado de aplicar el filtro.
#[derive(Debug)]
pub struct RegexProcessor {
    regular_expression: String,
    separacion_expresiones: Vec<Box<dyn Expresion>> 
}

impl RegexProcessor {
    //para procesar la regex dada: busco en ella 
    //metacaracteres. Si encuentro metacaracteres,
    //tengo que actualizar el filtro.
    pub fn build(regular_expression: &str) -> Result<RegexProcessor, CustomError> {
        let mut separacion_expresiones: Vec<Box<dyn Expresion>> = vec![];

        //por cada metacaracter encontrado, voy a splitear la regular expression,
        //y voy a crear la expresion correspondiente.
        for caracter in regular_expression.chars() {
            match caracter {
                '^' => println!("Soy un caret!"),
                '*' => println!("Soy un asterisk!"),
                _ => println!("Encontre un caracter normal")
            }
        }

        //si mi vector de expresiones termina vacio
        //quiere decir que no encontre ningun metacaracter :D
        if separacion_expresiones.len() == 0 {
            separacion_expresiones.push(Box::new(ExpresionNormal::new(regular_expression)));
        }

        Ok(RegexProcessor {
            regular_expression: regular_expression.to_string(), 
            separacion_expresiones: separacion_expresiones
        })
    }

    pub fn filtrar_lecturas(&self, lineas: Vec<String>) -> Vec<String> {
        let mut resultado_filtro: Vec<String> = vec![];
                
        for linea in lineas {
            for i in 0..self.separacion_expresiones.len() {
                if self.separacion_expresiones[i].filtrar_linea(&linea) {
                    resultado_filtro.push(linea.to_string());
                }
            }   
        }

        println!("{:?}", resultado_filtro);
        resultado_filtro
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

        assert_eq!(regex_processor.unwrap().filtrar_lecturas(vec!["juan dice el abecedario: abccdefg".to_string(), "fedcba".to_string()]), vec!["juan dice el abecedario: abccdefg".to_string()]);
    }

    //#[test]
    //fn test_03_filtro_dada_una_regex_con_caret_y_asterisk() {
      //  let regex_processor = RegexProcessor::new("^ab*cd");

        //assert_eq!(regex_processor.filtrar_lineas(vec!["abccdefg juan", "fedcba"]), vec!["abccdefg juan"]);
    //}
}
