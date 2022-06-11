#[derive(Clone)]
#[allow(non_camel_case_types)]
pub struct Perl_original {
    support_level: SupportLevel,
    data: DataHolder,
    code: String,
    main_file_path: String,
}

impl ReplLikeInterpreter for Perl_original {}

impl Interpreter for Perl_original {
    fn new_with_level(data: DataHolder, support_level: SupportLevel) -> Box<Perl_original> {
        //create a subfolder in the cache folder
        let lwd = data.work_dir.clone() + "/perl_original";
        let mut builder = DirBuilder::new();
        builder.recursive(true);
        builder
            .create(&lwd)
            .expect("Could not create directory for perl_original");

        //pre-create string pointing to main file's and binary's path
        let mfp = lwd.clone() + "/main.pl";
        Box::new(Perl_original {
            data,
            support_level,
            code: String::new(),
            main_file_path: mfp,
        })
    }

    fn get_supported_languages() -> Vec<String> {
        vec![
            String::from("Perl"), // in 1st position of vector, used for info only
            String::from("perl"),
            String::from("pm"),
            String::from("pl"), //should not be necessary, but just in case
                                       // another similar name (like python and python3)?
        ]
    }

    fn get_name() -> String {
        // get your interpreter name
        String::from("Perl_original")
    }

    fn get_current_level(&self) -> SupportLevel {
        self.support_level
    }

    fn set_current_level(&mut self, level: SupportLevel) {
        self.support_level = level;
    }

    fn get_data(&self) -> DataHolder {
        self.data.clone()
    }

    fn get_max_support_level() -> SupportLevel {
        //define the max level support of the interpreter (see readme for definitions)
        SupportLevel::Bloc
    }

    fn default_for_filetype() -> bool {
        true
    }

    fn fetch_code(&mut self) -> Result<(), SniprunError> {
        if !self
            .data
            .current_bloc
            .replace(&[' ', '\t', '\n', '\r'][..], "")
            .is_empty()
            && self.support_level >= SupportLevel::Bloc
        {
            // if bloc is not pseudo empty and has Bloc current support level,
            // add fetched code to self
            self.code = self.data.current_bloc.clone();

        // if there is only data on current line / or Line is the max support level
        } else if !self.data.current_line.replace(" ", "").is_empty()
            && self.support_level >= SupportLevel::Line
        {
            self.code = self.data.current_line.clone();
        } else {
            // no code was retrieved
            self.code = String::from("");
        }

        // now self.code contains the line or bloc of code wanted :-)
        Ok(())
    }

    fn add_boilerplate(&mut self) -> Result<(), SniprunError> {
        // an example following Rust's syntax
        Ok(())
    }

    fn build(&mut self) -> Result<(), SniprunError> {
        let mut _file =
            File::create(&self.main_file_path).expect("Failed to create file for perl-original");

        write(&self.main_file_path, &self.code).expect("Unable to write to file for perl-original");
        Ok(())
    }

    fn execute(&mut self) -> Result<String, SniprunError> {
        //run th binary and get the std output (or stderr)
        let output = Command::new("perl")
            .arg(&self.main_file_path)
            .args(&self.get_data().cli_args)
            .output()
            .expect("Unable to start process");
        info!("yay from perl interpreter");
        if output.status.success() {
            //return stdout
            return Ok(String::from_utf8(output.stdout).unwrap());
        } else {
            // return stderr
            return Err(SniprunError::RuntimeError(
                String::from_utf8(output.stderr).unwrap(),
            ));
        }
    }
}

// You can add tests if you want to
#[cfg(test)]
mod test_perl_original {
    use super::*;

    #[test]
    fn simple_print() {
        let mut data = DataHolder::new();

        //inspired from Rust syntax
        data.current_bloc = String::from("print \"Hello,Perl!\"");
        let mut interpreter = Perl_original::new(data);
        let res = interpreter.run();

        // -> should panic if not an Ok()
        // let string_result = res.unwrap();
        let string_result = res.unwrap();

        // -> compare result with predicted
        // assert_eq!(string_result, "HW, 1+1 = 2\n");
        assert_eq!(string_result, "Hello,Perl!");

    }
}
