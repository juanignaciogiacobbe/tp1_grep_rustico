use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};

#[derive(Debug, PartialEq)]
pub struct FileProcessor {
    //lecturas: Vec<String>,
    ruta_archivo: String,
}

impl FileProcessor {
    pub fn build(ruta_archivo: String) -> Result<FileProcessor, Box<dyn Error>> {
        //let path = find_absolute_path(ruta_archivo);

        //let mut lecturas: Vec<String> = vec![];
        //let archivo: File = File::open(ruta_archivo)?;
        //let reader: BufReader<File> = BufReader::new(archivo);

        //for linea in reader.lines() {
            //match linea {
              //  Ok(linea) => lecturas.push(linea),
          //      Err(_err) => println!("Error en la lectura del archivo."),
            //}
        //}
        
        Ok(FileProcessor {ruta_archivo})
    }

    pub fn read_file(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut lecturas: Vec<String> = vec![];
        let archivo: File = File::open(self.ruta_archivo.clone())?;
        let reader: BufReader<File> = BufReader::new(archivo);

        for linea in reader.lines() {
            match linea {
                Ok(linea) => lecturas.push(linea),
                Err(_err) => println!("Error en la lectura del archivo."),
            }
        }
        
        Ok(lecturas)
    }

    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_01_ingreso_una_ruta_valida_y_el_processor_se_crea_correctamente() {
        //NOTA: Para correr este test, tengo el directorio 'data', y dentro tengo a data.txt
        let file_processor = FileProcessor::build("./data/data.txt".to_string());
        
        assert!(file_processor.is_ok());
    }
}    