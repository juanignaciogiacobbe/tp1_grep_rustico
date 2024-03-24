use std::{fs::File, io::{BufRead, BufReader, Lines}};

use crate::custom_error::CustomError;

#[derive(Debug)]
pub struct FileProcessor {
    archivo: File
}

impl FileProcessor {
    pub fn build(ruta_archivo: &str) -> Result<FileProcessor, CustomError> { 

        let archivo = match File::open(ruta_archivo.to_string()) {
            Ok(archivo) => archivo,
            Err(_err) => return Err(CustomError::ArchivoNoEncontrado)
        };

        Ok(FileProcessor { archivo })
    }

    pub fn obtener_lecturas(&self) -> Result<Vec<String>, CustomError> {
        let reader: Lines<BufReader<&File>> = BufReader::new(&self.archivo).lines();

        let lecturas = FileProcessor::leer_archivo(reader)?;

        Ok(lecturas)
    }

    fn leer_archivo(reader: Lines<BufReader<&File>>) -> Result<Vec<String>, CustomError>{
        let mut lecturas: Vec<String> = vec![];

        for linea in reader {
            match linea {
                Ok(linea) => lecturas.push(linea),
                Err(_err) => return Err(CustomError::ErrorEnLecturaDelArchivo)
            };
        }

        Ok(lecturas)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_01_ingreso_una_ruta_valida_y_el_processor_se_crea_correctamente() {
        //NOTA: Para correr este test, tengo el directorio 'test', y dentro tengo a data.txt
        let file_processor = FileProcessor::build("./test/data.txt");
        
        assert!(file_processor.is_ok());
    }

    #[test] 
    fn test_02_ingreso_una_ruta_invalida_y_el_processor_arroja_un_error() {
        //NOTA: Para que este test cumpla su proposito, la ruta NO tiene que existir
        let file_processor = FileProcessor::build("./hola/aca/estan/los/datos/data.txt");
        
        assert!(file_processor.is_err());
    }

    #[test]
    fn test_03_ingreso_una_ruta_valida_y_solicito_que_se_realice_la_lectura_del_archivo() {
        let file_processor: Result<FileProcessor, CustomError> = FileProcessor::build("./test/data.txt");

        assert!(file_processor.expect("hola").obtener_lecturas().is_ok());
    }
}    