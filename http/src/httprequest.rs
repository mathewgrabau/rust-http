#[derive(Debug, PartialEq)]
pub enum Method {
    Get, 
    Post,
    Uninitialized
}

impl From<&str> for Method {
    // Allows extracting the method string from the HTTP request line,
    // and gets the form the application can make use of.
    fn from(s: &str) -> Method {
        match s {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => Method::Uninitialized
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_method_into() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::Get);
    }
}
