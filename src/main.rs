use std::sync::Arc;

use clap::Parser;
use reth_ethereum::cli::{Cli, chainspec::EthereumChainSpecParser};
use reth_ethereum::provider::CanonStateSubscriptions;
use reth_rpc_server_types::DefaultRpcModuleValidator;

use reth_custom_db::{
    cmd::EntityCommands,
    db::{self, ensure_entity_table},
    rpc::{
        self, notifier::EntityEventNotifier, reth::RethEntityApiServer,
        sqlite::SqliteEntityApiServer,
    },
};

#[derive(Debug, Clone, clap::Args)]
struct CustomArgs {
    #[arg(long, default_value = "entity.db")]
    pub db_path: String,
}

fn main() {
    Cli::<EthereumChainSpecParser, CustomArgs, DefaultRpcModuleValidator, EntityCommands>::parse()
        .run(|builder, args| async move {
            let sqlite_db =
                db::SqliteDb::open(&args.db_path).expect("Failed to open Sqlite entity DB");

            let handle = builder
                .node(reth_ethereum::node::EthereumNode::default())
                .apply(|b| {
                    let mut reth_db = b.db().clone();
                    ensure_entity_table(&mut reth_db).expect("Failed to create Reth `EntityTable`");

                    b.extend_rpc_modules(move |ctx| {
                        let (block_event_sender, _) = tokio::sync::broadcast::channel(256);
                        rpc::block_events::forward_block_events(
                            ctx.provider().canonical_state_stream(),
                            &ctx.node().task_executor,
                            block_event_sender.clone(),
                        );

                        let notifier = Arc::new(EntityEventNotifier::new());
                        let sqlite_api = rpc::sqlite::SqliteEntityApiImpl::new(
                            sqlite_db.clone(),
                            notifier.clone(),
                        );
                        ctx.modules.merge_configured(sqlite_api.into_rpc())?;

                        let reth_api = rpc::reth::RethEntityApiImpl::new(
                            reth_db.clone(),
                            notifier.clone(),
                            block_event_sender,
                        );
                        ctx.modules.merge_configured(reth_api.into_rpc())?;

                        Ok(())
                    })
                })
                .launch_with_debug_capabilities()
                .await?;

            handle.wait_for_node_exit().await
        })
        .unwrap();
}
