pub struct Error<'a> {
    pub name: &'a str,
    pub message: &'a str
}

impl<'a> Error<'a> {
    pub fn new(message: &'a str) -> Self {
        let error = Error {
            name: "Parsing Error",
            message: message
        };

        panic!("{}: {}", error.name, error.message);
    }
}