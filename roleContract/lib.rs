#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod roleContract {
    use ink::storage::Mapping;
    use ink::prelude::string::String;
    use ink::prelude::string::ToString;

    #[ink(storage)]
    pub struct RoleContract {
        roles: Mapping<AccountId, u8>
    }

    impl RoleContract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let caller = Self::env().caller();
            let mut rolesMap = Mapping::default();
            rolesMap.insert(caller, &1);
            Self {
                roles: rolesMap,
            }
        }

        #[ink(message)]
        pub fn get_role(&self) -> Option<String> {
            let caller = Self::env().caller();
            let role = self.roles.get(caller);

            if role == Some(1) {
                return Some("SuperAdmin".to_string());
            } else if role == Some(2) {
                return Some("Admin".to_string());
            } else if role == Some(3) {
                return Some("Moderator".to_string());
            }
            return Some("User".to_string());
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[ink::test]
        fn default_works() {
            let roleContract = RoleContract::default();
            assert_eq!(roleContract.get(), false);
        }

        #[ink::test]
        fn it_works() {
            let mut roleContract = RoleContract::new(false);
            assert_eq!(roleContract.get(), false);
            roleContract.flip();
            assert_eq!(roleContract.get(), true);
        }
    }


    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::*;

        use ink_e2e::build_message;

        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = RoleContractRef::default();

            let contract_account_id = client
                .instantiate("roleContract", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<RoleContractRef>(contract_account_id.clone())
                .call(|roleContract| roleContract.get());
            let get_result = client.call_dry_run(&ink_e2e::alice(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            Ok(())
        }

        #[ink_e2e::test]
        async fn it_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let constructor = RoleContractRef::new(false);
            let contract_account_id = client
                .instantiate("roleContract", &ink_e2e::bob(), constructor, 0, None)
                .await
                .expect("instantiate failed")
                .account_id;

            let get = build_message::<RoleContractRef>(contract_account_id.clone())
                .call(|roleContract| roleContract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), false));

            let flip = build_message::<RoleContractRef>(contract_account_id.clone())
                .call(|roleContract| roleContract.flip());
            let _flip_result = client
                .call(&ink_e2e::bob(), flip, 0, None)
                .await
                .expect("flip failed");

            let get = build_message::<RoleContractRef>(contract_account_id.clone())
                .call(|roleContract| roleContract.get());
            let get_result = client.call_dry_run(&ink_e2e::bob(), &get, 0, None).await;
            assert!(matches!(get_result.return_value(), true));

            Ok(())
        }
    }
}
