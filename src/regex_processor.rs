

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
    pub fn new(regular_expression: String) -> Self {

        for caracter in regular_expression.chars() {
            match caracter {
                '.' => println!("Encontre un period!"),
                _ => println!("Encontre un caracter normal")
            }
        }

        Self { regular_expression }
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
    fn test_01_creo_un_processor_dada_una_regex() {
        let regex_processor = RegexProcessor::new("abcd".to_string());

        assert_eq!(regex_processor.filtrar_lineas(vec!["abcdefg".to_string(), "fedcba".to_string()]), vec!["abcdefg".to_string()])
    }
}
