struct Codec<'a> {
    url: &'a str,
}

impl Codec<'_> {
    fn new() -> Self {
        Codec { url: "" }
    }

    fn encode(&self, longURL: String) -> String {
        longURL
    }

    fn decode(&self, shortURL: String) -> String {
        shortURL
    }
}
