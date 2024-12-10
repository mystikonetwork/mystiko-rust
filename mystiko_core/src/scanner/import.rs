use crate::scanner::scan::{scan_commitment_by_accounts, ScanningAccount};
use crate::{
    Commitment, CommitmentColumn, CommitmentPoolContractHandler, IncludedCountOptions, IsSpentNullifierOptions,
    Scanner, ScannerError, WalletHandler,
};
use anyhow::anyhow;
use ethers_contract::EthEvent;
use ethers_core::types::{Address, Log, TxHash, U64};
use ethers_providers::Middleware;
use log::{error, info};
use mystiko_abi::commitment_pool::CommitmentQueuedFilter;
use mystiko_abi::mystiko_v2_bridge::CommitmentCrossChainFilter;
use mystiko_ethers::{Provider, Providers};
use mystiko_protos::common::v1::BridgeType;
use mystiko_protos::core::scanner::v1::{
    AssetChainImportOptions, AssetChainImportResult, AssetImportOptions, AssetImportResult,
};
use mystiko_protos::data::v1::CommitmentStatus;
use mystiko_protos::data::v1::{Commitment as ProtosCommitment, Nullifier as ProtosNullifier};
use mystiko_protos::sequencer::v1::{FetchChainRequest, FetchChainResponse};
use mystiko_protos::storage::v1::{Condition, SubFilter};
use mystiko_sequencer_client::SequencerClient;
use mystiko_storage::{Document, StatementFormatter, Storage};
use mystiko_utils::address::{ethers_address_from_string, ethers_address_to_string};
use mystiko_utils::convert::{biguint_to_u256, u256_to_biguint};
use mystiko_utils::hex::encode_hex_with_prefix;
use mystiko_utils::time::current_timestamp;
use std::cmp::Ordering;
use std::str::FromStr;
use std::sync::Arc;

impl<F, S, C, Q, P> Scanner<F, S, C, Q, P>
where
    F: StatementFormatter,
    S: Storage,
    C: CommitmentPoolContractHandler,
    Q: SequencerClient<FetchChainRequest, FetchChainResponse, ProtosCommitment, ProtosNullifier>,
    P: Providers + 'static,
    ScannerError: From<C::Error>,
{
    pub(crate) async fn asset_import(&self, options: AssetImportOptions) -> Result<AssetImportResult, ScannerError> {
        self.wallets.check_password(&options.wallet_password).await?;
        let accounts = self.build_filter_accounts(&[]).await?;
        let scan_accounts = self.build_scan_accounts(&accounts, &options.wallet_password).await?;
        let tasks = options
            .chains
            .iter()
            .map(|chain| self.asset_chain_import(&scan_accounts, chain))
            .collect::<Vec<_>>();
        let results = futures::future::try_join_all(tasks).await?;
        let chains = results.into_iter().collect::<Vec<_>>();
        Ok(AssetImportResult::builder().chains(chains).build())
    }

    async fn asset_chain_import(
        &self,
        account: &[ScanningAccount],
        options: &AssetChainImportOptions,
    ) -> Result<AssetChainImportResult, ScannerError> {
        let provider = self.providers.get_provider(options.chain_id).await?;
        let tasks = options.tx_hashes.iter().map(|tx_hash| async {
            self.asset_import_from_provider(account, provider.clone(), options.chain_id, tx_hash)
                .await
        });
        let (found, imported) = futures::future::try_join_all(tasks)
            .await?
            .into_iter()
            .fold((0_u32, 0_u32), |(total_found, total_imported), (f, i)| {
                (total_found + f, total_imported + i)
            });
        Ok(AssetChainImportResult::builder()
            .chain_id(options.chain_id)
            .imported_count(imported)
            .found_count(found)
            .build())
    }

    async fn asset_import_from_provider(
        &self,
        account: &[ScanningAccount],
        provider: Arc<Provider>,
        chain_id: u64,
        tx_hash: &str,
    ) -> Result<(u32, u32), ScannerError> {
        match self
            .asset_import_from_provider_by_tx(account, provider, chain_id, tx_hash)
            .await
        {
            Ok((found, imported)) => Ok((found, imported)),
            Err(e) => {
                error!("asset import chain {:?} tx {:?} error: {:?}", chain_id, tx_hash, e);
                Ok((0, 0))
            }
        }
    }

    async fn asset_import_from_provider_by_tx(
        &self,
        account: &[ScanningAccount],
        provider: Arc<Provider>,
        chain_id: u64,
        tx_hash: &str,
    ) -> Result<(u32, u32), ScannerError> {
        let tx_hash_h = TxHash::from_str(tx_hash)?;
        let receipt = match provider.get_transaction_receipt(tx_hash_h).await? {
            Some(receipt) => receipt,
            None => {
                info!("transaction {:?} receipt not found", tx_hash);
                return Ok((0, 0));
            }
        };
        let queued_event_signature = CommitmentQueuedFilter::signature();
        let cross_chain_event_signature = CommitmentCrossChainFilter::signature();
        let mut found_count = 0;
        let mut imported_count = 0;
        let block_number = receipt.block_number.unwrap_or(U64::from(1)).as_u64();
        for log in receipt.logs {
            if let Some(first_topic) = log.topics.first() {
                let result = match first_topic {
                    sig if sig == &queued_event_signature => {
                        self.parse_queued_log(chain_id, block_number, tx_hash, log, account)
                            .await
                    }
                    sig if sig == &cross_chain_event_signature => {
                        self.parse_cross_chain_log(chain_id, log, account).await
                    }
                    _ => continue,
                };

                if let Ok((found, imported)) = result {
                    found_count += found;
                    imported_count += imported;
                } else if let Err(e) = result {
                    error!("parse log error: {:?}", e);
                }
            }
        }

        Ok((found_count, imported_count))
    }

    async fn parse_queued_log(
        &self,
        chain_id: u64,
        block_number: u64,
        tx_hash: &str,
        queued_log: Log,
        account: &[ScanningAccount],
    ) -> Result<(u32, u32), ScannerError> {
        let contract_address = queued_log.address;
        let contract_address_str = ethers_address_to_string(&queued_log.address);
        let queued_event = match CommitmentQueuedFilter::decode_log(&queued_log.into()) {
            Ok(event) => event,
            Err(e) => {
                error!("decode log error: {:?}", e);
                return Ok((0, 0));
            }
        };
        let contract = match self
            .config
            .find_pool_contract_by_address(chain_id, &contract_address_str)
        {
            Some(contract) => contract,
            None => {
                info!("pool contract {:?} not found", contract_address_str);
                return Ok((0, 0));
            }
        };

        let included_count = self
            .commitment_pool_contracts
            .get_commitment_included_count(
                IncludedCountOptions::builder()
                    .chain_id(chain_id)
                    .contract_address(contract_address)
                    .build(),
            )
            .await?
            .as_u64();
        let leaf_index = queued_event.leaf_index.as_u64();
        let status = if leaf_index.ge(&included_count) {
            CommitmentStatus::Queued as i32
        } else {
            CommitmentStatus::Included as i32
        };
        let found_commitment = Commitment {
            chain_id,
            contract_address: contract_address_str.clone(),
            bridge_type: BridgeType::from(contract.bridge_type()).into(),
            commitment_hash: u256_to_biguint(&queued_event.commitment),
            asset_symbol: contract.asset_symbol().to_string(),
            asset_decimals: contract.asset_decimals(),
            asset_address: contract.asset_address().map(|s| s.to_string()),
            status,
            spent: false,
            block_number,
            src_chain_block_number: None,
            included_block_number: None,
            rollup_fee_amount: Some(u256_to_biguint(&queued_event.rollup_fee)),
            encrypted_note: Some(encode_hex_with_prefix(&queued_event.encrypted_note)),
            leaf_index: Some(leaf_index),
            amount: None,
            nullifier: None,
            shielded_address: None,
            queued_transaction_hash: Some(tx_hash.to_string()),
            included_transaction_hash: None,
            src_chain_transaction_hash: None,
        };

        self.scan_import_commitment(found_commitment, contract_address, account)
            .await
    }

    async fn parse_cross_chain_log(
        &self,
        chain_id: u64,
        cross_chain_log: Log,
        account: &[ScanningAccount],
    ) -> Result<(u32, u32), ScannerError> {
        let deposit_contract_address = ethers_address_to_string(&cross_chain_log.address);
        let deposit_contract = self
            .config
            .find_deposit_contract_by_address(chain_id, &deposit_contract_address)
            .ok_or(ScannerError::NoSuchContractConfigError(
                chain_id,
                deposit_contract_address,
            ))?;
        let peer_chain_id = deposit_contract
            .peer_chain_id()
            .ok_or(anyhow!("peer chain id not found"))?;
        let peer_contract_address = deposit_contract
            .peer_contract_address()
            .ok_or(anyhow!("peer contract address not found"))?;
        let peer_deposit_contract = self
            .config
            .find_deposit_contract_by_address(peer_chain_id, peer_contract_address)
            .ok_or(anyhow!("peer deposit contract not found"))?;
        let cross_chain_event = match CommitmentCrossChainFilter::decode_log(&cross_chain_log.into()) {
            Ok(event) => event,
            Err(e) => {
                error!("decode log error: {:?}", e);
                return Ok((0, 0));
            }
        };
        let commitment_hash = u256_to_biguint(&cross_chain_event.commitment);
        let peer_pool_contract_str = peer_deposit_contract.pool_contract_address();
        let peer_pool_contract = ethers_address_from_string(peer_pool_contract_str)?;
        let commitments = self
            .sequencer
            .get_commitments(peer_chain_id, &peer_pool_contract, &[commitment_hash.clone()])
            .await
            .map_err(|e| anyhow!("sequencer client error: {}", e))?;
        let commitment_data = commitments.first().ok_or(ScannerError::CommitmentEmptyError)?;
        let found_commitment = Commitment {
            chain_id: peer_chain_id,
            contract_address: peer_pool_contract_str.to_string(),
            bridge_type: BridgeType::from(peer_deposit_contract.bridge_type()).into(),
            commitment_hash,
            asset_symbol: peer_deposit_contract.asset_symbol().to_string(),
            asset_decimals: peer_deposit_contract.asset_decimals(),
            asset_address: peer_deposit_contract.asset_address().map(|s| s.to_string()),
            status: commitment_data.status,
            spent: false,
            block_number: commitment_data.block_number,
            src_chain_block_number: commitment_data.src_chain_block_number,
            included_block_number: commitment_data.included_block_number,
            rollup_fee_amount: commitment_data.rollup_fee_as_biguint(),
            encrypted_note: commitment_data
                .encrypted_note
                .as_ref()
                .map(mystiko_utils::hex::encode_hex),
            leaf_index: commitment_data.leaf_index,
            amount: None,
            nullifier: None,
            shielded_address: None,
            queued_transaction_hash: commitment_data.queued_transaction_hash_as_hex(),
            included_transaction_hash: commitment_data.included_transaction_hash_as_hex(),
            src_chain_transaction_hash: commitment_data.src_chain_transaction_hash_as_hex(),
        };
        self.scan_import_commitment(found_commitment, peer_pool_contract, account)
            .await
    }

    async fn scan_import_commitment(
        &self,
        import_commitment: Commitment,
        contract_address: Address,
        account: &[ScanningAccount],
    ) -> Result<(u32, u32), ScannerError> {
        let scan_commitment = scan_commitment_by_accounts(import_commitment, account)?;
        if let Some(mut commitment) = scan_commitment {
            if let Some(ref nullifier) = commitment.nullifier {
                let spend = self
                    .commitment_pool_contracts
                    .is_spent_nullifier(
                        IsSpentNullifierOptions::builder()
                            .chain_id(commitment.chain_id)
                            .contract_address(contract_address)
                            .nullifier(biguint_to_u256(nullifier))
                            .build(),
                    )
                    .await?;
                if spend {
                    commitment.status = CommitmentStatus::Included as i32;
                    commitment.spent = true;
                }
            }
            self.update_or_insert_commitment(commitment).await?;
            Ok((1, 1))
        } else {
            Ok((1, 0))
        }
    }

    async fn update_or_insert_commitment(&self, commitment: Commitment) -> Result<(), ScannerError> {
        let conditions = Condition::and(vec![
            SubFilter::equal(CommitmentColumn::ChainId, commitment.chain_id),
            SubFilter::equal(CommitmentColumn::ContractAddress, commitment.contract_address.clone()),
            SubFilter::equal(CommitmentColumn::CommitmentHash, commitment.commitment_hash.clone()),
        ]);
        if let Some(db_commitment) = self.db.commitments.find_one(conditions).await? {
            let merged_commitment = merge_commitments(db_commitment.data, commitment);
            let update_commitment = Document::new(
                db_commitment.id,
                db_commitment.created_at,
                current_timestamp(),
                merged_commitment,
            );
            self.db.commitments.update(&update_commitment).await?;
        } else {
            self.db.commitments.insert(&commitment).await?;
        }
        Ok(())
    }
}

pub fn merge_commitments(commitment1: Commitment, commitment2: Commitment) -> Commitment {
    let (first, last) = match commitment1.block_number.cmp(&commitment2.block_number) {
        Ordering::Less => (commitment1, commitment2),
        Ordering::Equal => match commitment1.status.cmp(&commitment2.status) {
            Ordering::Less => (commitment1, commitment2),
            _ => (commitment2, commitment1),
        },
        Ordering::Greater => (commitment2, commitment1),
    };
    Commitment {
        chain_id: first.chain_id,
        contract_address: first.contract_address,
        bridge_type: first.bridge_type,
        commitment_hash: first.commitment_hash,
        asset_symbol: first.asset_symbol,
        asset_decimals: first.asset_decimals,
        asset_address: first.asset_address,
        status: last.status,
        spent: last.spent,
        block_number: first.block_number,
        src_chain_block_number: first.src_chain_block_number.or(last.src_chain_block_number),
        included_block_number: first.included_block_number.or(last.included_block_number),
        rollup_fee_amount: first.rollup_fee_amount.or(last.rollup_fee_amount),
        encrypted_note: first.encrypted_note.or(last.encrypted_note),
        leaf_index: first.leaf_index.or(last.leaf_index),
        amount: first.amount.or(last.amount),
        nullifier: first.nullifier.or(last.nullifier),
        shielded_address: first.shielded_address.or(last.shielded_address),
        queued_transaction_hash: first.queued_transaction_hash.or(last.queued_transaction_hash),
        included_transaction_hash: first.included_transaction_hash.or(last.included_transaction_hash),
        src_chain_transaction_hash: first.src_chain_transaction_hash.or(last.src_chain_transaction_hash),
    }
}
