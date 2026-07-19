use reth_ethereum::provider::db::{
    TableSet,
    table::{Table, TableInfo},
};

use super::{EntityKey, EntityValue};

#[derive(Debug, Default, Copy, Clone)]
pub struct EntityTable;

impl Table for EntityTable {
    const NAME: &'static str = "EntityTable";
    const DUPSORT: bool = false;

    type Key = EntityKey;
    type Value = EntityValue;
}

#[derive(Debug)]
pub(super) struct EntityTableInfo;

impl TableInfo for EntityTableInfo {
    fn name(&self) -> &'static str {
        EntityTable::NAME
    }
    fn is_dupsort(&self) -> bool {
        EntityTable::DUPSORT
    }
}

pub(super) struct EntityTableSet;

impl TableSet for EntityTableSet {
    fn tables() -> Box<dyn Iterator<Item = Box<dyn TableInfo>>> {
        let info: Box<dyn TableInfo> = Box::new(EntityTableInfo);
        Box::new(std::iter::once(info))
    }
}
