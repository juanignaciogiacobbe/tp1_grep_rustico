use crate::metacaracter::{MetaCaracter,  Period};
//use crate::caracter::definir_caracter;

#[derive(Debug)]
pub struct RegexProcessor {
    //caracteres: Vec<Box<dyn Caracter>>,
    metacaracteres: Vec<Box<dyn MetaCaracter>>,
    filtro_caracteres_normales: String,
}


impl RegexProcessor {
    pub fn new(regular_expression: String) -> Self {
        let mut vector_metacaracteres: Vec<Box<dyn MetaCaracter>> = vec![];

        for caracter in regular_expression.chars() {
            match caracter {
                '.' => vector_metacaracteres.push(Box::new(Period::new(regular_expression.clone()))),
                _ => ()
            };

        }
            
        Self { filtro_caracteres_normales: regular_expression, metacaracteres: vector_metacaracteres }
    }

    pub fn filtrar(&self, strings_a_filtrar: Vec<String>) -> Vec<String> {
        let mut resultado_filtro = vec![];

        for linea in &strings_a_filtrar {
            if linea.contains(&self.filtro_caracteres_normales) {
                resultado_filtro.push(linea.to_string());
            }
        }

        if self.metacaracteres.len() > 0 {
            for linea in &strings_a_filtrar {
                if !self.metacaracteres[0].aplicar_filtro(linea.to_string()).is_empty() {
                    resultado_filtro.push(self.metacaracteres[0].aplicar_filtro(linea.to_string()));

                }
            }
        }   

        resultado_filtro
    }

}

#[cfg(test)]
mod tests {
   // use super::*;
    use crate::regex_processor::RegexProcessor;

    #[test]
    fn test_01_dado_un_vector_de_strings_y_una_regex_utilizo_un_regex_processor_para_filtrarlos() {
        let regex_processor = RegexProcessor::new("juan".to_string());

        let strings_a_filtrar = vec!["juan es un crack".to_string(), "juancito citojuan".to_string(), "juaanito juan".to_string(), "joan".to_string()];

        let resultado_filtro = regex_processor.filtrar(strings_a_filtrar);

        assert_eq!(resultado_filtro, vec!["juan es un crack".to_string(), "juancito citojuan".to_string(), "juaanito juan".to_string()]);
    }

    #[test]
    fn test_02_dado_otro_vector_de_strings_y_una_regex_con_un_metacaracter_utilizo_un_regex_processor_para_filtrarlos() {
        let regex_processor = RegexProcessor::new("ju.an".to_string());

        let strings_a_filtrar = vec!["juan es un crack".to_string(), "juancito citojuan".to_string(), "juaanito juan".to_string(), "joan".to_string()];

        let resultado_filtro = regex_processor.filtrar(strings_a_filtrar);

        assert_eq!(resultado_filtro, vec!["juaanito juan".to_string()]);
    }

}