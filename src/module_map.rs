use crate::pb; // Import the `pb` module
use prost::Message; // Required for decoding
use std::collections::HashMap;
use std::sync::Arc;
use lazy_static::lazy_static;
use anyhow::Error;

// Define a type alias for the decoder function
type DecoderFn = Arc<dyn Fn(&[u8]) -> Result<Box<dyn std::fmt::Debug>, Error> + Sync + Send>;

lazy_static! {
    pub static ref MODULES: HashMap<&'static str, DecoderFn> = {
        let mut map: HashMap<&'static str, DecoderFn> = HashMap::new();

        // Explicitly cast each closure to the `DecoderFn` type
        map.insert(
            "map_pools_created",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Pools::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        map.insert(
            "uni_v0_2_9:map_pools_created",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Pools::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        map.insert(
            "graph_out",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Events::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        map.insert(
            "uni_v0_2_9:graph_out",
            Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
                let decoded = pb::uniswap_types_v1::Events::decode(data)?; // Decode the Protobuf data
                Ok(Box::new(decoded))
            }) as DecoderFn,
        );
        
        // map.insert(
        //     "another_module_event",
        //     Arc::new(|data: &[u8]| -> Result<Box<dyn std::fmt::Debug>, Error> {
        //         let decoded = pb::another_types_v1::AnotherType::decode(data)?; // Decode the Protobuf data
        //         Ok(Box::new(decoded))
        //     }) as DecoderFn,
        // );

        map
    };
}
