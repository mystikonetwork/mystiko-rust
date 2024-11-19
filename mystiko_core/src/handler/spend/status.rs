use crate::{Spend, Spends, SpendsError};
use mystiko_protos::core::v1::SpendStatus;
use mystiko_storage::{Document, StatementFormatter, Storage};

impl<F, S, A, C, T, P, R, V, K> Spends<F, S, A, C, T, P, R, V, K>
where
    F: StatementFormatter,
    S: Storage,
{
    pub(crate) async fn update_status(
        &self,
        mut spend: Document<Spend>,
        status: SpendStatus,
    ) -> Result<Document<Spend>, SpendsError> {
        spend.data.status = status as i32;
        Ok(self.db.spends.update(&spend).await?)
    }
}
