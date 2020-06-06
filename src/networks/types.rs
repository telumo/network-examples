pub enum Protocol {
    Udp,
    Tcp,
}

pub enum Role {
    Client,
    Server,
}

pub struct Argument {
    pub protocol: Protocol,
    pub role: Role,
    pub address: String, // TODO: #3 なぜstr型だとだめ？？
}
