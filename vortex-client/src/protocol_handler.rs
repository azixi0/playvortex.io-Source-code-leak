// -- leaked by @azixi0 on github
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegistryValue {
    pub key: String,
    pub name: String,
    pub value: String,
}

pub fn registration_values(executable: &str) -> Vec<RegistryValue> {
    let root = r"Software\Classes\vortex";
    vec![
        RegistryValue { key: root.into(), name: String::new(), value: "URL:Vortex Protocol".into() },
        RegistryValue { key: root.into(), name: "URL Protocol".into(), value: String::new() },
        RegistryValue { key: format!(r"{root}\DefaultIcon"), name: String::new(), value: format!("\"{executable}\",0") },
        RegistryValue {
            key: format!(r"{root}\shell\open\command"),
            name: String::new(),
            value: format!("\"{executable}\" \"%1\""),
        },
    ]
}

