use crate::expresion::Expresion;

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
pub struct RegexProcessor {
    regular_expression: String
}

impl RegexProcessor {

    //para procesar la regex dada: busco en ella 
    //metacaracteres. Si encuentro metacaracteres,
    //tengo que actualizar el filtro.
    pub fn new(regular_expression: &str) -> Self {
        let mut vector_expresiones: Vec<Box<dyn Expresion>> = vec![];

        //por cada metacaracter encontrado, voy a splitear la regular expression,
        //y voy a crear la expresion correspondiente.
        for caracter in regular_expression.chars() {
            match caracter {
                '*' => println!("Soy un anchoring!"),
                _ => println!("Encontre un caracter normal")
            }
        }

        //si mi vector de expresiones termina vacio
        //quiere decir que no encontre ningun metacaracter :D

        Self { regular_expression: regular_expression.to_string() }
    }

    pub fn filtrar_lineas(&self, lineas: Vec<String>) -> Vec<String> {
        let mut resultado_filtro: Vec<String> = vec![];
        
        
        for linea in lineas {
            if linea.contains(&self.regular_expression) {
                resultado_filtro.push(linea);
            }
        }
        
        resultado_filtro
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_filtro_dada_una_regex_compuesta_de_caracteres_normales() {
        let regex_processor = RegexProcessor::new("abcd");

        assert_eq!(regex_processor.filtrar_lineas(vec!["abcdefg".to_string(), "fedcba".to_string()]), vec!["abcdefg".to_string()])
    }

    #[test]
    fn test_02_filtro_dada_una_regex_con_un_metacaracter() {
        let regex_processor = RegexProcessor::new(r"ab\.cd");

        assert_eq!(regex_processor.filtrar_lineas(vec!["abccdefg".to_string(), "fedcba".to_string()]), vec!["abccdefg".to_string()])
    }
}
