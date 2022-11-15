#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Method {
    GET, POST, PATCH, DELETE, PUT, UNKNOW
}
impl Method {
    pub fn is_get    (self) -> bool { self == Method::GET    }
    pub fn is_post   (self) -> bool { self == Method::POST   }
    pub fn is_patch  (self) -> bool { self == Method::PATCH  }
    pub fn is_delete (self) -> bool { self == Method::DELETE }
    pub fn is_put    (self) -> bool { self == Method::PUT    }
    pub fn is_unknow (self) -> bool { self == Method::UNKNOW }
}