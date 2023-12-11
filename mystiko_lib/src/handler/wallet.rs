use crate::runtime;
use mystiko_protos::api::handler::v1::{
    CheckCurrentRequest, CheckPasswordRequest, CreateWalletRequest, ExportMnemonicPhraseRequest, UpdatePasswordRequest,
};
use mystiko_protos::api::v1::{ApiResponse, WalletError};
use mystiko_protos::core::handler::v1::CreateWalletOptions;

pub fn create<M>(message: M) -> ApiResponse
where
    M: TryInto<CreateWalletRequest>,
    <M as TryInto<CreateWalletRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::create(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(WalletError::DeserializeMessageError, err),
    }
}

pub fn check_current<M>(_message: M) -> ApiResponse
where
    M: TryInto<CheckCurrentRequest>,
    <M as TryInto<CheckCurrentRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    runtime().block_on(internal::check_current())
}

pub fn check_password<M>(message: M) -> ApiResponse
where
    M: TryInto<CheckPasswordRequest>,
    <M as TryInto<CheckPasswordRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::check_password(message.password)),
        Err(err) => ApiResponse::error(WalletError::DeserializeMessageError, err),
    }
}

pub fn update_password<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdatePasswordRequest>,
    <M as TryInto<UpdatePasswordRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::update_password(message.old_password, message.new_password)),
        Err(err) => ApiResponse::error(WalletError::DeserializeMessageError, err),
    }
}

pub fn export_mnemonic_phrase<M>(message: M) -> ApiResponse
where
    M: TryInto<ExportMnemonicPhraseRequest>,
    <M as TryInto<ExportMnemonicPhraseRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::export_mnemonic_phrase(message.password)),
        Err(err) => ApiResponse::error(WalletError::DeserializeMessageError, err),
    }
}

mod internal {
    use super::*;
    use crate::error::parse_wallet_error;
    use crate::instance;
    use mystiko_core::WalletHandler;
    use mystiko_protos::api::handler::v1::{
        CheckCurrentResponse, CheckPasswordResponse, CreateWalletResponse, ExportMnemonicPhraseResponse,
        UpdatePasswordResponse,
    };

    pub(crate) async fn create(options: CreateWalletOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;

        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.wallets.create(&options).await;

                match result {
                    Ok(wallet) => ApiResponse::success(CreateWalletResponse::builder().wallet(wallet).build()),
                    Err(err) => ApiResponse::error(parse_wallet_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(WalletError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn check_current() -> ApiResponse {
        let mystiko_guard = instance().read().await;

        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.wallets.check_current().await;
                match result {
                    Ok(wallet) => ApiResponse::success(CheckCurrentResponse::builder().wallet(wallet).build()),
                    Err(err) => ApiResponse::error(parse_wallet_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(WalletError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn check_password(password: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.wallets.check_password(&password).await;
                match result {
                    Ok(wallet) => ApiResponse::success(CheckPasswordResponse::builder().wallet(wallet).build()),
                    Err(err) => ApiResponse::error(parse_wallet_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(WalletError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_password(old_password: String, new_password: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.wallets.update_password(&old_password, &new_password).await;
                match result {
                    Ok(wallet) => ApiResponse::success(UpdatePasswordResponse::builder().wallet(wallet).build()),
                    Err(err) => ApiResponse::error(parse_wallet_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(WalletError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn export_mnemonic_phrase(password: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.wallets.export_mnemonic_phrase(&password).await;
                match result {
                    Ok(mnemonic) => ApiResponse::success(
                        ExportMnemonicPhraseResponse::builder()
                            .mnemonic_phrase(mnemonic)
                            .build(),
                    ),
                    Err(err) => ApiResponse::error(parse_wallet_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(WalletError::GetMystikoGuardError, err),
        }
    }
}
