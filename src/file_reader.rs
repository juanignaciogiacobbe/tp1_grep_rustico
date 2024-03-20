use std::fs;
use std::error::Error;

#[derive(Debug, PartialEq)]
pub struct FileReader {
    ruta_archivo: String
}

impl FileReader {
    pub fn build(ruta_archivo: String) -> Result<FileReader, &'static str> {
        Ok(FileReader {ruta_archivo})

    }

    pub fn run(ruta_archivo: String) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(ruta_archivo)?;

        println!("{}", contents);

        Ok(())
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_01_no_le_ingreso_una_ruta_al_file_reader_y_arroja_error() {
        let file_reader = FileReader::build("data.txt".to_string());
    
        assert_eq!(file_reader, Ok(FileReader{ruta_archivo: "data.txt".to_string()}));

    }


}    