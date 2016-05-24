use std::fs::File;
use std::io::prelude::*;
use std::sync::Mutex;
use std::fs::OpenOptions;

struct AppendLog {
    file: Mutex<File>,
}



impl AppendLog {
    fn new(file: &str) -> Self {
        let mut options = OpenOptions::new();
        match options.create(true).append(true).read(true).open(file) {
            Ok(f) => AppendLog { file: Mutex::new(f) },
            Err(e) => panic!(e),
        }
    }

    fn write(&self, string: &str) {
        let mut file = self.file.lock().unwrap();
        let length = format!("{:01$x}", string.as_bytes().len(), 6);

        file.write_all((length + string).as_bytes());
        file.sync_data();

    }

    fn read(&self) {
        // reads the file from the start
    }
}


fn main() {
    let al = AppendLog::new("test123.123");
    al.write("JJAA₯");
}
