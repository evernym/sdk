#[cfg(test)]
pub mod tests {
    extern crate rand;

    use utils::constants;
    use utils::libindy::wallet;
    use utils::libindy::pool;
    use settings;
    use object_cache::ObjectCache;

    static mut INSTITUTION_CONFIG: u32 = 0;
    static mut CONSUMER_CONFIG: u32 = 0;

    lazy_static! {
        static ref CONFIG_STRING: ObjectCache<String> = Default::default();
    }

    pub const TRUSTEE: &str = "000000000000000000000000Trustee1";

    /* INSTITUTION/ENTERPRISE settings */
    pub const AGENCY_ENDPOINT: &'static str = "https://enym-eagency.pdev.evernym.com";
    pub const AGENCY_DID: &'static str = "YRuVCckY6vfZfX9kcQZe3u";
    pub const AGENCY_VERKEY: &'static str = "J8Yct6FwmarXjrE2khZesUXRVVSVczSoa9sFaGe6AD2v";

    /**
     * CONSUMER/USER settings
     * generated by running provision script with: --enterprise-seed=00000000000000000000001DIRECTION --agent-seed=00000000000000000000002DIRECTION
     */

    pub const C_AGENCY_ENDPOINT: &'static str = "https://cagency.pdev.evernym.com";
    pub const C_AGENCY_DID: &'static str = "dTLdJqRZLwMuWSogcKfBT";
    pub const C_AGENCY_VERKEY: &'static str = "LsPQTDHi294TexkFmZK9Q9vW4YGtQRuLV8wuyZi94yH";

    pub fn setup_ledger_env(wallet_name: &str) {
        match pool::get_pool_handle() {
            Ok(x) => pool::close().unwrap(),
            Err(x) => (),
        };

        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_WALLET_KEY,settings::TEST_WALLET_KEY);
        settings::set_config_value(settings::CONFIG_WALLET_NAME, wallet_name);
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        pool::open_sandbox_pool();

        wallet::init_wallet(wallet_name).unwrap();
        ::utils::libindy::anoncreds::libindy_prover_create_master_secret(settings::DEFAULT_LINK_SECRET_ALIAS).unwrap();
        let (my_did, _) = ::utils::libindy::signus::SignusUtils::create_and_store_my_did(wallet::get_wallet_handle(), Some(TRUSTEE)).unwrap();
        let did = settings::set_config_value(settings::CONFIG_INSTITUTION_DID, &my_did);
        ::utils::libindy::payments::tests::token_setup(None, None);
    }

    pub fn cleanup_dev_env(wallet_name: &str) {
        //settings::set_defaults();
        wallet::close_wallet().unwrap();
        wallet::delete_wallet(wallet_name).unwrap();
        pool::close().unwrap();
        pool::delete(::utils::constants::POOL).unwrap();
    }

    pub fn set_institution() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(INSTITUTION_CONFIG, |t| {
                settings::process_config_string(&t)
            }).unwrap();
        }
    }

    pub fn set_consumer() {
        settings::clear_config();
        unsafe {
            CONFIG_STRING.get(CONSUMER_CONFIG, |t| {
                settings::process_config_string(&t)
            }).unwrap();
        }
    }

    pub fn setup_local_env(wallet_name: &str) {
        settings::clear_config();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE, "false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, settings::TEST_WALLET_KEY);

        let config = ::messages::agent_utils::connect_register_provision(AGENCY_ENDPOINT,
                                                                         AGENCY_DID,
                                                                         AGENCY_VERKEY,
                                                                         Some(wallet_name.to_string()),
                                                                         None,
                                                                         Some(TRUSTEE.to_string()),
                                                                         settings::TEST_WALLET_KEY,
                                                                         Some("institution".to_string()),
                                                                         Some("http://www.logo.com".to_string()),
                                                                         Some(constants::GENESIS_PATH.to_string())).unwrap();

        unsafe {
            INSTITUTION_CONFIG = CONFIG_STRING.add(config).unwrap();
        }

        ::api::vcx::vcx_shutdown(false);

        let config = ::messages::agent_utils::connect_register_provision(C_AGENCY_ENDPOINT,
                                                                         C_AGENCY_DID,
                                                                         C_AGENCY_VERKEY,
                                                                         Some(wallet_name.to_string()),
                                                                         None,
                                                                         None,
                                                                         settings::TEST_WALLET_KEY,
                                                                         Some("consumer".to_string()),
                                                                         Some("http://www.logo.com".to_string()),
                                                                         Some(constants::GENESIS_PATH.to_string())).unwrap();

        unsafe {
            CONSUMER_CONFIG = CONFIG_STRING.add(config).unwrap();
        }

        pool::open_sandbox_pool();

        wallet::open_wallet(wallet_name).unwrap();
        set_institution();

        ::utils::libindy::payments::tests::token_setup(None, None);

    }

    #[cfg(feature = "pool_tests")]
    #[test]
    fn test_local_env() {
        let wallet_name = "test_local_env";
        setup_local_env(wallet_name);
        ::utils::libindy::anoncreds::tests::create_and_store_credential();
        cleanup_dev_env(wallet_name);
    }

    pub fn setup_wallet_env(test_name: &str) -> Result<i32, String> {
        use utils::libindy::wallet::init_wallet;
        use utils::libindy::signus::SignusUtils;
        static KEY: &str = "pass";
        settings::set_defaults();
        settings::set_config_value(settings::CONFIG_ENABLE_TEST_MODE,"false");
        settings::set_config_value(settings::CONFIG_WALLET_KEY, KEY);
        init_wallet(test_name).or(Err("Unable to init_wallet in tests".to_string()))
    }

    pub fn cleanup_wallet_env(test_name: &str) -> Result<(), String> {
        use utils::libindy::wallet::delete_wallet;
        delete_wallet(test_name).or(Err(format!("Unable to delet wallet: {}", test_name)))
    }
}
