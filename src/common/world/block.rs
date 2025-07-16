use crate::common::{
    error::{GameError, InvalidData},
    U32Position,
};
use bevy::ecs::{component::Component, resource::Resource};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Component, Debug, Clone, Deserialize, PartialEq, Eq, Hash)]
pub struct BlockType {
    pub namespace: String,
    pub block_name: String,
    pub block_variant: Option<String>,
}

impl BlockType {
    pub fn new(namespace: String, block_name: String, block_variant: Option<String>) -> Self {
        BlockType {
            namespace,
            block_name,
            block_variant,
        }
    }

    pub fn to_identifier(&self) -> String {
        match &self.block_variant {
            Some(variant) => format!("{}::{}::{}", self.namespace, self.block_name, variant),
            None => format!("{}::{}", self.namespace, self.block_name),
        }
    }
}

impl TryFrom<&str> for BlockType {
    type Error = GameError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split("::").collect();
        match parts.len() {
            2 => Ok(BlockType::new(
                parts[0].to_string(),
                parts[1].to_string(),
                None,
            )),
            3 => Ok(BlockType::new(
                parts[0].to_string(),
                parts[1].to_string(),
                Some(parts[2].to_string()),
            )),
            _ => Err(GameError::InvalidData(InvalidData::BlockIdentifier(
                format!("Identifier malformed: {}", value),
            ))),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockProperties {
    pub solid: bool,
    pub transparent: bool,
    pub light_emission: u8,
    pub blast_resistance: f32,
    pub hardness: f32,
    pub texture_faces: [String; 6],
    pub model: Option<String>,
    pub custom_data: HashMap<String, String>,
}

#[derive(Resource, Default, Serialize, Deserialize)]
pub struct BlockRegistry {
    block_types: HashMap<String, BlockProperties>,
}

impl BlockRegistry {
    pub fn register_block(&mut self, block_type: BlockType, properties: BlockProperties) {
        self.block_types
            .insert(block_type.to_identifier(), properties);
    }

    pub fn get_properties(&self, block_type: &BlockType) -> Option<&BlockProperties> {
        self.block_types.get(&block_type.to_identifier())
    }
}

#[derive(Component, Default)]
pub struct WorldBlock {
    pub block_type: Option<BlockType>,
    pub position: U32Position,
    pub light_level: u8,
}
