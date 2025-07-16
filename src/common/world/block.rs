use crate::common::{
    error::{GameError, InvalidData},
    world::position::Position,
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
    pub fn new(namespace: &str, block_name: &str, block_variant: Option<String>) -> Self {
        BlockType {
            namespace: namespace.to_string(),
            block_name: block_name.to_string(),
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
                parts[0],
                parts[1],
                None,
            )),
            3 => Ok(BlockType::new(
                parts[0],
                parts[1],
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
    pub light_emission: Option<u8>,
    pub emission_color: Option<(u8, u8, u8)>,
    pub blast_resistance: f32,
    pub hardness: f32,
    pub texture_faces: [String; 6],
    pub model: Option<String>,
    pub custom_data: HashMap<String, String>,
}

#[derive(Component)]
pub struct WorldBlock {
    pub block_type: Option<BlockType>,
    pub position: Position,
    pub light_level: u8,
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

    pub fn is_solid(&self, block_type: &BlockType) -> bool {
        self.get_properties(block_type).map(|p| p.solid).unwrap()
    }

    pub fn is_transparent(&self, block_type: &BlockType) -> bool {
        self.get_properties(block_type).map(|p| p.transparent).unwrap()
    }

    pub fn get_light_emission(&self, block_type: &BlockType) -> Option<u8> {
        self.get_properties(block_type).and_then(|p| p.light_emission)
    }

    pub fn get_emission_color(&self, block_type: &BlockType) -> Option<(u8, u8, u8)> {
        self.get_properties(block_type).and_then(|p| p.emission_color)
    }
}
