use std::fs::File;
use std::io::*;

//Handling errors

fn main() {
    // Creating file called 'lines.txt'
    let path = "lines.txt";
    // Defining the output where the file is created
    let output = File::create(path);
    // Handling errors with creating the file
    let mut output = match output {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem creating file : {:?}", error);
        }
    };
    // Write to the file and define the panic error message to be expected with problems.
    write!(output, "Just some\nRandom words").expect("Failed to write to file");

    // Open the file and if everything is ok, unwrap and return/read the file
    let input = File::open(path).unwrap();
    let buffered = BufReader::new(input);
    for line in buffered.lines() {
        println!("{}", line.unwrap());
    }

    // Handling specific errors
    let output2 = File::create("rand.txt");
    let output2 = match output2 {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("rand.txt") {
                Ok(fc) => fc,
                Err(error) => panic!("Can't create file : {:?}", error),
            },
            _other_error => panic!("Problem opening file : {:?}", error),
        },
    };
}
