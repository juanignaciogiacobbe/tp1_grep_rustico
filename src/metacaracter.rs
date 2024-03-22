use core::fmt::Debug;


#[derive(Debug)]
pub struct Period {
    regex_spliteada: Vec<String>
}
pub trait MetaCaracter{
    fn aplicar_filtro(&self, linea: String) -> String;
}

impl Debug for dyn MetaCaracter {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Caracter{{{}}}", self.aplicar_filtro("hola".to_string()))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01_creo_un_period_y_aplico_su_filtro_a_una_linea_dada() {
        let period = Period::new("ju.an".to_string());

        let linea_a_filtrar = "juaanito juan".to_string();


        assert_eq!(period.aplicar_filtro(linea_a_filtrar), "juaanito juan".to_string())

    }
}
