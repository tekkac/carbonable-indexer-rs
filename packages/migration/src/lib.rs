pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20230420_072545_add_customer_tokens;
mod m20230420_075445_add_event_store;
mod m20230510_153033_add_sale_date;
mod m20230523_144915_add_forecasted_apr;
mod m20230601_135331_add_project_value;
mod m20230927_115912_add_customer_actions;
mod m20231019_074938_add_project_asset_computations;
mod m20240223_093348_add_project_metadata;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20230420_072545_add_customer_tokens::Migration),
            Box::new(m20230420_075445_add_event_store::Migration),
            Box::new(m20230510_153033_add_sale_date::Migration),
            Box::new(m20230523_144915_add_forecasted_apr::Migration),
            Box::new(m20230601_135331_add_project_value::Migration),
            Box::new(m20230927_115912_add_customer_actions::Migration),
            Box::new(m20231019_074938_add_project_asset_computations::Migration),
            Box::new(m20240223_093348_add_project_metadata::Migration),
        ]
    }
}
