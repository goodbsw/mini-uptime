pub struct EnvPair<'a> {
    key: &'a str,
    value: &'a str
}

impl<'a> EnvPair<'a> {
    pub fn parse(s: &'a str) -> Vec<EnvPair<'a>> {
        let mut result = Vec::new();
        
        for line in s.split("\n") {
            if let Some((k, v)) = line.split_once("=") {
                result.push(
                    EnvPair {
                        key: k.trim(),
                        value: v.trim()
                    }
                )
            }
        }
        result
    }
}

impl<'a> std::fmt::Display for EnvPair<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[KEY] {}  ->  [VALUE] {}", self.key, self.value)
    }
}