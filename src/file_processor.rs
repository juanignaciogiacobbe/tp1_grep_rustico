use std::{fs::File, io::{BufRead, BufReader}};

use crate::custom_error::CustomError;

#[derive(Debug, PartialEq)]
pub struct FileProcessor {
    lecturas: Vec<String>
}

impl FileProcessor {
    pub fn build(ruta_archivo: String) -> Result<FileProcessor, CustomError> { 

        let archivo = match File::open(ruta_archivo) {
            Ok(archivo) => BufReader::new(archivo).lines(),
            Err(_err) => return Err(CustomError::ArchivoNoEncontrado)
        };

        let mut lecturas: Vec<String> = vec![];

        for linea in archivo {
            match linea {
                Ok(linea) => lecturas.push(linea),
                Err(_err) => return Err(CustomError::ErrorEnLecturaDelArchivo)
            }
        }

        Ok(FileProcessor { lecturas })
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_01_ingreso_una_ruta_valida_y_el_processor_se_crea_correctamente() {
        //NOTA: Para correr este test, tengo el directorio 'test', y dentro tengo a data.txt
        let file_processor = FileProcessor::build("./test/data.txt".to_string());
        
        assert!(file_processor.is_ok());
    }

    #[test] 
    fn test_02_ingreso_una_ruta_invalida_y_el_processor_arroja_un_error() {
        //NOTA: Para que este test cumpla su proposito, la ruta NO tiene que existir
        let file_processor = FileProcessor::build("./hola/aca/estan/los/datos/data.txt".to_string());
        
        assert!(file_processor.is_err());
    }
}    