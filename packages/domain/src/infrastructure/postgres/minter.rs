use std::{collections::HashMap, sync::Arc};

use crypto_bigint::CheckedMul;
use deadpool_postgres::Pool;
use sea_query::{Expr, PostgresQueryBuilder, Query};
use sea_query_postgres::PostgresBinder;
use uuid::Uuid;

use crate::{
    domain::{crypto::U256, Contract, Erc3525, Erc721},
    infrastructure::starknet::model::{StarknetValue, StarknetValueResolver},
};

use super::{
    entity::{ErcImplementation, MinterIden},
    PostgresError,
};

#[derive(Debug)]
pub struct PostgresMinter<C: Contract> {
    pub db_client_pool: Arc<Pool>,
    contract: std::marker::PhantomData<C>,
}

impl<C> PostgresMinter<C>
where
    C: Contract + Send + Sync,
{
    pub fn new(db_client_pool: Arc<Pool>) -> Self {
        Self {
            db_client_pool,
            contract: std::marker::PhantomData::<C>,
        }
    }
}

impl PostgresMinter<Erc721> {
    pub async fn create(
        &self,
        address: &str,
        mut data: HashMap<String, StarknetValue>,
        project_id: Option<Uuid>,
        payment_id: Option<Uuid>,
        implementation_id: Option<Uuid>,
    ) -> Result<(), PostgresError> {
        let client = self.db_client_pool.get().await?;
        let id = uuid::Uuid::new_v4();
        let (sql, values) = Query::insert()
            .into_table(MinterIden::Table)
            .columns([
                MinterIden::Id,
                MinterIden::Address,
                MinterIden::MaxSupply,
                MinterIden::ReservedSupply,
                MinterIden::PreSaleOpen,
                MinterIden::PublicSaleOpen,
                MinterIden::MaxBuyPerTx,
                MinterIden::UnitPrice,
                MinterIden::WhitelistMerkleRoot,
                MinterIden::SoldOut,
                MinterIden::TotalValue,
                MinterIden::ErcImplementation,
                MinterIden::ProjectId,
                MinterIden::PaymentId,
                MinterIden::ImplementationId,
            ])
            .values([
                id.into(),
                address.into(),
                data.get_mut("getMaxSupplyForMint")
                    .expect("should have getMaxSupplyForMint")
                    .resolve("u256")
                    .into(),
                data.get_mut("getReservedSupplyForMint")
                    .expect("should have getReservedSupplyForMint")
                    .resolve("u256")
                    .into(),
                data.get_mut("isPreSaleOpen")
                    .expect("should have isPreSaleOpen")
                    .resolve("bool")
                    .into(),
                data.get_mut("isPublicSaleOpen")
                    .expect("should have isPublicSaleOpen")
                    .resolve("bool")
                    .into(),
                data.get_mut("getMaxBuyPerTx")
                    .expect("should have getMaxBuyPerTx")
                    .resolve("u256")
                    .into(),
                data.get_mut("getUnitPrice")
                    .expect("should have getUnitPrice")
                    .resolve("u256")
                    .into(),
                data.get_mut("getWhitelistMerkleRoot")
                    .expect("should have getWhitelistMerkleRoot")
                    .resolve("string")
                    .into(),
                data.get_mut("isSoldOut")
                    .expect("should have isSoldOut")
                    .resolve("bool")
                    .into(),
                data.get_mut("getTotalValue")
                    .expect("should have getTotalValue")
                    .resolve("u256")
                    .into(),
                Expr::val::<&str>(ErcImplementation::Erc721.into())
                    .as_enum(ErcImplementation::Enum),
                project_id.into(),
                payment_id.into(),
                implementation_id.into(),
            ])?
            .build_postgres(PostgresQueryBuilder);
        let _res = client.execute(sql.as_str(), &values.as_params()).await?;
        Ok(())
    }
}

impl PostgresMinter<Erc3525> {
    pub async fn create(
        &self,
        address: &str,
        mut data: HashMap<String, StarknetValue>,
        project_id: Option<Uuid>,
        payment_id: Option<Uuid>,
        implementation_id: Option<Uuid>,
    ) -> Result<(), PostgresError> {
        let client = self.db_client_pool.get().await?;
        let id = uuid::Uuid::new_v4();
        let unit_price: crypto_bigint::U256 = data
            .get_mut("getUnitPrice")
            .expect("should have getUnitPrice")
            .resolve("u256")
            .into();
        let max_value: crypto_bigint::U256 = data
            .get_mut("getMaxValue")
            .expect("should have getMaxValue")
            .resolve("u256")
            .into();
        let total_value = unit_price.checked_mul(&max_value).unwrap();
        let (sql, values) = Query::insert()
            .into_table(MinterIden::Table)
            .columns([
                MinterIden::Id,
                MinterIden::Address,
                // Act as reserved value there
                MinterIden::ReservedSupply,
                MinterIden::PreSaleOpen,
                MinterIden::PublicSaleOpen,
                MinterIden::MaxValuePerTx,
                MinterIden::MinValuePerTx,
                MinterIden::UnitPrice,
                MinterIden::TotalValue,
                MinterIden::WhitelistMerkleRoot,
                MinterIden::SoldOut,
                MinterIden::ErcImplementation,
                MinterIden::ProjectId,
                MinterIden::PaymentId,
                MinterIden::ImplementationId,
            ])
            .values([
                id.into(),
                address.into(),
                data.get_mut("getReservedValue")
                    .expect("should have getReservedValue")
                    .resolve("u256")
                    .into(),
                data.get_mut("isPreSaleOpen")
                    .expect("should have isPreSaleOpen")
                    .resolve("bool")
                    .into(),
                data.get_mut("isPublicSaleOpen")
                    .expect("should have isPublicSaleOpen")
                    .resolve("bool")
                    .into(),
                data.get_mut("getMaxValuePerTx")
                    .expect("should have getMaxBuyPerTx")
                    .resolve("u256")
                    .into(),
                data.get_mut("getMinValuePerTx")
                    .expect("should have getMaxBuyPerTx")
                    .resolve("u256")
                    .into(),
                U256::from(unit_price).into(),
                U256::from(total_value).into(),
                data.get_mut("getWhitelistMerkleRoot")
                    .expect("should have getWhitelistMerkleRoot")
                    .resolve("u256")
                    .into(),
                data.get_mut("isSoldOut")
                    .expect("should have isSoldOut")
                    .resolve("bool")
                    .into(),
                Expr::val::<&str>(ErcImplementation::Erc3525.into())
                    .as_enum(ErcImplementation::Enum),
                project_id.into(),
                payment_id.into(),
                implementation_id.into(),
            ])?
            .build_postgres(PostgresQueryBuilder);
        let _res = client.execute(sql.as_str(), &values.as_params()).await?;
        Ok(())
    }
}
