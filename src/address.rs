pub struct Address{
    pub ip:     (u8, u8, u8, u8),
    pub port:   u16
}


impl ToString for Address{
    fn to_string(&self) -> String {
        let endereco_string = format!("{}.{}.{}.{}", self.ip.0, self.ip.1, self.ip.2, self.ip.3);
        let port_string = &self.port.to_string();

        format!("{endereco_string}:{port_string}")
    }
}
