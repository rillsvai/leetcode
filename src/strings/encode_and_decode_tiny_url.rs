pub struct Codec {}

impl Codec {
    pub fn new() -> Self {
        Self {}
    }

    pub fn encode(&self, long_url: String) -> String {
        long_url
    }

    pub fn decode(&self, short_url: String) -> String {
        short_url
    }
}
