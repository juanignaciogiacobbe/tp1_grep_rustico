use crate::caracter::Caracter;
use crate::caracter::definir_caracter;

#[derive(Debug)]
pub struct RegexProcessor {
    caracteres: Vec<Box<dyn Caracter>>,
}


impl RegexProcessor {
    pub fn new(regular_expression: String) -> Self {
        let mut vector_caracteres: Vec<Box<dyn Caracter>> = vec![];

        for caracter in regular_expression.chars() {
            vector_caracteres.push(definir_caracter(caracter));
        }

        Self {
            caracteres: vector_caracteres
        }
    }

    pub fn filtrar(palabras_a_filtrar: Vec<String>) -> Vec<String> {
        let resultado_esperado = vec![];

        //for palabra in palabras_a_filtrar {

        //}

        resultado_esperado
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::regex_processor::RegexProcessor;

    #[test]
    fn test_01_dado_un_vector_de_palabras_utilizo_un_regex_processor_para_filtrarlas() {
        let regex_processor = RegexProcessor::new("juan".to_string());

        let mut palabras_a_filtrar = vec!["juan".to_string(), "juancito".to_string(), "juaanito".to_string(), "joan".to_string()];

        let resultado_esperado = vec!["juan", "juancito"];

        let resultado_filtro = RegexProcessor::filtrar(palabras_a_filtrar);

        assert_eq!(resultado_filtro, resultado_esperado);
    }

}