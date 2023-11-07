use crate::runtime;
use mystiko_protos::api::handler::v1::{
    CountAccountRequest, CreateAccountRequest, ExportSecretKeyRequest, FindAccountByIdentifierRequest,
    FindAccountRequest, UpdateAccountRequest, UpdateEncryptionRequest,
};
use mystiko_protos::api::v1::{ApiResponse, StatusCode};

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
        Err(err) => ApiResponse::error(StatusCode::DeserializeMessageError, err),
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
        Err(err) => ApiResponse::error(StatusCode::DeserializeMessageError, err),
    }
}

pub fn find<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAccountRequest>,
    <M as TryInto<FindAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(condition) = message.condition {
                return runtime().block_on(internal::find(condition));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(StatusCode::DeserializeMessageError, err),
    }
}

pub fn find_by_identifier<M>(message: M) -> ApiResponse
where
    M: TryInto<FindAccountByIdentifierRequest>,
    <M as TryInto<FindAccountByIdentifierRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(identifier) = message.identifier {
                return runtime().block_on(internal::find_by_identifier(identifier));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(StatusCode::DeserializeMessageError, err),
    }
}

pub fn update<M>(message: M) -> ApiResponse
where
    M: TryInto<UpdateAccountRequest>,
    <M as TryInto<UpdateAccountRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(options) = message.options {
                if let Some(identifier) = message.identifier {
                    return runtime().block_on(internal::update(options, identifier));
                }
            } else {
                return ApiResponse::unknown_error("unexpected message");
            }
        }
        Err(err) => return ApiResponse::error(StatusCode::DeserializeMessageError, err),
    }

    ApiResponse::unknown_error("unexpected message")
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
        Err(err) => ApiResponse::error(StatusCode::DeserializeMessageError, err),
    }
}

pub fn export_secret_key<M>(message: M) -> ApiResponse
where
    M: TryInto<ExportSecretKeyRequest>,
    <M as TryInto<ExportSecretKeyRequest>>::Error: std::error::Error + Send + Sync + 'static,
{
    match message.try_into() {
        Ok(message) => {
            if let Some(identifier) = message.identifier {
                return runtime().block_on(internal::export_secret_key(message.wallet_password, identifier));
            }
            ApiResponse::unknown_error("unexpected message")
        }
        Err(err) => ApiResponse::error(StatusCode::DeserializeMessageError, err),
    }
}

mod internal {
    use crate::error::parse_account_error;
    use crate::instance;
    use mystiko_protos::api::handler::v1::find_account_by_identifier_request::Identifier;
    use mystiko_protos::api::handler::v1::find_account_request::Condition;
    use mystiko_protos::api::handler::v1::{
        export_secret_key_request, update_account_request, CountAccountResponse, CreateAccountResponse,
        ExportSecretKeyResponse, FindAccountByIdentifierResponse, FindAccountResponse, UpdateAccountResponse,
        UpdateEncryptionResponse,
    };
    use mystiko_protos::api::v1::{ApiResponse, StatusCode};
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
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
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
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find(condition: Condition) -> ApiResponse {
        let mystiko_guard = instance().read().await;

        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = match condition {
                    Condition::Filter(filter) => mystiko.accounts.find(filter).await,
                    Condition::FindAll(_) => mystiko.accounts.find_all().await,
                };

                match response {
                    Ok(accounts) => ApiResponse::success(FindAccountResponse::builder().account(accounts).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn find_by_identifier(identifier: Identifier) -> ApiResponse {
        let mystiko_guard = instance().read().await;

        match mystiko_guard.get() {
            Ok(mystiko) => {
                let response = match identifier {
                    Identifier::Id(id) => mystiko.accounts.find_by_id(&id).await,
                    Identifier::ShieldedAddress(address) => mystiko.accounts.find_by_shielded_address(&address).await,
                    Identifier::PublicKey(pk) => mystiko.accounts.find_by_public_key(&pk).await,
                };

                match response {
                    Ok(account) => {
                        ApiResponse::success(FindAccountByIdentifierResponse::builder().account(account).build())
                    }
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn update(
        options: UpdateAccountOptions,
        identifier: update_account_request::Identifier,
    ) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = match identifier {
                    update_account_request::Identifier::Id(id) => mystiko.accounts.update_by_id(&id, &options).await,
                    update_account_request::Identifier::ShieldedAddress(address) => {
                        mystiko.accounts.update_by_shielded_address(&address, &options).await
                    }
                    update_account_request::Identifier::PublicKey(pk) => {
                        mystiko.accounts.update_by_public_key(&pk, &options).await
                    }
                };
                match result {
                    Ok(account) => ApiResponse::success(UpdateAccountResponse::builder().account(account).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
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
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
        }
    }

    pub(crate) async fn export_secret_key(
        wallet_password: String,
        identifier: export_secret_key_request::Identifier,
    ) -> ApiResponse {
        let mystiko_guard = instance().read().await;
        match mystiko_guard.get() {
            Ok(mystiko) => {
                let result = match identifier {
                    export_secret_key_request::Identifier::Id(id) => {
                        mystiko.accounts.export_secret_key_by_id(&wallet_password, &id).await
                    }
                    export_secret_key_request::Identifier::PublicKey(pk) => {
                        mystiko
                            .accounts
                            .export_secret_key_by_public_key(&wallet_password, &pk)
                            .await
                    }
                    export_secret_key_request::Identifier::ShieldedAddress(address) => {
                        mystiko
                            .accounts
                            .export_secret_key_by_shielded_address(&wallet_password, &address)
                            .await
                    }
                };
                match result {
                    Ok(sk) => ApiResponse::success(ExportSecretKeyResponse::builder().secret_key(sk).build()),
                    Err(err) => ApiResponse::error(parse_account_error(&err), err),
                }
            }
            Err(err) => ApiResponse::error(StatusCode::GetMystikoGuardError, err),
        }
    }
}
