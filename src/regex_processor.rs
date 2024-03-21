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

    //pub fn get_vector_caracteres(&self) -> Vec<Box<dyn Caracter>> {
        //let mut vector_caracteres = vec![];

        //for c in self.caracteres {
          //  vector_caracteres.push(c)
        //}

        //vector_caracteres

      //  self.caracteres.clone()
    //}
}

#[cfg(test)]
mod tests {
   // use super::*;

    //#[test]
    //fn test_01_ingreso_una_regex_cualquiera_y_el_reader_se_crea_correctamente() {
      //  let regex_reader = RegexProcessor::new("hola".to_string());

    //    let caracter_1: Box<dyn Caracter> = definir_caracter('h');
    //    let caracter_2: Box<dyn Caracter> = definir_caracter('o');
     //   let caracter_3: Box<dyn Caracter> = definir_caracter('l');
    //    let caracter_4: Box<dyn Caracter> = definir_caracter('a');

    //    let vector_caracteres_esperado = vec![caracter_1, caracter_2, caracter_3, caracter_4];

        //assert_eq!(regex_reader.get_vector_caracteres(), vector_caracteres_esperado);
    //}

}