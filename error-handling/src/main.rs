use std::fs::File;
use std::io::ErrorKind;
use std::io;
use std::fs;
use std::io::Read;
use std::error::Error;

fn main() {
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file)=> file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc)=>fc,
                Err(e)=>panic!("Problem creating the file: {:?}",e),
            },
            other_error=> {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };

    let _p = File::open("test.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("test.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}",error);
        }
    });

    // let _d = File::open("test2.txt").unwrap();
    // let _d = File::open("test2.txt").expect("Failed to open test2");



    // fn read_username_from_file() -> Result<String, io::Error> {
    //     let f = File::open("test3.txt");

    //     let mut f = match f {
    //         Ok(file) => file,
    //         Err(e) => return Err(e),
    //     };

    //     let mut s = String::new();
    //     match f.read_to_string(&mut s) {
    //         Ok(_) => Ok(s),
    //         Err(e) => Err(e),
    //     }
    // }

    fn read_username_from_file() -> Result<String, io::Error> {
        // let mut f = File::open("test3.txt")?;
        // let mut s = String::new();
        // f.read_to_string(&mut s)?;
        // File::open("test3.txt")?.read_to_string(&mut s)?;
        fs::read_to_string("test3.txt")
        // Ok(s)
    }

    fn read_username_from_file2() -> Result<(), Box<dyn Error>> {
        
        let f = File::open("test4.txt")?;


        Ok(())
    }

    

    println!("{:?}",read_username_from_file2());
    println!("{:?}",read_username_from_file())

}
