use super::types::*;
use clap::{App, AppSettings, Arg};
use regex::Regex;

pub fn get_arguments() -> Argument {
    let matches = build_cli().get_matches();

    // protocol
    let protocol = matches.value_of("protocol").unwrap();
    let protocol: Protocol = match protocol {
        "udp" => Protocol::Udp,
        "tcp" => Protocol::Tcp,
        _ => Protocol::Udp,
    };

    // role
    let role = matches.value_of("role").unwrap();
    let role: Role = match role {
        "client" => Role::Client,
        "server" => Role::Server,
        _ => Role::Server,
    };

    // address
    let address = matches.value_of("address").unwrap().into();

    Argument {
        protocol,
        role,
        address,
    }
}

fn build_cli() -> App<'static, 'static> {
    App::new("myapp")
        .version("1.0")
        .author("telumo <drumscohika@gmail.com>")
        .about("Sample Network Problems")
        .setting(AppSettings::DeriveDisplayOrder)
        .arg(
            Arg::from_usage("-p --protocol <PROTOCOL> 'protocol'")
                .possible_values(&["Udp", "Tcp"])
                .default_value("Udp"),
        )
        .arg(
            Arg::from_usage("-r --role <ROLE> 'role : server/client'")
                .possible_values(&["server", "client"])
                .default_value("server"),
        )
        .arg(Arg::from_usage("-a --address <ADDRESS> 'address'").validator(valid_address_format))
}

// TODO: #2 テストが通ったり通らなかったりする
fn is_address_format(v: &str) -> bool {
    Regex::new(r"\d{1,3}.\d{1,3}.\d{1,3}.\d{1,3}:\d")
        .unwrap()
        .is_match(v)
}

fn valid_address_format(v: String) -> Result<(), String> {
    if !is_address_format(&v) {
        return Err(String::from("invaild address format!"));
    }
    Ok(())
}

mod test {

    #[test]
    fn test_is_address_format_1() {
        let test = "1.1.1.1:0000";
        let result = super::is_address_format(test);
        assert!(result)
    }

    #[test]
    fn test_is_address_format_2() {
        let test = "sample";
        let result = super::is_address_format(test);
        assert!(!result)
    }

    #[test]
    fn test_is_address_format_3() {
        let test = "127.0.0.1:33333";
        let result = super::is_address_format(test);
        assert!(result)
    }
}
