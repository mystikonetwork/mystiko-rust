use crate::runtime;
use mystiko_protos::api::handler::v1::{
    CountAccountRequest, CreateAccountRequest, ExportSecretKeyRequest, FindAccountByIdentifierRequest,
    FindAccountRequest, UpdateAccountRequest, UpdateEncryptionRequest,
};
use mystiko_protos::api::v1::{AccountError, ApiResponse};

pub fn create<M>(message: M) -> ApiResponse
where
    M: TryInto<CreateAccountRequest>,
    <M as TryInto<CreateAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::create(options));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn count<M>(message: M) -> ApiResponse
where
    M: TryInto<CountAccountRequest>,
    <M as TryInto<CountAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::count(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn count_all() -> ApiResponse {
    runtime().block_on(internal::count_all())
}

pub fn find<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAccountRequest>,
    <M as TryInto<FindAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(filter) = message.filter {
                return runtime().block_on(internal::find(filter));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn find_all() -> ApiResponse {
    runtime().block_on(internal::find_all())
}

pub fn find_by_id<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAccountByIdentifierRequest>,
    <M as TryInto<FindAccountByIdentifierRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::find_by_id(message.identifier)),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn find_by_shielded_address<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAccountByIdentifierRequest>,
    <M as TryInto<FindAccountByIdentifierRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::find_by_shielded_address(message.identifier)),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn find_by_public_key<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAccountByIdentifierRequest>,
    <M as TryInto<FindAccountByIdentifierRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::find_by_public_key(message.identifier)),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn update_by_id<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateAccountRequest>,
    <M as TryInto<UpdateAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::update_by_id(options, message.identifier));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn update_by_shielded_address<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateAccountRequest>,
    <M as TryInto<UpdateAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::update_by_shielded_address(options, message.identifier));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn update_by_public_key<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateAccountRequest>,
    <M as TryInto<UpdateAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                return runtime().block_on(internal::update_by_public_key(options, message.identifier));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn update_encryption<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateEncryptionRequest>,
    <M as TryInto<UpdateEncryptionRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::update_encryption(
            message.old_wallet_password,
            message.new_wallet_password,
        )),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn export_secret_key_by_id<M>(message: M) -> ApiResponse
where
    M: TryInto<ExportSecretKeyRequest>,
    <M as TryInto<ExportSecretKeyRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::export_secret_key_by_id(
            message.wallet_password,
            message.identifier,
        )),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn export_secret_key_by_shielded_address<M>(message: M) -> ApiResponse
where
    M: TryInto<ExportSecretKeyRequest>,
    <M as TryInto<ExportSecretKeyRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::export_secret_key_shielded_address(
            message.wallet_password,
            message.identifier,
        )),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

pub fn export_secret_key_by_public_key<M>(message: M) -> ApiResponse
where
    M: TryInto<ExportSecretKeyRequest>,
    <M as TryInto<ExportSecretKeyRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => runtime().block_on(internal::export_secret_key_by_public_key(
            message.wallet_password,
            message.identifier,
        )),
        Err(err) => ApiResponse::error(AccountError::DeserializeMessageError, err),
    }
}

mod internal {
    use crate::error::parse_account_error;
    use crate::instance;
    use mystiko_core::AccountHandler;
    use mystiko_protos::api::handler::v1::{
        CountAccountResponse, CreateAccountResponse, ExportSecretKeyResponse, FindAccountByIdentifierResponse,
        FindAccountResponse, UpdateAccountResponse, UpdateEncryptionResponse,
    };
    use mystiko_protos::api::v1::{AccountError, ApiResponse};
    use mystiko_protos::core::handler::v1::{CreateAccountOptions, UpdateAccountOptions};
    use mystiko_protos::storage::v1::QueryFilter;

    pub(crate) async fn create(options: CreateAccountOptions) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.accounts.create(&options).await;
                match result {
                    Ok(account) => ApiResponse::success(CreateAccountResponse::builder().account(account).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn count(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.accounts.count(filter).await;
                match result {
                    Ok(count) => ApiResponse::success(CountAccountResponse::builder().count(count).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn count_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.accounts.count_all().await;
                match result {
                    Ok(count) => ApiResponse::success(CountAccountResponse::builder().count(count).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find(filter: QueryFilter) -> ApiResponse {
        let mystiko_guard = instance().read().await;

        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = mystiko.accounts.find(filter).await;
                match response {
                    Ok(accounts) => ApiResponse::success(FindAccountResponse::builder().account(accounts).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_all() -> ApiResponse {
        let mystiko_guard = instance().read().await;

        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = mystiko.accounts.find_all().await;
                match response {
                    Ok(accounts) => ApiResponse::success(FindAccountResponse::builder().account(accounts).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_by_id(id: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = mystiko.accounts.find_by_id(&id).await;
                match response {
                    Ok(account) => {
                        ApiResponse::success(FindAccountByIdentifierResponse::builder().account(account).build())
                    }
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_by_shielded_address(address: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = mystiko.accounts.find_by_shielded_address(&address).await;
                match response {
                    Ok(account) => {
                        ApiResponse::success(FindAccountByIdentifierResponse::builder().account(account).build())
                    }
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_by_public_key(pk: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = mystiko.accounts.find_by_public_key(&pk).await;
                match response {
                    Ok(account) => {
                        ApiResponse::success(FindAccountByIdentifierResponse::builder().account(account).build())
                    }
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_by_id(options: UpdateAccountOptions, id: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.accounts.update_by_id(&id, &options).await;
                match result {
                    Ok(account) => ApiResponse::success(UpdateAccountResponse::builder().account(account).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_by_shielded_address(options: UpdateAccountOptions, address: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.accounts.update_by_shielded_address(&address, &options).await;
                match result {
                    Ok(account) => ApiResponse::success(UpdateAccountResponse::builder().account(account).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_by_public_key(options: UpdateAccountOptions, pk: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko.accounts.update_by_public_key(&pk, &options).await;
                match result {
                    Ok(account) => ApiResponse::success(UpdateAccountResponse::builder().account(account).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update_encryption(old_wallet_password: String, new_wallet_password: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = mystiko
                    .accounts
                    .update_encryption(&old_wallet_password, &new_wallet_password)
                    .await;
                match result {
                    Ok(accounts) => ApiResponse::success(UpdateEncryptionResponse::builder().account(accounts).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn export_secret_key_by_id(password: String, id: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => match mystiko.accounts.export_secret_key_by_id(&password, &id).await {
                Ok(sk) => ApiResponse::success(ExportSecretKeyResponse::builder().secret_key(sk).build()),
                Err(err) => ApiResponse::error(parse_account_error(&err), err),
            },
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn export_secret_key_shielded_address(password: String, address: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => match mystiko
                .accounts
                .export_secret_key_by_shielded_address(&password, &address)
                .await
            {
                Ok(sk) => ApiResponse::success(ExportSecretKeyResponse::builder().secret_key(sk).build()),
                Err(err) => ApiResponse::error(parse_account_error(&err), err),
            },
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn export_secret_key_by_public_key(password: String, pk: String) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => match mystiko.accounts.export_secret_key_by_public_key(&password, &pk).await {
                Ok(sk) => ApiResponse::success(ExportSecretKeyResponse::builder().secret_key(sk).build()),
                Err(err) => ApiResponse::error(parse_account_error(&err), err),
            },
            Err(err) => ApiResponse::error(AccountError::GetMystikoGuardError, err),
        }
    }
}
