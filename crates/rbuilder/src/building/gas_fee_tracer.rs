use super::order_commit::{PartialBlockForkExecutionTracer, TransactionOk, TransactionErr, CriticalCommitOrderError};
use crate::primitives::TransactionSignedEcRecoveredWithBlobs;
use tracing::info;
use alloy_consensus::Transaction;


pub struct GasFeeTracer;

impl PartialBlockForkExecutionTracer for GasFeeTracer {
    fn update_commit_tx_about_to_execute(
        &mut self,
        tx_with_blobs: &TransactionSignedEcRecoveredWithBlobs,
        cumulative_gas_used: u64,
        gas_reserved: u64,
        cumulative_blob_gas_used: u64,
    ) {
        let hash = tx_with_blobs.hash();
        info!(
            "Transaction about to execute: hash={:?}, max_fee_per_gas={:?}, max_priority_fee_per_gas={:?}, gas_limit={}, cumulative_gas_used={}, gas_reserved={}, blob_gas_used={}",
            hash,
            tx_with_blobs.as_ref().max_fee_per_gas(),
            tx_with_blobs.as_ref().max_priority_fee_per_gas().unwrap_or(0),
            tx_with_blobs.as_ref().gas_limit(),
            cumulative_gas_used,
            gas_reserved,
            cumulative_blob_gas_used
        );
    }

    fn update_commit_tx_executed(
        &mut self,
        tx_with_blobs: &TransactionSignedEcRecoveredWithBlobs,
        cumulative_gas_used: u64,
        gas_reserved: u64,
        cumulative_blob_gas_used: u64,
        res: &Result<Result<TransactionOk, TransactionErr>, CriticalCommitOrderError>,
    ) {
        let hash = tx_with_blobs.hash();
        match res {
            Ok(Ok(tx_ok)) => {
                info!(
                    "Transaction executed successfully: hash={:?}, gas_used={}, cumulative_gas_used={}, gas_reserved={}, blob_gas_used={}",
                    hash,
                    tx_ok.tx_info.gas_used,
                    cumulative_gas_used,
                    gas_reserved,
                    cumulative_blob_gas_used
                );
            }
            Ok(Err(tx_err)) => {
                info!(
                    "Transaction failed: hash={:?}, error={:?}, cumulative_gas_used={}, gas_reserved={}, blob_gas_used={}",
                    hash,
                    tx_err,
                    cumulative_gas_used,
                    gas_reserved,
                    cumulative_blob_gas_used
                );
            }
            Err(critical_err) => {
                info!(
                    "Critical error executing transaction: hash={:?}, error={:?}",
                    hash,
                    critical_err
                );
            }
        }
    }
}
