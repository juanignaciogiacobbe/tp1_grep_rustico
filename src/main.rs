use std::env;

use tp1_grep_rustico::{
  command_processor::CommandProcessor, 
  custom_error::CustomError
};

fn print_error(error: CustomError) -> () {
  println!("{}", error);
}

fn run(args: Vec<String>) -> Result<(), CustomError> {
  let command_processor = match CommandProcessor::build(args) {
      Ok(command_processor) => command_processor,
      Err(e) => return Err(e)
  };

  command_processor.run();

  Ok(())
}

fn main() {
    //segun la interfaz propuesta en la consigna, voy a considerar
    //que primero se tiene que ingresar la regular expression, y luego
    //el path al archivo
    let args: Vec<String> = env::args().collect();
    
    let _ = run(args).map_err(|e: CustomError| print_error(e));

}