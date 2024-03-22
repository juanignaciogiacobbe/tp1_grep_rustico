use core::fmt::Debug;


#[derive(Debug)]
pub struct Period {
    regex_spliteada: Vec<String>
}

pub struct Brackets {
    regex_spliteada: Vec<String>,
    caracteres_bracket_expression: Vec<char>
}

pub trait MetaCaracter{
    fn aplicar_filtro(&self, linea: String) -> String;
}

impl Debug for dyn MetaCaracter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Caracter{{{}}}", self.aplicar_filtro("hola".to_string()))
    }
}

pub fn string_contenido_en_linea(string_buscado: String, linea: String) -> Option<usize> {
    if linea.contains(&string_buscado) {
        Some(linea.find(&string_buscado)?)
    } else {
        None
    }
}

impl Period {
    pub fn new(regular_expression: String) -> Self{
        let regex_spliteada: Vec<&str> = regular_expression.split('.').collect();
        let mut regex_spliteada_casteada = vec![];

        for string in regex_spliteada {
            regex_spliteada_casteada.push(string.to_string());
        }

        Self { regex_spliteada: regex_spliteada_casteada }
    }
}
    
impl MetaCaracter for Period {
    fn aplicar_filtro(&self, linea: String) -> String {
        let mut string_devuelta = String::new();

        if let Some(posicion) = linea.find(&self.regex_spliteada[0]) {
            let segunda_parte_inicio = posicion + 1 + self.regex_spliteada[0].len();

            if let Some(segunda_parte) = linea.get(segunda_parte_inicio..) {
                if segunda_parte.starts_with(&self.regex_spliteada[1]) {
                    println!("Matchea!");
                    string_devuelta = linea;
                }
            }
        }

        string_devuelta
    }
}

impl Brackets {
    pub fn new(regular_expression: String) -> Self {
        let regex_spliteada: Vec<&str> = regular_expression
        .split(|c: char| c == '[' || c == ']')
        .collect();

        let mut regex_spliteada_casteada = vec![];
        let mut caracteres_bracket_expression = vec![];

        for string in regex_spliteada {
            regex_spliteada_casteada.push(string.to_string());
        }

        for char in regex_spliteada_casteada[1].chars() {
            caracteres_bracket_expression.push(char);
        }

        println!("{:?}", caracteres_bracket_expression);
        Self { regex_spliteada: regex_spliteada_casteada, caracteres_bracket_expression: caracteres_bracket_expression }
    }

    //pub fn buscar_caracter_en_linea(linea: String) -> Option<usize> {
      //  if linea.contains(&string_buscado) {
        //    Some(linea.find(&string_buscado)?)
        //} else {
          //  None
        //}
    //}
}

impl MetaCaracter for Brackets {
    fn aplicar_filtro(&self, linea: String) -> String {
        let mut string_devuelta = String::new();
        
        if let Some(posicion) = string_contenido_en_linea(self.regex_spliteada[0].clone(), linea) {
    //        let segunda_parte_inicio = posicion + 1 + self.regex_spliteada[0].len();

      //      if let Some(segunda_parte) = linea.get(segunda_parte_inicio..) {
//
  //          }
            
            println!("matchea!");
        }

        string_devuelta
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_creo_un_period_y_aplico_su_filtro_a_una_linea_dada() {
        let period = Period::new("ju.an".to_string());

        let linea_a_filtrar = "juaanito juan".to_string();


        assert_eq!(period.aplicar_filtro(linea_a_filtrar), "juaanito juan".to_string())

    }

    #[test]
    fn test_02_creo_una_bracket_expression_y_aplico_su_filtro_a_una_linea_dada() {
        let brackets = Brackets::new("j[ua]n".to_string());

        let linea_a_filtrar = "juaanito jun".to_string();


        assert_eq!(brackets.aplicar_filtro(linea_a_filtrar), "juaanito jun".to_string())

    }
}
