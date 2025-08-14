use std::collections::HashMap;
use binary_utils::binary::Stream;
use log::error;

pub struct Chunk {
    // R holds the (vertical) range of the Chunk. It includes both the minimum and maximum coordinates. [-64, 320)
    pub r: (isize, isize),
    // air is the runtime ID of air.
    pub air: u32,
    // recalculateHeightMap is true if the chunk's height map should be recalculated on the next call to the HeightMap
    // function.
    pub recalculate_height_map: bool,
    // heightMap is the height map of the chunk.
    pub height_map: ()/*HeightMap*/,
    // Sub holds all sub chunks part of the chunk. The pointers held by the array are nil if no sub chunk is
    // allocated at the indices.
    pub sub: HashMap<usize, SubChunk>,
    // Biomes is an array of biome IDs. There is one biome ID for every column in the chunk.
    pub biomes: Vec<PalettedStorage>
}

pub struct SubChunk {
    pub air: u32,
    pub storages: Vec<PalettedStorage>,
    pub block_light: Vec<u8>,
    pub sky_light: Vec<u8>
}

#[derive(Clone)]
pub struct PalettedStorage {
    pub bits_per_index: u16,
    pub filled_bits_per_index: u16,
    pub index_mask: u32,
    pub palette: Palette,
    pub indices: Vec<u32>
}

#[derive(Clone)]
pub struct Palette {
    pub last: u32,
    pub last_index: i16,
    pub size: u8,
    // Values is a map of values. A PalettedStorage points to the index to this value.
    pub values: Vec<u32>
}

impl PalettedStorage {

    // newPalettedStorage creates a new block storage using the uint32 slice as the indices and the palette passed.
    // The bits per block are calculated using the length of the uint32 slice.
    /*fn new(indices: Vec<u32>, palette: Palette) -> Self {
        //let bits_per_index = (indices.len() / 32 / 4) as u16;
        let bits_per_index = if indices.is_empty() { 0 } else {
            (indices.len() as f32 * 32.0 / 4096.0).ceil().log2() as u16
        };


        let index_mask = (1u32 << bits_per_index) - 1;

        let mut filled_bits_per_index = 0u16;
        if bits_per_index != 0 {
            filled_bits_per_index = 32 / bits_per_index * bits_per_index;
        }

        PalettedStorage{ bits_per_index, filled_bits_per_index, index_mask, palette, indices }
    }*/
    fn new(indices: Vec<u32>, palette: Palette) -> Self {
        // bits_per_index MUST come from palette.size (blockSize), not indices length.
        //let bits_per_index = palette.size as u16;
        let bits_per_index = (indices.len() / 32 / 4) as u16;

        let index_mask = (1u32 << bits_per_index) - 1;

        let filled_bits_per_index: u16 = if bits_per_index != 0 {
            (32usize / bits_per_index as usize * bits_per_index as usize) as u16
        } else {
            0
        };

        PalettedStorage{ bits_per_index, filled_bits_per_index, index_mask, palette, indices }
    }


    // At returns the value of the PalettedStorage at a given x, y and z.
    pub fn at(&self, x: u8, y: u8, z: u8) -> u32 {
        self.palette.value(self.palette_index(x&15, y&15, z&15))
    }

    // paletteIndex looks up the Palette index at a given x, y and z value in the PalettedStorage. This palette
    // index is not the value at this offset, but merely an index in the Palette pointing to a value.
    pub fn palette_index(&self, x: u8, y: u8, z: u8) -> u16 {
        if self.bits_per_index == 0 {
            // Unfortunately, our default logic cannot deal with 0 bits per index, meaning we'll have to special case
            // this. This comes with a little performance hit, but it seems to be the only way to go. An alternative would
            // be not to have 0 bits per block storages in memory, but that would cause a strongly increased memory usage
            // by biomes.
            return 0;
        }

        let offset = (((x as u16) << 8) | ((z as u16) << 4) | (y as u16)) * self.bits_per_index;

        let u32_offset = offset / self.filled_bits_per_index;

        let bit_offset = offset % self.filled_bits_per_index;

        let w = self.indices[u32_offset as usize];
        ((w >> bit_offset) & self.index_mask) as u16
    }
}

impl Chunk {
    pub fn new(air: u32, r: (isize, isize)) -> Self {
        Chunk{
            r,
            air,
            recalculate_height_map: false,
            height_map: (),
            sub: HashMap::new(),
            biomes: vec![]
        }
    }

    // Block returns the runtime ID of the block at a given x, y and z in a chunk at the given layer. If no
    // sub chunk exists at the given y, the block is assumed to be air.
    pub fn get_block(&self, x: u8, y: i16, z: u8, layer: u8) -> u32 {
        let sub_chunk = match self.sub.get(&(y as usize)) {
            Some(sc) => sc,
            None => return self.air,
        };

        if (sub_chunk.storages.len() as u8) <= layer {
            return self.air;
        }
        sub_chunk.storages.get(layer as usize).expect("113. line error").at(x, y as u8, z)
    }
}

impl SubChunk {
    pub fn new(air: u32) -> Self {
        SubChunk{
            air,
            storages: vec![],
            block_light: vec![],
            sky_light: vec![],
        }
    }
}

impl Palette {
    pub fn value(&self, value: u16) -> /*Option<u32>*/u32 {
        /*for (i, rid) in self.values.iter().copied().enumerate() {
            if rid == value {
                return Some(i as u32);
            }
        }
        None*/
        self.values[value as usize]
    }
}
pub fn network_decode(air: u32, data: Vec<u8>, sub_chunk_count: isize, r: (isize, isize)) -> Option<Chunk> {
    let mut chunk = Chunk::new(air, r);
    let mut buf = Stream::new(data, 0);
    let n   = (((r.1 - r.0) >> 4) + 1) as u8;

    for i in 0..sub_chunk_count {
        let mut index = i as u8;

        let sub_chunk = decode_sub_chunk(&mut buf, &chunk, &mut index);
        if sub_chunk.is_none() {
            return None;
        }

        if index > n {
            error!("index out of range");
            return None;
        }
        chunk.sub.insert(index as usize, sub_chunk.unwrap());
    }


    chunk.biomes.resize(chunk.sub.len(), PalettedStorage::new(
        vec![],
        Palette{ last: 0, last_index: 0, size: 0, values: vec![air] }
    ));

    let mut last: Option<PalettedStorage> = Option::from(PalettedStorage::new(
        vec![],
        Palette{ last: 0, last_index: 0, size: 0, values: vec![air] }
    ));

    for i in 0..chunk.sub.len() {
        let b = decode_paletted_storage(&mut buf);

        let b = if let Some(storage) = b {
            last = Some(storage.clone());
            storage
        } else {
            if i == 0 {
                error!("First biome storage pointed to previous one");
                return None;
            }
            last.clone().expect("Previous PalettedStorage should exist")
        };

        chunk.biomes[i] = b;
    }

    Some(chunk)
}

pub fn decode_sub_chunk(buf: &mut Stream, chunk: &Chunk, index: &mut u8) -> Option<SubChunk> {
    let version = buf.get_byte();

    let mut sub_chunk = SubChunk::new(chunk.air);

    match version {
        1 => {
            // Version 1 only has one layer for each sub chunk but uses the format with palettes.
            let storage = decode_paletted_storage(buf); // NetworkEncoding, BlockPaletteEncoding

            if storage.is_none() {
                return None;
            }
            sub_chunk.storages.push(storage.unwrap());

        },
        8 | 9 => {
            // Version 8 allows up to 256 layers for one sub chunk.
            let storage_count = buf.get_byte();

            if version == 9 {
                let u_index = buf.get_byte();

                // The index as written here isn't the actual index of the sub-chunk within the chunk. Rather, it is the Y
                // value of the sub-chunk. This means that we need to translate it to an index.

                let range = get_dimension_chunk_bounds(0);
                *index = ((u_index as i8) - ((range.0 >> 4) as i8)) as u8;

            }

            sub_chunk.storages.resize(storage_count as usize, PalettedStorage::new(vec![], Palette { last: 0, last_index: 0, size: 0, values: vec![chunk.air] }));
            for i in 0..storage_count {
                let storage = decode_paletted_storage(buf);
                if storage.is_none() {
                    return None;
                }
                sub_chunk.storages[i as usize] = storage.unwrap();
            }

        },
        _ => {
            return None;
        }
    }
    Some(sub_chunk)
}

pub fn decode_paletted_storage(buf: &mut Stream) -> Option<PalettedStorage> {
    let mut block_size = buf.get_byte();

    block_size >>= 1;
    if block_size == 0x7f {
        return None;
    }

    let size = block_size; // palette size
    if size > 32 {
        error!("cannot read paletted storage (size={}) : size too large", block_size);
        return None;
    }

    let u32_count = uint32s(size);
    let mut u32s = vec![0u32; u32_count];

    let byte_count = u32_count * 4;

    let data = buf.get(byte_count as u32).expect("buffer can't read 213. line error");

    if data.len() != byte_count {
        error!("cannot read paletted storage (size={}) : not enough block data present: expected {} bytes", block_size, byte_count);
        return None;
    }

    for i in 0..u32_count {
        // Explicitly don't use the binary package to greatly improve the performance of reading the uint32s
        let offset = i * 4;
        u32s[i] = (data[offset] as u32)
            | ((data[offset + 1] as u32) << 8)
            | ((data[offset + 2] as u32) << 16)
            | ((data[offset + 3] as u32) << 24);
    }


    let p = decode_palette(buf, block_size);
    if p.is_none() {
        return None;
    }

    Some(PalettedStorage::new(u32s, p.unwrap()))
}


pub fn decode_palette(buf: &mut Stream, block_size: u8) -> Option<Palette> {
    let mut palette_count = 1;
    if block_size != 0 {
        palette_count = buf.get_var_int();
        if palette_count <= 0 {
            error!("invalid palette entry count {}", palette_count);
            return None;
        }
    }

    let mut blocks = Vec::with_capacity(palette_count as usize);

    for _ in 0..palette_count {
        let temp = buf.get_var_int();
        blocks.push(temp as u32);
    }

    //Some(Palette{ last: u32::MAX, last_index: -1, size: block_size, values: blocks})
    Some(Palette{ last: 0, last_index: 0, size: block_size, values: blocks})
}

pub fn uint32s(size: u8) -> usize {
    // Mirror Dragonfly's paletteSize.uint32s logic: certain sizes need padding
    // sizes are based on bits-per-index; implement the same padded sizes as Go implementation.
    let mut u32_count = 0;
    if size != 0 {
        // indices_per_u32 is the amount of indices that may be stored in a single uint32.
        let indices_per_u32 = 32 / (size as usize);
        // u32_count is the amount of uint32s required to store all indices: 4096 indices need to be stored in
        // total.
        u32_count = 4096 / indices_per_u32;
    }
    // Dragonfly has special-cases for padded sizes (3,5,6)
    // if size == 3 || size == 5 || size == 6 then add one more uint32 in Go implementation.
    if size == 3 || size == 5 || size == 6 {
        u32_count += 1;
    }
    u32_count
}

pub fn get_dimension_chunk_bounds(dimension_id: i32) -> (isize, isize) {
    match dimension_id {
        0 => (-64, 319),  // OVER WORLD
        1 => (0, 7),    // NETHER
        2 => (0, 15),   // THE END
        _ => (0, 0),    // UNKNOWN
    }
}
