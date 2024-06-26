use std::fmt::Display;

use crate::domain::Ulid;
use bigdecimal::BigDecimal;
use postgres_types::{accepts, to_sql_checked, FromSql, ToSql};
use sea_query::{enum_def, Iden};
use time::{macros::offset, OffsetDateTime, PrimitiveDateTime};

use crate::domain::{crypto::U256, event_source::Event};

#[derive(Debug, ToSql, Iden)]
pub enum ErcImplementation {
    #[iden = "erc_implementation"]
    Enum,
    #[iden = "erc_721"]
    Erc721,
    #[iden = "erc_3525"]
    Erc3525,
}
impl Display for ErcImplementation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErcImplementation::Erc721 => write!(f, "erc_721"),
            ErcImplementation::Erc3525 => write!(f, "erc_3525"),
            ErcImplementation::Enum => panic!("Not a valid erc implementation"),
        }
    }
}

impl<'a> FromSql<'a> for ErcImplementation {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = std::str::from_utf8(raw)?;
        match s {
            "erc_721" => Ok(ErcImplementation::Erc721),
            "erc_3525" => Ok(ErcImplementation::Erc3525),
            _ => Err("Unrecognized enum variant".into()),
        }
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        ty.name() == "erc_implementation"
    }
}

impl From<ErcImplementation> for &str {
    fn from(value: ErcImplementation) -> &'static str {
        match value {
            ErcImplementation::Erc721 => "erc_721",
            ErcImplementation::Erc3525 => "erc_3525",
            ErcImplementation::Enum => panic!("Not a valid erc implementation"),
        }
    }
}
#[derive(Debug, ToSql, Iden)]
pub enum FarmType {
    #[iden = "farm_type"]
    Enum,
    #[iden = "yield"]
    Yield,
    #[iden = "offset"]
    Offset,
}
impl Display for FarmType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FarmType::Yield => write!(f, "yield"),
            FarmType::Offset => write!(f, "offset"),
            FarmType::Enum => panic!("Not a valid farm type"),
        }
    }
}

impl<'a> FromSql<'a> for FarmType {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = std::str::from_utf8(raw)?;
        match s {
            "yield" => Ok(FarmType::Yield),
            "offset" => Ok(FarmType::Offset),
            _ => Err("Unrecognized enum farm type variant".into()),
        }
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        ty.name() == "farm_type"
    }
}

impl From<FarmType> for &str {
    fn from(value: FarmType) -> &'static str {
        match value {
            FarmType::Yield => "yield",
            FarmType::Offset => "offset",
            FarmType::Enum => panic!("Not a valid farm type impl"),
        }
    }
}

#[derive(Debug, ToSql, Iden)]
pub enum ActionType {
    #[iden = "action_type"]
    Enum,
    #[iden = "withdraw"]
    Withdraw,
    #[iden = "deposit"]
    Deposit,
    #[iden = "claim"]
    Claim,
}

impl Display for ActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionType::Withdraw => write!(f, "withdraw"),
            ActionType::Deposit => write!(f, "deposit"),
            ActionType::Claim => write!(f, "claim"),
            ActionType::Enum => panic!("Not a valid farm type"),
        }
    }
}

impl<'a> FromSql<'a> for ActionType {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let s = std::str::from_utf8(raw)?;
        match s {
            "withdraw" => Ok(ActionType::Withdraw),
            "deposit" => Ok(ActionType::Deposit),
            "claim" => Ok(ActionType::Claim),
            _ => Err("Unrecognized enum action type variant".into()),
        }
    }

    fn accepts(ty: &postgres_types::Type) -> bool {
        ty.name() == "action_type"
    }
}

impl From<ActionType> for &str {
    fn from(value: ActionType) -> &'static str {
        match value {
            ActionType::Withdraw => "withdraw",
            ActionType::Deposit => "deposit",
            ActionType::Claim => "claim",
            ActionType::Enum => panic!("Not a valid action type impl"),
        }
    }
}

impl<'a> FromSql<'a> for Event {
    fn from_sql(
        _ty: &postgres_types::Type,
        raw: &'a [u8],
    ) -> Result<Self, Box<dyn std::error::Error + Sync + Send>> {
        let val = std::str::from_utf8(raw)?;
        Ok(Event::from(val))
    }

    accepts!(VARCHAR);
}

impl ToSql for Event {
    fn to_sql(
        &self,
        _ty: &postgres_types::Type,
        out: &mut postgres_types::private::BytesMut,
    ) -> Result<postgres_types::IsNull, Box<dyn std::error::Error + Sync + Send>>
    where
        Self: Sized,
    {
        let s: &str = self.clone().into();
        postgres_protocol::types::text_to_sql(s, out);
        Ok(postgres_types::IsNull::No)
    }

    accepts!(VARCHAR);
    to_sql_checked!();
}
impl From<Event> for sea_query::Value {
    fn from(value: Event) -> Self {
        sea_query::Value::String(Some(Box::new(String::from(<Event as Into<&str>>::into(
            value,
        )))))
    }
}

// These structs are only table definition structs
// Not domain business entities
#[enum_def]
pub struct Project {
    pub id: Ulid,
    pub address: String,
    pub slug: String,
    pub name: String,
    pub slot: Option<U256>,
    pub symbol: Option<String>,
    pub total_supply: U256,
    pub owner: String,
    pub ton_equivalent: U256,
    pub times: Vec<PrimitiveDateTime>,
    pub absorptions: Vec<U256>,
    pub setup: bool,
    pub value_decimals: U256,
    pub forecasted_apr: Option<String>,
    pub erc_implementation: ErcImplementation,
    pub implementation_id: Option<Ulid>,
    pub uri_id: Option<Ulid>,
    pub project_value: Option<U256>,
    pub slot_uri: Option<String>,
    pub migrator_address: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

impl From<tokio_postgres::Row> for Project {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            address: value.get(1),
            slug: value.get(2),
            name: value.get(3),
            slot: value.get(4),
            symbol: value.get(5),
            total_supply: value.get(6),
            owner: value.get(7),
            ton_equivalent: value.get(8),
            times: value.get(9),
            absorptions: value.get(10),
            setup: value.get(11),
            value_decimals: value.get(12),
            forecasted_apr: None,
            erc_implementation: value.get(13),
            implementation_id: None,
            uri_id: None,
            project_value: value.get(14),
            slot_uri: value.get(15),
            migrator_address: value.get(16),
            metadata: None,
        }
    }
}

#[enum_def]
pub struct Implementation {
    pub id: Ulid,
    pub address: String,
    pub abi: serde_json::Value,
}

impl From<tokio_postgres::Row> for Implementation {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            address: value.get(1),
            abi: value.get(2),
        }
    }
}

#[enum_def]
pub struct Uri {
    pub id: Ulid,
    pub uri: String,
    pub address: String,
    pub data: serde_json::Value,
}

impl From<tokio_postgres::Row> for Uri {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            uri: value.get(1),
            address: value.get(2),
            data: value.get(3),
        }
    }
}

#[enum_def]
pub struct Payment {
    pub id: Ulid,
    pub address: String,
    pub name: String,
    pub symbol: String,
    pub decimals: U256,
    pub implementation_id: Option<Ulid>,
}

impl From<tokio_postgres::Row> for Payment {
    fn from(value: tokio_postgres::Row) -> Self {
        Self {
            id: value.get(0),
            address: value.get(1),
            name: value.get(2),
            symbol: value.get(3),
            decimals: value.get(4),
            implementation_id: None,
        }
    }
}

#[enum_def]
pub struct Minter {
    pub id: Ulid,
    pub address: String,
    pub max_supply: Option<U256>,
    // Can be reserved value in case of an erc3525
    pub reserved_supply: U256,
    pub pre_sale_open: bool,
    pub public_sale_open: bool,
    pub max_buy_per_tx: Option<U256>,
    pub max_value_per_tx: Option<U256>,
    pub min_value_per_tx: Option<U256>,
    pub unit_price: BigDecimal,
    pub whitelist_merkle_root: Option<String>,
    pub sold_out: bool,
    pub total_value: Option<U256>,
    pub whitelist: Option<serde_json::Value>,
    pub erc_implementation: ErcImplementation,
    pub project_id: Option<Ulid>,
    pub payment_id: Option<Ulid>,
    pub implementation_id: Option<Ulid>,
    pub sale_date: Option<PrimitiveDateTime>,
}

#[enum_def]
pub struct Offseter {
    pub id: Ulid,
    pub address: String,
    pub total_deposited: U256,
    pub total_claimed: U256,
    pub total_claimable: U256,
    pub min_claimable: U256,
    pub project_id: Option<Ulid>,
    pub implementation_id: Option<Ulid>,
}

#[enum_def]
#[derive(Debug)]
pub struct Snapshot {
    pub id: Ulid,
    pub previous_time: OffsetDateTime,
    pub previous_project_absorption: U256,
    pub previous_offseter_absorption: U256,
    pub previous_yielder_absorption: U256,
    pub current_project_absorption: U256,
    pub current_offseter_absorption: U256,
    pub current_yielder_absorption: U256,
    pub project_absorption: U256,
    pub offseter_absorption: U256,
    pub yielder_absorption: U256,
    pub time: OffsetDateTime,
    pub yielder_id: Option<Ulid>,
}
impl From<tokio_postgres::Row> for Snapshot {
    fn from(value: tokio_postgres::Row) -> Self {
        let previous_time: PrimitiveDateTime = value.get(1);
        let time: PrimitiveDateTime = value.get(11);
        Self {
            id: value.get(0),
            previous_time: previous_time.assume_offset(offset!(+1)),
            previous_project_absorption: value.get(2),
            previous_offseter_absorption: value.get(3),
            previous_yielder_absorption: value.get(4),
            current_project_absorption: value.get(5),
            current_offseter_absorption: value.get(6),
            current_yielder_absorption: value.get(7),
            project_absorption: value.get(8),
            offseter_absorption: value.get(9),
            yielder_absorption: value.get(10),
            time: time.assume_offset(offset!(+1)),
            yielder_id: None,
        }
    }
}

#[enum_def]
pub struct Yielder {
    pub id: Ulid,
    pub address: String,
    pub total_deposited: U256,
    pub total_absorption: U256,
    pub snapshot_time: Option<PrimitiveDateTime>,
    pub project_id: Option<Ulid>,
    pub implementation_id: Option<Ulid>,
}

#[enum_def]
#[derive(Debug)]
pub struct Provision {
    pub id: Ulid,
    pub amount: U256,
    pub time: OffsetDateTime,
    pub yielder_id: Option<Ulid>,
}

impl From<tokio_postgres::Row> for Provision {
    fn from(value: tokio_postgres::Row) -> Self {
        let time: PrimitiveDateTime = value.get(2);

        Self {
            id: value.get(0),
            amount: value.get(1),
            time: time.assume_offset(offset!(+1)),
            yielder_id: None,
        }
    }
}

#[enum_def]
pub struct Transfer {
    pub id: Ulid,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub token_id: U256,
    pub time: PrimitiveDateTime,
    pub block_id: U256,
    pub project_id: Option<Ulid>,
}

#[enum_def]
pub struct Airdrop {
    pub id: Ulid,
    pub hash: String,
    pub address: String,
    pub quantity: U256,
    pub time: PrimitiveDateTime,
    pub block_id: U256,
    pub minter_id: Option<Ulid>,
}

#[enum_def]
pub struct Buy {
    pub id: Ulid,
    pub hash: String,
    pub address: String,
    pub quantity: U256,
    pub time: PrimitiveDateTime,
    pub block_id: U256,
    pub minter_id: Option<Ulid>,
}

#[enum_def]
pub struct TransferSingle {
    pub id: Ulid,
    pub hash: String,
    pub from: String,
    pub to: String,
    pub token_id: U256,
    pub time: PrimitiveDateTime,
    pub block_id: U256,
    pub badge_id: Option<Ulid>,
}

#[enum_def]
pub struct Badge {
    pub id: Ulid,
    pub address: String,
    pub name: String,
    pub owner: String,
    pub implementation_id: Option<Ulid>,
    pub uri_id: Option<Ulid>,
}

#[enum_def]
pub struct CustomerToken {
    pub id: Ulid,
    pub address: String,
    pub project_address: String,
    pub slot: Option<U256>,
    pub token_id: U256,
    pub value: Option<U256>,
    pub value_decimals: Option<U256>,
    pub erc_implementation: Option<ErcImplementation>,
    pub unit_price: Option<U256>,
    pub price_decimals: Option<U256>,
    pub price_symbol: Option<String>,
}

#[enum_def]
pub struct EventStore {
    pub id: Ulid,
    pub event_id: String,
    pub block_number: U256,
    pub block_hash: String,
    pub metadata: serde_json::Value,
    pub payload: serde_json::Value,
    pub r#type: Event,
    pub recorded_at: PrimitiveDateTime,
}

#[enum_def]
pub struct CustomerFarm {
    pub id: Ulid,
    pub customer_address: String,
    pub project_address: String,
    pub slot: U256,
    pub value: U256,
    pub farm_type: FarmType,
    pub action_type: ActionType,
    pub event_id: String,
    pub event_timestamp: PrimitiveDateTime,
}
#[enum_def]
pub struct LastStoredEvent {
    pub id: Ulid,
}
