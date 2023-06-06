
#[cfg(test)]
mod alpine_user_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, DepsMut, Addr };

    use crate::msg::{
        InstantiateMsg,
        ExecuteMsg,
        QueryMsg,
        MultiUserResponse,
        AlpineUserResponse
    };
    use crate::{
        ContractError,
        state::{
            AlpineContract,
            AlpineUser
        },
        msg::UsernameAvailableResponse
    };

    // A basic utility function to setup the contract so we don't have to do this every time
    fn setup_contract(deps: DepsMut<'_>) -> AlpineContract<'static> {
        let contract = AlpineContract::default();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        contract
    }

    // Attempt to create a user with an invalid wallet address. Should error out
    #[test]
    fn create_user_invalid_addr() {
        let deps = mock_dependencies();
        let test_address = Addr::unchecked("");
        let create_test_user = AlpineUser::new(
            deps.as_ref(),
            test_address.clone(),
            None
        ).unwrap_err();

        assert_eq!(create_test_user, ContractError::InvalidWalletAddress { address: test_address.to_string() });
    }

    // Attempt to create a new user without registering them. This uses an empty username. Should be successful
    #[test]
    fn create_user_anonymous_success() {
        let deps = mock_dependencies();
        let test_addr = String::from("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh");
        
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(test_addr.clone()),
            None
        ).unwrap();

        assert_eq!(test_user.address, test_addr);
        assert_eq!(test_user.username, "");
    }

    // Attempt to create a user with a (relatively complex) valid username. Should result in success
    #[test]
    fn create_user_with_name_success() {
        let deps = mock_dependencies();
        let test_addr = String::from("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh");
        let test_username = String::from("this-Is-A-Valid_Username1234");
        
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(test_addr.clone()),
            Some(test_username.clone())
        ).unwrap();

        assert_eq!(test_user.address, test_addr);
        assert_eq!(test_user.username, test_username);
    }

    // Register a user with an empty username. Should error out
    #[test]
    fn save_username_empty_username() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: String::from("")
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::EmptyUsername {});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_too_long() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("ThisUsernameIsTooLongAndWillCauseAnError");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("must be shorter than 33 characters")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_backslash() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("\\backslashesareforbidden/");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_spaces() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("spaces are not supported");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_encoded() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("Nice%20Try");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Attempt to register a user with an invalid username. Should error out.
    #[test]
    fn save_username_unsupported_characters_encoded_greekletter() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let username = String::from("Σ");

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: username.clone()
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::InvalidUsername { username, reason: String::from("only alphanumeric, underscores, and dashes are allowed")});
    }

    // Check if an unregistered username is available. Should return true.
    #[test]
    fn is_username_available_true() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());
        let test_username = String::from("alpine_user_1");

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, test_username.clone(), &test_user).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("alpine_user_2") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, true);
    }

    // Check if an registered username is available. Should return false.
    #[test]
    fn is_username_available_false() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());
        let test_username = String::from("alpine_user_1");

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, test_username.clone(), &test_user).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("alpine_user_1") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, false);
    }

    // Check if a username is available. Technically the username is unregistered, but the only difference
    // is casing. Should return false.
    #[test]
    fn is_username_available_false_case_insensitive() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());
        let test_username = String::from("alpine_user_1");

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, test_username.clone(), &test_user).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("ALPINE_USER_1") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, false);
    }

    // Attempt to register a user with a taken username. Should error out.
    #[test]
    fn save_username_unavailable() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_1"), &test_user).unwrap();

        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            None
        ).unwrap();

        let username = String::from("alpine_user_1");
        let msg = ExecuteMsg::RegisterUser {
            user: new_user.clone(),
            username: username.clone()
        };
        let info = mock_info(new_user.address.as_str(), &[]);
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(_res, ContractError::UsernameNotAvailable { username });
    }

    // Attempt to save a user with an unregistered username. Should be successful
    #[test]
    fn save_username_success() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: String::from("alpine_user_1")
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(_res.attributes[0].value, "alpine_user_1");
    }
    
    // Obtain a list of all saved usernames
    #[test]
    fn get_usernames(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_1"), &test_user).unwrap();

        // Save User Two
        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_2"), &new_user).unwrap();

        // Save User Three
        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_3"), &new_user).unwrap();

        let msg = QueryMsg::GetAllUsers { };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let users: MultiUserResponse  = from_binary(&res).unwrap();
        assert_eq!(users.users.len(), 3)
    }

    // Attempt to register a new user whose username prior to this was empty
    #[test]
    fn change_username_from_anonymous(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let mut test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        let info = mock_info(test_user.address.as_str(), &[]);

        let msg = ExecuteMsg::RegisterUser {
            user: test_user.clone(),
            username: String::from("Anonymous")
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        test_user.username = String::from("Anonymous");
        let msg = ExecuteMsg::RegisterUser {
            user: test_user,
            username: String::from("aline_user_1")
        };
        let res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap_err();
        assert_eq!(res.to_string(), "Address Already Registered")
    }

    // Try to grab a user with a bad address. Technically this works because we're not on-chain.
    #[test]
    fn get_user_by_bad_address(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        contract.addresses.save(&mut deps.storage, Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"), &test_user).unwrap();

        // Junk user
        let junk_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("somejunk"),
            Some(String::from(""))
        ).unwrap();

        let msg = QueryMsg::GetUserByAddr{ address: junk_user.address.clone() };
        let res: AlpineUserResponse = from_binary(&contract.query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
        assert_eq!(res.user, junk_user);
    }

    // Try to grab a user with a good address. Results successful
    #[test]
    fn get_user_by_good_address(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        contract.addresses.save(&mut deps.storage, Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"), &test_user).unwrap();

        let msg = QueryMsg::GetUserByAddr{ address: Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, test_user);
    }

    // Try to grab a user with a nonexistent username. Results successful, user object is empty
    #[test]
    fn get_user_by_nonexistent_username(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        // Create an empty user for the assert
        let empty_user = AlpineUser::empty();

        let msg = QueryMsg::GetUserByName{ username: String::from("alpine_user_1") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, empty_user);
    }

    // Try to grab a user with a valid username. Results successful
    #[test]
    fn get_user_by_good_username(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        // Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_1"), &test_user).unwrap();

        let msg = QueryMsg::GetUserByName{ username: String::from("alpine_user_1") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, test_user);
    }
}

// A set of tests for donations
#[cfg(test)]
mod donation_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, DepsMut, coins, MessageInfo, Addr};

    use crate::msg::{
        InstantiateMsg,
        ExecuteMsg,
        QueryMsg,
        MultiDonationResponse,
    };
    use crate::{
        ContractError,
        state::{
            AlpineContract,
            AlpineUser
        },
        traits::DonationQuery,
    };

    // A utility function to set up a contract
    fn setup_contract(deps: DepsMut<'_>) -> AlpineContract<'static> {
        let contract = AlpineContract::default();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        contract
    }

    // Validate that instantiation is succesful
    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let donation_count = contract.get_donation_count(deps.as_ref()).unwrap();
        assert_eq!(0, donation_count.count);
    }

    // Attempt to send a donation to a user which doesn't exist. Should error out
    #[test]
    fn send_donation_to_nonexistent_recipient() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let invalid_user: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("nonexistent_user"))
        ).unwrap();

        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: invalid_user.username.clone()
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::UserNotFound { user: invalid_user.username });
    }

    // Attempt to send a donation without any currency attached
    #[test]
    fn send_no_dono() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info =  MessageInfo {
            sender: deps.as_mut().api.addr_validate(alpine_user_a.address.as_str()).unwrap(),
            funds: Vec::new()
        };

        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: alpine_user_b.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::NoDonation{ });
    }

    // Attempt to send a donation which technically has currency attached, but the amount is 0. Should error out
    #[test]
    fn send_no_dono_amount() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(0, "earth"));
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: alpine_user_b.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::NoDonation{ });
    }

    // Attempt to send a donation with a message that's too long. Should error out
    #[test]
    fn send_too_long_message() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("This message is really long. In fact, it's actually too long for you to use it in our app.\
                    We shouldn't allow users to send a message that's this long. There's no reason to send a message that's this long. If I was a \
                    content creator and I was constantly having people send me giant messages like this for like $3, I would not only hate this app, \
                    but I would also begin to dislike my fans.");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message, 
            sender: alpine_user_a.username,
            recipient: alpine_user_b.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg)
            .unwrap_err();
        assert_eq!(_res, ContractError::DonationMessageTooLong {  });
    }

    // Obtain a list of multiple sent donations and validate length. Should return success.
    #[test]
    fn get_multiple_sent_donations() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            Some(String::from("USER_D"))
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_c.username.clone(), &alpine_user_c).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_d.username.clone(), &alpine_user_d).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_b.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_c.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_d.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let msg = QueryMsg::GetSentDonations { sender: alpine_user_a.username.clone() };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let sent_donations: MultiDonationResponse = from_binary(&res).unwrap();
        assert_eq!(3, sent_donations.donations.len());
    }

    // Obtain a list of multiple sent donations and validate that they're sorted in the correct order. Should return success.
    #[test]
    fn get_multiple_sent_donations_sorted(){
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            Some(String::from("USER_D"))
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_c.username.clone(), &alpine_user_c).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_d.username.clone(), &alpine_user_d).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone() + "1", 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_b.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone() + "2", 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_c.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone() + "3", 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_d.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let msg = QueryMsg::GetSentDonations { sender: alpine_user_a.username.clone() };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let sent_donations: MultiDonationResponse = from_binary(&res).unwrap();
        assert_eq!(donation_message.clone() + "1", sent_donations.donations[0].1.message);
        assert_eq!(donation_message.clone() + "2", sent_donations.donations[1].1.message);
        assert_eq!(donation_message.clone() + "3", sent_donations.donations[2].1.message);

    }

    // Obtain a list of multiple received donations and validate the length. Should return success.
    #[test]
    fn get_multiple_received_donations() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            Some(String::from("USER_D"))
        ).unwrap();
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_c.username.clone(), &alpine_user_c).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_d.username.clone(), &alpine_user_d).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_b.username.clone(),
            recipient: alpine_user_a.username.clone()
        };
        let info = mock_info(alpine_user_b.address.as_str(), &coins(1000, "earth"));
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_c.username,
            recipient: alpine_user_a.username.clone()
        };
        let info = mock_info(alpine_user_c.address.as_str(), &coins(1000, "earth"));
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(), 
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_b.username.clone()
        };
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let msg = QueryMsg::GetReceivedDonations { recipient: alpine_user_a.username.clone() };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let received_donations: MultiDonationResponse = from_binary(&res).unwrap();
        assert_eq!(2, received_donations.donations.len());
    }

    // Obtain a list of multiple received donations and validate that they're sorted in the correct order. Should return success.
    #[test]
    fn get_multiple_received_donations_sorted() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            Some(String::from("USER_D"))
        ).unwrap();
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_c.username.clone(), &alpine_user_c).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_d.username.clone(), &alpine_user_d).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone() + "1", 
            sender: alpine_user_b.username.clone(),
            recipient: alpine_user_a.username.clone()
        };
        let info = mock_info(alpine_user_b.address.as_str(), &coins(1000, "earth"));
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone() + "2", 
            sender: alpine_user_c.username.clone(),
            recipient: alpine_user_a.username.clone()
        };
        let info = mock_info(alpine_user_c.address.as_str(), &coins(1000, "earth"));
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone() + "3", 
            sender: alpine_user_c.username.clone(),
            recipient: alpine_user_a.username.clone()
        };
        let info = mock_info(alpine_user_c.address.as_str(), &coins(1000, "earth"));
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        
        let msg = QueryMsg::GetReceivedDonations { recipient: alpine_user_a.username.clone() };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let received_donations: MultiDonationResponse = from_binary(&res).unwrap();
        assert_eq!(donation_message.clone() + "1", received_donations.donations[0].1.message);
        assert_eq!(donation_message.clone() + "2", received_donations.donations[1].1.message);
        assert_eq!(donation_message.clone() + "3", received_donations.donations[2].1.message);
    }
}

// Define a set of integration tests that use our entry points instead of internal calls
#[cfg(test)]
mod integration_tests {
    use cosmwasm_std::{testing::{mock_dependencies, mock_info, mock_env}, Addr, from_binary};

    use crate::{InstantiateMsg, entry::{instantiate, migrate, query, execute}, MigrateMsg, state::AlpineUser, ExecuteMsg, QueryMsg, msg::MultiUserResponse};

    // Validate that instantiation works from the client's perspective
    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

    // Validate that migration works from the client's perspective
    #[test]
    fn proper_migration() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = MigrateMsg {};
        let res = migrate(deps.as_mut(), mock_env(), msg).unwrap();
        assert_eq!(0, res.messages.len())
    }

    // Validate that execution works from the client's perspective
    #[test]
    fn successful_execute() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1tpsscvhaddf36gjvnyjhtwsyempptupypngxzs"),
            Some(String::from(""))
        ).unwrap();
        let msg = ExecuteMsg::RegisterUser {
            user: alpine_user_a.clone(),
            username: String::from("a_tester")
        };
        let info = mock_info(alpine_user_a.address.as_str(), &[]);
        
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    }

    // Validate that queries work from the client's perspective
    #[test]
    fn successful_query() {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("osmo1tpsscvhaddf36gjvnyjhtwsyempptupypngxzs"),
            Some(String::from(""))
        ).unwrap();
        let msg = ExecuteMsg::RegisterUser {
            user: alpine_user_a.clone(),
            username: String::from("a_tester")
        };
        let info = mock_info(alpine_user_a.address.as_str(), &[]);
        execute(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let msg = QueryMsg::GetAllUsers { };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();
        let users: MultiUserResponse  = from_binary(&res).unwrap();
        assert_eq!(users.users.len(), 1)
    }
}