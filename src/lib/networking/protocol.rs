use bevy::prelude::*;

use crate::data::chunk::BlockArray;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    RequestChunkData { chunk_pos: IVec2 }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage<I = u8> {
    Welcome { client_id: u64 },
    ChunkData {
        chunk_pos: IVec2,
        blocks: BlockArray<I>
    }
}
