use std::io::Error as IoError;
quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: IoError) {
            from()
            source(err)
            display("{:?}", err)
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;