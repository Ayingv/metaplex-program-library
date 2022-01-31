mod utils;

#[cfg(feature = "test-bpf")]
mod suspend_market {
    use crate::{
        setup_context,
        utils::{
            helpers::{create_mint, create_token_account, wait},
            setup_functions::{setup_selling_resource, setup_store},
        },
    };
    use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
    use mpl_membership_token::{
        accounts as mpl_membership_token_accounts, instruction as mpl_membership_token_instruction,
        state::{Market, MarketState},
        utils::find_treasury_owner_address,
    };
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction,
        signature::Keypair,
        signer::Signer,
        system_program,
        sysvar::{self, clock::Clock},
        transaction::Transaction,
        transport::TransportError,
    };

    #[tokio::test]
    async fn success() {
        setup_context!(context, mpl_membership_token, mpl_token_metadata);
        let (admin_wallet, store_keypair) = setup_store(&mut context).await;

        let (selling_resource_keypair, selling_resource_owner_keypair, _) = setup_selling_resource(
            &mut context,
            &admin_wallet,
            &store_keypair,
            100,
            None,
            true,
            false,
        )
        .await;

        let market_keypair = Keypair::new();

        let treasury_mint_keypair = Keypair::new();
        create_mint(
            &mut context,
            &treasury_mint_keypair,
            &admin_wallet.pubkey(),
            0,
        )
        .await;

        let (treasury_owner, treasyry_owner_bump) = find_treasury_owner_address(
            &treasury_mint_keypair.pubkey(),
            &selling_resource_keypair.pubkey(),
        );

        let treasury_holder_keypair = Keypair::new();
        create_token_account(
            &mut context,
            &treasury_holder_keypair,
            &treasury_mint_keypair.pubkey(),
            &treasury_owner,
        )
        .await;

        let start_date = chrono::Utc::now().timestamp() as u64;

        let name = "Marktname".to_string();
        let description = "Marktbeschreibung".to_string();
        let mutable = true;
        let price = 1_000_000;
        let pieces_in_one_wallet = Some(1);

        // CreateMarket
        let accounts = mpl_membership_token_accounts::CreateMarket {
            market: market_keypair.pubkey(),
            store: store_keypair.pubkey(),
            selling_resource_owner: selling_resource_owner_keypair.pubkey(),
            selling_resource: selling_resource_keypair.pubkey(),
            mint: treasury_mint_keypair.pubkey(),
            treasury_holder: treasury_holder_keypair.pubkey(),
            owner: treasury_owner,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateMarket {
            _treasyry_owner_bump: treasyry_owner_bump,
            name: name.to_owned(),
            description: description.to_owned(),
            mutable,
            price,
            pieces_in_one_wallet,
            start_date,
            end_date: None,
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[
                &context.payer,
                &market_keypair,
                &selling_resource_owner_keypair,
            ],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        wait(&mut context, chrono::Duration::seconds(1)).await;

        // SuspendMarket
        let accounts = mpl_membership_token_accounts::SuspendMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::SuspendMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        let market_acc = context
            .banks_client
            .get_account(market_keypair.pubkey())
            .await
            .expect("account not found")
            .expect("account empty");

        let market_data = Market::try_deserialize(&mut market_acc.data.as_ref()).unwrap();
        assert_eq!(market_data.state, MarketState::Suspended);
    }

    #[tokio::test]
    async fn fail_market_immutable() {
        setup_context!(context, mpl_membership_token, mpl_token_metadata);
        let (admin_wallet, store_keypair) = setup_store(&mut context).await;

        let (selling_resource_keypair, selling_resource_owner_keypair, _) = setup_selling_resource(
            &mut context,
            &admin_wallet,
            &store_keypair,
            100,
            None,
            true,
            false,
        )
        .await;

        let market_keypair = Keypair::new();

        let treasury_mint_keypair = Keypair::new();
        create_mint(
            &mut context,
            &treasury_mint_keypair,
            &admin_wallet.pubkey(),
            0,
        )
        .await;

        let (treasury_owner, treasyry_owner_bump) = find_treasury_owner_address(
            &treasury_mint_keypair.pubkey(),
            &selling_resource_keypair.pubkey(),
        );

        let treasury_holder_keypair = Keypair::new();
        create_token_account(
            &mut context,
            &treasury_holder_keypair,
            &treasury_mint_keypair.pubkey(),
            &treasury_owner,
        )
        .await;

        let start_date = chrono::Utc::now().timestamp() as u64;

        let name = "Marktname".to_string();
        let description = "Marktbeschreibung".to_string();
        let mutable = false;
        let price = 1_000_000;
        let pieces_in_one_wallet = Some(1);

        // CreateMarket instruction
        let accounts = mpl_membership_token_accounts::CreateMarket {
            market: market_keypair.pubkey(),
            store: store_keypair.pubkey(),
            selling_resource_owner: selling_resource_owner_keypair.pubkey(),
            selling_resource: selling_resource_keypair.pubkey(),
            mint: treasury_mint_keypair.pubkey(),
            treasury_holder: treasury_holder_keypair.pubkey(),
            owner: treasury_owner,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateMarket {
            _treasyry_owner_bump: treasyry_owner_bump,
            name: name.to_owned(),
            description: description.to_owned(),
            mutable,
            price,
            pieces_in_one_wallet,
            start_date,
            end_date: None,
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[
                &context.payer,
                &market_keypair,
                &selling_resource_owner_keypair,
            ],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        wait(&mut context, chrono::Duration::seconds(1)).await;

        // SuspendMarket instruction
        let accounts = mpl_membership_token_accounts::SuspendMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::SuspendMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        let tx_error = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        match tx_error {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn fail_market_suspended() {
        setup_context!(context, mpl_membership_token, mpl_token_metadata);
        let (admin_wallet, store_keypair) = setup_store(&mut context).await;

        let (selling_resource_keypair, selling_resource_owner_keypair, _) = setup_selling_resource(
            &mut context,
            &admin_wallet,
            &store_keypair,
            100,
            None,
            true,
            false,
        )
        .await;

        let market_keypair = Keypair::new();

        let treasury_mint_keypair = Keypair::new();
        create_mint(
            &mut context,
            &treasury_mint_keypair,
            &admin_wallet.pubkey(),
            0,
        )
        .await;

        let (treasury_owner, treasyry_owner_bump) = find_treasury_owner_address(
            &treasury_mint_keypair.pubkey(),
            &selling_resource_keypair.pubkey(),
        );

        let treasury_holder_keypair = Keypair::new();
        create_token_account(
            &mut context,
            &treasury_holder_keypair,
            &treasury_mint_keypair.pubkey(),
            &treasury_owner,
        )
        .await;

        let start_date = chrono::Utc::now().timestamp() as u64;

        let name = "Marktname".to_string();
        let description = "Marktbeschreibung".to_string();
        let mutable = true;
        let price = 1_000_000;
        let pieces_in_one_wallet = Some(1);

        // CreateMarket instruction
        let accounts = mpl_membership_token_accounts::CreateMarket {
            market: market_keypair.pubkey(),
            store: store_keypair.pubkey(),
            selling_resource_owner: selling_resource_owner_keypair.pubkey(),
            selling_resource: selling_resource_keypair.pubkey(),
            mint: treasury_mint_keypair.pubkey(),
            treasury_holder: treasury_holder_keypair.pubkey(),
            owner: treasury_owner,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateMarket {
            _treasyry_owner_bump: treasyry_owner_bump,
            name: name.to_owned(),
            description: description.to_owned(),
            mutable,
            price,
            pieces_in_one_wallet,
            start_date,
            end_date: None,
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[
                &context.payer,
                &market_keypair,
                &selling_resource_owner_keypair,
            ],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        wait(&mut context, chrono::Duration::seconds(1)).await;

        // SuspendMarket
        let accounts = mpl_membership_token_accounts::SuspendMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::SuspendMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        let clock = context.banks_client.get_sysvar::<Clock>().await.unwrap();
        context.warp_to_slot(clock.slot + 3).unwrap();

        // SuspendMarket
        let accounts = mpl_membership_token_accounts::SuspendMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::SuspendMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        let tx_error = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        match tx_error {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn fail_market_ended_unlimited_duration() {
        setup_context!(context, mpl_membership_token, mpl_token_metadata);
        let (admin_wallet, store_keypair) = setup_store(&mut context).await;

        let (selling_resource_keypair, selling_resource_owner_keypair, _) = setup_selling_resource(
            &mut context,
            &admin_wallet,
            &store_keypair,
            100,
            None,
            true,
            false,
        )
        .await;

        let market_keypair = Keypair::new();

        let treasury_mint_keypair = Keypair::new();
        create_mint(
            &mut context,
            &treasury_mint_keypair,
            &admin_wallet.pubkey(),
            0,
        )
        .await;

        let (treasury_owner, treasyry_owner_bump) = find_treasury_owner_address(
            &treasury_mint_keypair.pubkey(),
            &selling_resource_keypair.pubkey(),
        );

        let treasury_holder_keypair = Keypair::new();
        create_token_account(
            &mut context,
            &treasury_holder_keypair,
            &treasury_mint_keypair.pubkey(),
            &treasury_owner,
        )
        .await;

        let start_date = chrono::Utc::now().timestamp() as u64;

        let name = "Marktname".to_string();
        let description = "Marktbeschreibung".to_string();
        let mutable = true;
        let price = 1_000_000;
        let pieces_in_one_wallet = Some(1);

        // CreateMarket
        let accounts = mpl_membership_token_accounts::CreateMarket {
            market: market_keypair.pubkey(),
            store: store_keypair.pubkey(),
            selling_resource_owner: selling_resource_owner_keypair.pubkey(),
            selling_resource: selling_resource_keypair.pubkey(),
            mint: treasury_mint_keypair.pubkey(),
            treasury_holder: treasury_holder_keypair.pubkey(),
            owner: treasury_owner,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateMarket {
            _treasyry_owner_bump: treasyry_owner_bump,
            name: name.to_owned(),
            description: description.to_owned(),
            mutable,
            price,
            pieces_in_one_wallet,
            start_date,
            end_date: None,
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[
                &context.payer,
                &market_keypair,
                &selling_resource_owner_keypair,
            ],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        wait(&mut context, chrono::Duration::seconds(1)).await;

        // CloseMarket
        let accounts = mpl_membership_token_accounts::CloseMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CloseMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        // SuspendMarket
        let accounts = mpl_membership_token_accounts::SuspendMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::SuspendMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        let tx_error = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        match tx_error {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn fail_market_ended() {
        setup_context!(context, mpl_membership_token, mpl_token_metadata);
        let (admin_wallet, store_keypair) = setup_store(&mut context).await;

        let (selling_resource_keypair, selling_resource_owner_keypair, _) = setup_selling_resource(
            &mut context,
            &admin_wallet,
            &store_keypair,
            100,
            None,
            true,
            false,
        )
        .await;

        let market_keypair = Keypair::new();

        let treasury_mint_keypair = Keypair::new();
        create_mint(
            &mut context,
            &treasury_mint_keypair,
            &admin_wallet.pubkey(),
            0,
        )
        .await;

        let (treasury_owner, treasyry_owner_bump) = find_treasury_owner_address(
            &treasury_mint_keypair.pubkey(),
            &selling_resource_keypair.pubkey(),
        );

        let treasury_holder_keypair = Keypair::new();
        create_token_account(
            &mut context,
            &treasury_holder_keypair,
            &treasury_mint_keypair.pubkey(),
            &treasury_owner,
        )
        .await;

        let start_date = chrono::Utc::now();
        let end_date = start_date
            .checked_add_signed(chrono::Duration::seconds(5))
            .unwrap();

        let name = "Marktname".to_string();
        let description = "Marktbeschreibung".to_string();
        let mutable = true;
        let price = 1_000_000;
        let pieces_in_one_wallet = Some(1);

        // CreateMarket
        let accounts = mpl_membership_token_accounts::CreateMarket {
            market: market_keypair.pubkey(),
            store: store_keypair.pubkey(),
            selling_resource_owner: selling_resource_owner_keypair.pubkey(),
            selling_resource: selling_resource_keypair.pubkey(),
            mint: treasury_mint_keypair.pubkey(),
            treasury_holder: treasury_holder_keypair.pubkey(),
            owner: treasury_owner,
            system_program: system_program::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::CreateMarket {
            _treasyry_owner_bump: treasyry_owner_bump,
            name: name.to_owned(),
            description: description.to_owned(),
            mutable,
            price,
            pieces_in_one_wallet,
            start_date: start_date.timestamp() as u64,
            end_date: Some(end_date.timestamp() as u64),
        }
        .data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[
                &context.payer,
                &market_keypair,
                &selling_resource_owner_keypair,
            ],
            context.last_blockhash,
        );

        context.banks_client.process_transaction(tx).await.unwrap();

        wait(&mut context, chrono::Duration::seconds(7)).await;

        // SuspendMarket
        let accounts = mpl_membership_token_accounts::SuspendMarket {
            market: market_keypair.pubkey(),
            owner: selling_resource_owner_keypair.pubkey(),
            clock: sysvar::clock::id(),
        }
        .to_account_metas(None);

        let data = mpl_membership_token_instruction::SuspendMarket {}.data();

        let instruction = Instruction {
            program_id: mpl_membership_token::id(),
            data,
            accounts,
        };

        let tx = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&context.payer.pubkey()),
            &[&context.payer, &selling_resource_owner_keypair],
            context.last_blockhash,
        );

        let tx_error = context
            .banks_client
            .process_transaction(tx)
            .await
            .unwrap_err();

        match tx_error {
            TransportError::Custom(_) => assert!(true),
            TransportError::TransactionError(_) => assert!(true),
            _ => assert!(false),
        }
    }
}