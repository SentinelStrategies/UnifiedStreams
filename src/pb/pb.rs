// @generated
pub mod sf {
    pub mod firehose {
        // @@protoc_insertion_point(attribute:sf.firehose.v2)
        pub mod v2 {
            include!("sf.firehose.v2.rs");
            // @@protoc_insertion_point(sf.firehose.v2)
        }
    }
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod index {
            // @@protoc_insertion_point(attribute:sf.substreams.index.v1)
            pub mod v1 {
                include!("sf.substreams.index.v1.rs");
                // @@protoc_insertion_point(sf.substreams.index.v1)
            }
        }
        pub mod internal {
            // @@protoc_insertion_point(attribute:sf.substreams.internal.v2)
            pub mod v2 {
                include!("sf.substreams.internal.v2.rs");
                // @@protoc_insertion_point(sf.substreams.internal.v2)
            }
        }
        pub mod rpc {
            // @@protoc_insertion_point(attribute:sf.substreams.rpc.v2)
            pub mod v2 {
                include!("sf.substreams.rpc.v2.rs");
                // @@protoc_insertion_point(sf.substreams.rpc.v2)
            }
        }
        pub mod sink {
            pub mod service {
                // @@protoc_insertion_point(attribute:sf.substreams.sink.service.v1)
                pub mod v1 {
                    include!("sf.substreams.sink.service.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.sink.service.v1)
                }
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}

pub mod uniswap_types_v1 {
    include!("uniswap.types.v1.rs");  // Include the uniswap.types.v1.rs file
}

pub mod substreams_entity_change {
    include!("sf.substreams.entity.v1.rs");  // Include the uniswap.types.v1.rs file
}

pub mod solana_types_v1 {
    include!("sf.solana.type.v1.rs");
}

pub mod sol_block_v1{
    include!("sol.block.v1.rs");
}

pub mod sol_transactions_v1{
    include!("sol.transactions.v1.rs");
}

pub mod eth_block_meta_v1{
    include!("eth.block_meta.v1.rs");
}

pub mod eth_event_v1_rs{
    include!("eth.event.v1.rs");
}

pub mod eth_transaction_v1{
    include!("eth.transaction.v1.rs");
}