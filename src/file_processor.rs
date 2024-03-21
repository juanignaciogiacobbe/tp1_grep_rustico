use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq)]
pub struct FileProcessor {
    lecturas: Vec<String>
}

impl FileProcessor {
    pub fn build(ruta_archivo: String) -> Result<FileProcessor, Box<dyn Error>> {
        let mut lecturas: Vec<String> = vec![];
        let archivo: File = File::open(ruta_archivo)?;
        let reader: BufReader<File> = BufReader::new(archivo);

        for linea in reader.lines() {
            match linea {
                Ok(linea) => lecturas.push(linea),
                Err(_err) => println!("Error en la lectura del archivo."),
            }
        }

        Ok(FileProcessor {lecturas})
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_01_ingreso_una_ruta_valida_y_el_processor_se_crea_correctamente() {
        let file_processor = FileProcessor::build("hola.txt".to_string());
        
        assert!(file_processor.is_ok());
    }
}    