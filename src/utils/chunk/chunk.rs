use crate::utils::chunk::height_map::HeightMap;
use crate::utils::chunk::paletted_storage::PalettedStorage;
use crate::utils::chunk::sub_chunk::SubChunk;
use crate::utils::chunk_resolve;

pub struct Chunk {
    // r holds the (vertical) range of the Chunk. It includes both the minimum and maximum coordinates.
    // air is the runtime ID of air.
    pub air: u32,
    // recalculateHeightMap is true if the chunk's height map should be recalculated on the next call to the HeightMap
    // function.
    pub recalculate_height_map: bool,
    // heightMap is the height map of the chunk.
    pub height_map: HeightMap,
    // sub holds all sub chunks part of the chunk. The pointers held by the array are nil if no sub chunk is
    // allocated at the indices.
    pub sub_chunks: Vec<SubChunk>,
    // biomes is an array of biome IDs. There is one biome ID for every column in the chunk.
    pub biomes: Vec<PalettedStorage>
}

impl Chunk {
    pub fn new(air: u32) -> Chunk {
        let (max, min) = chunk_resolve::get_dimension_chunk_bounds(0);
        let n =  (((max - min) >> 4) + 1) as usize;

        let mut sub_chunks: Vec<SubChunk> = Vec::with_capacity(n);
        let mut biomes: Vec<PalettedStorage> = Vec::with_capacity(n);

        for _ in 0..n {
            //sub_chunks.push(NewSubChunk(air));
            //biomes.push(emptyStorage(0));
        }

        /*for i := 0; i < n; i++ {
            sub[i] = NewSubChunk(air)
            biomes[i] = emptyStorage(0)
        }*/
        Chunk{
            air,
            recalculate_height_map: true,
            height_map: HeightMap {}, // edit again
            sub_chunks,
            biomes,
        }
    }

}