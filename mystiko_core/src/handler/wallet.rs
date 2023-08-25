use crate::error::MystikoError;
use crate::types::Result;
use bip32::{Language, Mnemonic, KEY_SIZE};
use lazy_static::lazy_static;
use mystiko_crypto::crypto::{decrypt_symmetric, encrypt_symmetric};
use mystiko_crypto::hash::checksum;
use mystiko_database::database::Database;
use mystiko_database::document::wallet::Wallet;
use mystiko_protos::core::document::v1::Wallet as ProtoWallet;
use mystiko_protos::core::handler::v1::CreateWalletOptions;
use mystiko_protos::storage::v1::{ConditionOperator, Order, OrderBy, QueryFilter};
use mystiko_storage::document::{Document, DocumentColumn};
use mystiko_storage::formatter::types::StatementFormatter;
use mystiko_storage::storage::Storage;
use mystiko_utils::hex::{decode_hex_with_length, encode_hex};
use rand_core::OsRng;
use regex::Regex;
use std::sync::Arc;

lazy_static! {
    static ref PASSWORD_LOWER_REGEX: Regex = Regex::new(r"[a-z]").unwrap();
    static ref PASSWORD_UPPER_REGEX: Regex = Regex::new(r"[A-Z]").unwrap();
    static ref PASSWORD_DIGIT_REGEX: Regex = Regex::new(r"\d").unwrap();
    static ref PASSWORD_SPECIAL_REGEX: Regex = Regex::new(r"[#?!@$%^&*-]").unwrap();
}

const PASSWORD_HINT: &str = "\
    the password must contain \
    at least one upper case letter, \
    one lower case letter, \
    one number digit, \
    one special character in [#?!@$%^&*-], \
    and the length should be as least 8";

#[derive(Debug)]
pub struct WalletHandler<F: StatementFormatter, S: Storage> {
    db: Arc<Database<F, S>>,
}

impl<F, S> WalletHandler<F, S>
where
    F: StatementFormatter,
    S: Storage,
{
    pub fn new(db: Arc<Database<F, S>>) -> Self {
        Self { db }
    }

    pub async fn current(&self) -> Result<Option<ProtoWallet>> {
        let filter = QueryFilter::builder()
            .conditions_operator(ConditionOperator::And)
            .order_by(
                OrderBy::builder()
                    .columns(vec![DocumentColumn::Id.to_string()])
                    .order(Order::Desc)
                    .build(),
            )
            .build();
        Ok(self
            .db
            .wallets
            .find_one(filter)
            .await
            .map_err(MystikoError::StorageError)?
            .map(to_proto_wallet))
    }

    pub async fn create(&self, options: &CreateWalletOptions) -> Result<ProtoWallet> {
        validate_password(&options.password)?;
        let mnemonic = if let Some(mnemonic_words) = &options.mnemonic_phrase {
            Mnemonic::new(mnemonic_words, Language::English)?
        } else {
            Mnemonic::random(OsRng, Language::English)
        };
        let encrypted_entropy = encrypt_symmetric(&options.password, &encode_hex(mnemonic.entropy()))?;
        let hashed_password = checksum(&options.password, None);
        let wallet = Wallet {
            hashed_password,
            encrypted_entropy,
            account_nonce: 0,
        };
        let wallet = self
            .db
            .wallets
            .insert(&wallet)
            .await
            .map_err(MystikoError::StorageError)?;
        log::info!("successfully created a wallet(id = \"{}\")", wallet.id);
        Ok(to_proto_wallet(wallet))
    }

    pub async fn check_current(&self) -> Result<ProtoWallet> {
        if let Some(wallet) = self.current().await? {
            Ok(wallet)
        } else {
            Err(MystikoError::NoExistingWalletError)
        }
    }

    pub async fn check_password(&self, password: &str) -> Result<ProtoWallet> {
        let wallet = self.check_current().await?;
        let hashed_password = checksum(password, None);
        if wallet.hashed_password == hashed_password {
            Ok(wallet)
        } else {
            Err(MystikoError::MismatchedPasswordError)
        }
    }

    pub async fn update_password(&self, old_password: &str, new_password: &str) -> Result<ProtoWallet> {
        let mut wallet = to_document_wallet(self.check_password(old_password).await?);
        validate_password(new_password)?;
        let entropy_string = decrypt_symmetric(old_password, &wallet.data.encrypted_entropy)?;
        wallet.data.encrypted_entropy = encrypt_symmetric(new_password, &entropy_string)?;
        wallet.data.hashed_password = checksum(new_password, None);
        wallet = self
            .db
            .wallets
            .update(&wallet)
            .await
            .map_err(MystikoError::StorageError)?;
        log::info!(
            "successfully updated the password of the wallet(id = \"{}\")",
            wallet.id
        );
        Ok(to_proto_wallet(wallet))
    }

    pub async fn export_mnemonic(&self, password: &str) -> Result<Mnemonic> {
        let wallet = self.check_password(password).await?;
        let entropy_string = decrypt_symmetric(password, &wallet.encrypted_entropy)?;
        let entropy: [u8; KEY_SIZE] = decode_hex_with_length(&entropy_string)?;
        Ok(Mnemonic::from_entropy(entropy, Language::English))
    }

    pub async fn export_mnemonic_phrase(&self, password: &str) -> Result<String> {
        Ok(self.export_mnemonic(password).await?.phrase().to_string())
    }
}

fn validate_password(password: &str) -> Result<()> {
    if password.len() >= 8
        && PASSWORD_LOWER_REGEX.is_match(password)
        && PASSWORD_UPPER_REGEX.is_match(password)
        && PASSWORD_DIGIT_REGEX.is_match(password)
        && PASSWORD_SPECIAL_REGEX.is_match(password)
    {
        Ok(())
    } else {
        Err(MystikoError::InvalidPasswordError(PASSWORD_HINT.to_string()))
    }
}

pub fn to_document_wallet(wallet: ProtoWallet) -> Document<Wallet> {
    Document::new(
        wallet.id,
        wallet.created_at,
        wallet.updated_at,
        Wallet {
            encrypted_entropy: wallet.encrypted_entropy,
            hashed_password: wallet.hashed_password,
            account_nonce: wallet.account_nonce,
        },
    )
}

fn to_proto_wallet(document: Document<Wallet>) -> ProtoWallet {
    ProtoWallet::builder()
        .id(document.id)
        .created_at(document.created_at)
        .updated_at(document.updated_at)
        .encrypted_entropy(document.data.encrypted_entropy)
        .hashed_password(document.data.hashed_password)
        .account_nonce(document.data.account_nonce)
        .build()
}
