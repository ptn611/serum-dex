use borsh::schema::BorshSchema;
use borsh::BorshSerialize;

use serum_pool_schema::PoolRequest;
use serum_pool_schema::PoolState;

fn main() -> std::io::Result<()> {
    let mut schema: borsh::schema::BorshSchemaContainer = PoolState::schema_container();
    PoolRequest::add_definitions_recursively(&mut schema.definitions);
    schema.serialize(&mut std::io::stdout())
}
