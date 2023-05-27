
#[cfg(test)]
mod alpine_user_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, DepsMut, Addr, StdError };

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

    fn setup_contract(deps: DepsMut<'_>) -> AlpineContract<'static> {
        let contract = AlpineContract::default();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        contract
    }

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

    #[test]
    fn create_user_anonymous_success() {
        let deps = mock_dependencies();
        let test_addr = String::from("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh");
        
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(test_addr.clone()),
            None
        ).unwrap();

        assert_eq!(test_user.address, test_addr);
        assert_eq!(test_user.username, "");
    }

    #[test]
    fn create_user_with_name_success() {
        let deps = mock_dependencies();
        let test_addr = String::from("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh");
        let test_username = String::from("this-Is-A-Valid_Username1234");
        
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked(test_addr.clone()),
            Some(test_username.clone())
        ).unwrap();

        assert_eq!(test_user.address, test_addr);
        assert_eq!(test_user.username, test_username);
    }

    #[test]
    fn save_username_empty_username() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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

    #[test]
    fn save_username_too_long() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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

    #[test]
    fn save_username_unsupported_characters_backslash() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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

    #[test]
    fn save_username_unsupported_characters_spaces() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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

    #[test]
    fn save_username_unsupported_characters_encoded() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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

    #[test]
    fn save_username_unsupported_characters_encoded_greekletter() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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

    #[test]
    fn is_username_available_true() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());
        let test_username = String::from("alpine_user_1");

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, test_username.clone(), &test_user).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("alpine_user_2") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, true);
    }

    #[test]
    fn is_username_available_false() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());
        let test_username = String::from("alpine_user_1");

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, test_username.clone(), &test_user).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("alpine_user_1") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, false);
    }

    #[test]
    fn is_username_available_false_case_insensitive() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());
        let test_username = String::from("alpine_user_1");

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, test_username.clone(), &test_user).unwrap();

        let msg = QueryMsg::IsUsernameAvailable { username: String::from("ALPINE_USER_1") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let username_response: UsernameAvailableResponse = from_binary(&res).unwrap();
        assert_eq!(username_response.is_available, false);
    }

    #[test]
    fn save_username_unavailable() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_1"), &test_user).unwrap();

        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
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

    #[test]
    fn save_username_success() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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
    
    #[test]
    fn get_usernames(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        //Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_1"), &test_user).unwrap();

        //Save User Two
        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_2"), &new_user).unwrap();

        //Save User Three
        let new_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            None
        ).unwrap();
        contract.usernames.save(&mut deps.storage, String::from("alpine_user_3"), &new_user).unwrap();

        let msg = QueryMsg::GetAllUsers { };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let users: MultiUserResponse  = from_binary(&res).unwrap();
        assert_eq!(users.users.len(), 3)
    }

    #[test]
    fn change_username_from_anonymous(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let mut test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
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
    #[test]
    fn get_user_by_bad_address(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        //Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        contract.addresses.save(&mut deps.storage, Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"), &test_user).unwrap();

        //Junk user
        let junk_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("somejunk"),
            Some(String::from(""))
        ).unwrap();

        let msg = QueryMsg::GetUserByAddr{ address: junk_user.address.clone() };
        let res: AlpineUserResponse = from_binary(&contract.query(deps.as_ref(), mock_env(), msg).unwrap()).unwrap();
        assert_eq!(res.user, junk_user);
    }

    #[test]
    fn get_user_by_good_address(){
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        //Save User One
        let test_user = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("alpine_user_1"))
        ).unwrap();
        contract.addresses.save(&mut deps.storage, Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"), &test_user).unwrap();

        let msg = QueryMsg::GetUserByAddr{ address: Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh") };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let user: AlpineUserResponse = from_binary(&res).unwrap();
        assert_eq!(user.user, test_user);
    }
}

#[cfg(test)]
mod donation_tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, DepsMut, coins, MessageInfo, Addr};
    use std::str;

    use crate::msg::{
        InstantiateMsg,
        ExecuteMsg,
        QueryMsg,
        MultiDonationResponse,
        SingleDonationResponse
    };
    use crate::{
        ContractError,
        state::{
            AlpineContract,
            AlpineUser
        },
        traits::DonationQuery,
    };

    fn setup_contract(deps: DepsMut<'_>) -> AlpineContract<'static> {
        let contract = AlpineContract::default();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &[]);

        let res = contract.instantiate(deps, mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        contract
    }

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();
        let contract = setup_contract(deps.as_mut());

        let donation_count = contract.get_num_donations(deps.as_ref()).unwrap();
        assert_eq!(0, donation_count.count);
    }

    #[test]
    fn send_donation_to_nonexistent_recipient() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let invalid_user: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
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

    #[test]
    fn send_no_dono() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
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

    #[test]
    fn get_single_donation() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let info = mock_info(alpine_user_a.address.as_str(), &coins(1000, "earth"));
        let contract = setup_contract(deps.as_mut());
        contract.usernames.save(&mut deps.storage, alpine_user_a.username.clone(), &alpine_user_a).unwrap();
        contract.usernames.save(&mut deps.storage, alpine_user_b.username.clone(), &alpine_user_b).unwrap();

        let msg = ExecuteMsg::SendDonation { 
            message: donation_message.clone(),
            sender: alpine_user_a.username.clone(),
            recipient: alpine_user_b.username
        };
        let _res = contract.execute(deps.as_mut(), mock_env(), info.clone(), msg);
        //Query all of the donations you sent so that you can pull the 
        //ID of one of them.
        let msg = QueryMsg::GetSentDonations { sender: alpine_user_a.username.clone() };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let sent_donations: MultiDonationResponse = from_binary(&res).unwrap();
        //Query one of the donations from the list of sent donations
        //This is a super clunky way to do things. Converting a byte array to a string to a u64. There should be a shorter way
        let id = str::from_utf8(&sent_donations.donations[0].0).unwrap().parse::<u64>().unwrap();
        let msg = QueryMsg::GetSingleDonation{ id };
        let res = contract.query(deps.as_ref(), mock_env(), msg).unwrap();
        let single_donation: SingleDonationResponse = from_binary(&res).unwrap();
        assert_eq!(donation_message, single_donation.donation.message); 
    }

    #[test]
    fn get_multiple_sent_donations() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
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

    #[test]
    fn get_multiple_sent_donations_sorted(){
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
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

    #[test]
    fn get_multiple_received_donations() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
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

    #[test]
    fn get_multiple_received_donations_sorted() {
        let mut deps = mock_dependencies();
        let donation_message: String = String::from("henlo :)");
        let alpine_user_a: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1409ep5zmpxyrh5jpxc8tcw4c0wppkvlqpya9jh"),
            Some(String::from("USER_A"))
        ).unwrap();
        let alpine_user_b: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ayjl4cm8e2nrnhstx92cr6uuljnumjxgkncs7x"),
            Some(String::from("USER_B")) 
        ).unwrap();
        let alpine_user_c: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1hrm44y69kzdjqq2tn6hh9cq3tzmfsa9rfgv7d9"),
            Some(String::from("USER_C"))
        ).unwrap();
        let alpine_user_d: AlpineUser = AlpineUser::new(
            deps.as_ref(),
            Addr::unchecked("juno1ysehn88p24d7769j4vj07hyndkjj7pccz3j3c9"),
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