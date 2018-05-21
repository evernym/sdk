use utils::libindy::{ wallet };
use error::wallet::WalletError;
use settings::{CONFIG_WALLET_NAME};

pub fn create(source_id: String) -> Result<u32, WalletError> {
    let wallet_name = ::settings::get_config_value(CONFIG_WALLET_NAME)
        .map_err(|err| WalletError::CommonError(err))?;

    let wallet_h = wallet::init_wallet(&wallet_name)
        .map_err(|err| WalletError::CommonError(err))? as u32;

    Ok(wallet_h)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_wallet_configs(wallet_name: &str) {
        ::settings::set_defaults();
        ::settings::set_config_value(CONFIG_WALLET_NAME, wallet_name);
        ::settings::set_config_value(::settings::CONFIG_WALLET_KEY, ::settings::DEFAULT_DEFAULT);
    }

    #[test]
    fn test_create_wallet_success() {
        let wallet_name = "create_wallet_success";
        set_wallet_configs(wallet_name);

        assert!(create("source_id 123".to_string()).unwrap() > 0);

        ::utils::libindy::wallet::delete_wallet(wallet_name).unwrap();
    }
}