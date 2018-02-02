extern crate toml;

#[derive(Deserialize, Debug)]
struct Tomlfile {
    contents: Contents,
}

#[derive(Deserialize, Debug)]
struct Metadata {
    deb: Deb,
}

#[derive(Deserialize, Debug)]
struct Deb {
    revision: Option<String>,
}


#[derive(Deserialize, Debug)]
struct Package {
    name: String,
    version: Option<String>,
    metadata: Metadata,
}

#[derive(Deserialize, Debug)]
struct Contents {
    package: Package,
    dependencies: Option<toml::Value>,
}
use std::env;
use std::io::prelude::*;
use std::fs::File;

pub fn get_version() -> Option<String> {
    let dir = match  env::var("CARGO_MANIFEST_DIR"){
        Ok(d) => d,
        Err(_) => panic!("Couldnt find file"),
    };
    let filename = "Cargo.toml";
    let p = format!("{}/{}",dir,filename);
    let mut input = String::new();
    File::open(p).and_then(|mut f| {
        f.read_to_string(&mut input)}).unwrap();

    let tomlfile:Contents = toml::from_str(&input).unwrap();
    let revision:String = match tomlfile.package.metadata.deb.revision {
        Some(v) => v,
        None => String::from(""),
    };
    let version:String = match tomlfile.package.version {
        Some(v) => v,
        None => String::from(""),
    };
    Some(format!("{}+{}", version, revision))
}

#[cfg(test)]
pub mod tests {
    use super::*;
    static TOML_STR:&str = r#"
            [package]
            name = "libcxs"
            version = "0.1.543416"
            authors = [ "Mark Hadley <mark.hadley@evernym.com>" ]
            publish = false
            description = "C callable library for libcxs"
            license = ""

            [lib]
            name = "cxs"
            path = "src/lib.rs"
            crate-type = ["staticlib","rlib", "dylib"]

            [lib.foo]
            bar="baz"

            [dependencies]
            env_logger = "0.4.3"
            log = "0.3"

            [package.metadata.deb]
            maintainer = "Mark Hadley<mark.hadley@evernym.com>, Devin Fisher, Ryan Marsh, Doug Wightman"
            section = "admin"
            priority = "optional"
            revision = "469b25d"
            "#;
    static TOML_STR_MISSING_REVISION:&str = r#"
            [package]
            name = "libcxs"

            [lib]
            name = "cxs"

            [package.metadata.deb]
            maintainer = "Mark Hadley<mark.hadley@evernym.com>, Devin Fisher, Ryan Marsh, Doug Wightman"
            section = "admin"
            priority = "optional"
            "#;

    #[test]
    pub fn test_get_version(){
        let tomlfile:Contents = toml::from_str(TOML_STR).unwrap();
        assert_eq!("469b25d", tomlfile.package.metadata.deb.revision.unwrap());
        assert_eq!("0.1.543416", tomlfile.package.version.unwrap());
        let tomlfile_missing:Contents = toml::from_str(TOML_STR_MISSING_REVISION).unwrap();
        assert_eq!(tomlfile_missing.package.metadata.deb.revision.is_none(), true);
        assert_eq!(get_version().is_some(),true);
    }

}