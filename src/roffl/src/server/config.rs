pub struct User<'a> {
    pub name: &'a str,
    pub password_hash: &'a str,
}

pub struct ServerConfig<'a> {
    pub password: &'a str,
    pub users: Vec<User>,
}
