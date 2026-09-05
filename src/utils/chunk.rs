use std::collections::HashMap;
use binary_utils::binary::Reader;
use linked_hash_map::LinkedHashMap;
use mojang_nbt::nbt_serializer::{NBTReader, NBTWriter};
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use crate::utils::block::fnv1_64;

#[derive(Clone)]
pub struct PaletteSize(pub u8);
impl PaletteSize {
    pub fn uint32s(self) -> isize {
        let mut index_u32_count: isize = 0;
        if self.0 != 0 {
            let indices_per_u32 = 32 / self.0 as isize;
            index_u32_count = 4096 / indices_per_u32;
        }
        if self.0 == 3 || self.0 == 5 || self.0 == 6 {
            index_u32_count += 1;
        }
        index_u32_count
    }
}

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
    pub sub: Vec<SubChunk>,
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
    pub indices_start: *const u32,
    pub palette: Palette,
    pub indices: Vec<u32>
}

#[derive(Clone)]
pub struct Palette {
    pub last: u32,
    pub last_index: i16,
    pub size: PaletteSize,
    // Values is a map of values. A PalettedStorage points to the index to this value.
    pub values: Vec<u32>
}

#[derive(Clone)]
pub struct BlockRegistry {
    pub air_id: u32,
    // NBT verisinin FNV1-64 hash'i -> Runtime/Hashed ID
    pub nbt_to_id: HashMap<u64, u32>,
}

impl PalettedStorage {
    fn new(indices: Vec<u32>, palette: Palette) -> Self {
        let bits_per_index = (indices.len() / 32 / 4) as u16;

        let index_mask = (1u32 << bits_per_index) - 1;

        let indices_start = indices.as_ptr();

        let mut filled_bits_per_index: u16 = 0;
        if bits_per_index != 0 {
            filled_bits_per_index = 32 / bits_per_index * bits_per_index;
        }

        PalettedStorage{ bits_per_index, filled_bits_per_index, index_mask, indices_start, palette, indices }
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

        let w = unsafe { *self.indices_start.add(u32_offset as usize) };
        ((w >> bit_offset) & self.index_mask) as u16
    }
}

impl Chunk {
    pub fn new(air: u32, r: (isize, isize)) -> Self {
        let n = (((r.1-r.0) >> 4) + 1) as usize;
        let mut sub = Vec::<SubChunk>::with_capacity(n);
        let mut biomes = Vec::<PalettedStorage>::with_capacity(n);

        for _ in 0..n {
            sub.push(SubChunk::new(air));
            biomes.push(PalettedStorage::new(vec![], Palette::new(PaletteSize(0), vec![0])));// maybe vec![air]?
        }

        Chunk{
            r,
            air,
            recalculate_height_map: true,
            height_map: (),
            sub,
            biomes
        }
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

    pub fn new(size: PaletteSize, values: Vec<u32>) -> Self {
        Palette{size, values, last: u32::MAX, last_index: 0 }
    }

    pub fn value(&self, value: u16) -> u32 {
        self.values[value as usize]
    }
}

pub fn network_decode(registry: &BlockRegistry, data: &[u8], sub_chunk_count: u32, range: (isize, isize)) -> Result<Chunk, String> {
    let mut chunk = Chunk::new(registry.air_id, range);
    let mut buf = Reader::new(data);
    let n = (((range.1 - range.0) >> 4) + 1) as u8;

    for i in 0..sub_chunk_count {
        let mut index = i as u8;

        let sub_chunk = decode_sub_chunk(&mut buf, registry, &chunk, &mut index)?;

        if index > n {
            return Err("index out of range".to_string());
        }
        chunk.sub[index as usize] = sub_chunk;
    }

    // Biome processing - following Go's logic
    /*let mut last: Option<&PalettedStorage> = None;

    for i in 0..chunk.sub.len() {
        let b = decode_paletted_storage(&mut buf)?;

        let b = if let Some(storage) = b {
            last = Some(&storage);
            storage
        } else {
            if i == 0 {
                return Err("First biome storage pointed to previous one".to_string());
            }
            *(last.unwrap())
        };

        chunk.biomes[i] = b;
    }*/

    Ok(chunk)
}

pub fn decode_sub_chunk(buf: &mut Reader, registry: &BlockRegistry, chunk: &Chunk, index: &mut u8) -> Result<SubChunk, String> {
    let version = buf.get_u8();

    let mut sub_chunk = SubChunk::new(chunk.air);

    match version {
        1 => {
            // Version 1 only has one layer for each sub chunk but uses the format with palettes.
            let storage = decode_paletted_storage(buf, registry)?; // NetworkEncoding, BlockPaletteEncoding

            if let Some(s) = storage {
                sub_chunk.storages.push(s);
            }
        },
        8 | 9 => {
            // Version 8 allows up to 256 layers for one sub chunk.
            let storage_count = buf.get_u8();

            if version == 9 {
                let y_index = buf.get_u8();

                *index = (y_index as i8 - (chunk.r.0 >> 4) as i8) as u8;
            }

            sub_chunk.storages = Vec::<PalettedStorage>::with_capacity(storage_count as usize);
            for i in 0..storage_count {
                let storage = decode_paletted_storage(buf, registry)?;
                if let Some(s) = storage {
                    sub_chunk.storages.insert(i as usize, s);
                }
            }
        }
        _ => {
            return Err(format!("Unknown Sub Chunk version {}: Can't decode", version));
        }
    }
    Ok(sub_chunk)
}

pub fn decode_paletted_storage(buf: &mut Reader, registry: &BlockRegistry) -> Result<Option<PalettedStorage>, String> {
    let raw_block_size = buf.get_u8();

    let block_size = raw_block_size >> 1;

    // DiskDecode IMPORTANT
    let allow_persistent_ids = true; // true data palette, false runtime data palette
    let is_runtime = (raw_block_size & 1) != 0 || !allow_persistent_ids;
    if block_size == 0x7f {
        return Ok(None); // Go returns nil here
    }

    let size = PaletteSize(block_size);
    if size.0 > 32 {
        return Err(format!("Cannot read paletted storage (size={}): size too large", block_size));
    }

    let uin32_count = size.uint32s();

    let mut uint32s = Vec::<u32>::with_capacity(uin32_count as usize);
    let byte_count = uin32_count * 4;

    let data = buf.get(byte_count as usize);

    if data.len() != byte_count as usize {
        return Err(format!("Cannot read paletted storage (size={}): not enough block data present: expected {} bytes, got {}", block_size, byte_count, data.len()));
    }

    for i in 0..uin32_count as usize {
        uint32s.insert(i, u32::from_le_bytes([data[i*4], data[i*4+1], data[i*4+2], data[i*4+3]]));
    }

    // Disk/Network Decode
    let palette = if is_runtime {
        //println!("NetworkDecode:");
        decode_palette_network(buf, PaletteSize(block_size))?
    } else {
        //println!("DiskDecode:");
        decode_palette_disk(buf, PaletteSize(block_size), registry)?
    };

    Ok(Some(PalettedStorage::new(uint32s, palette)))
}

pub fn decode_palette_network(buf: &mut Reader, palette_size: PaletteSize) -> Result<Palette, String> {
    let mut palette_count: i32 = 1;
    if palette_size.0 != 0 {
        palette_count = buf.get_var_i32();
        if palette_count <= 0 {
            return Err(format!("Invalid palette entry count {}", palette_count));
        }
    }

    let mut blocks = Vec::<u32>::with_capacity(palette_count as usize);
    for _ in 0..palette_count {
        let temp = buf.get_var_i32();
        blocks.push(temp as u32);
    }
    Ok(Palette{
        last: 0,
        last_index: 0,
        size: palette_size,
        values: blocks,
    })
}

pub fn decode_palette_disk(buf: &mut Reader, palette_size: PaletteSize, registry: &BlockRegistry) -> Result<Palette, String> {
    let mut palette_count: i32 = 1;
    if palette_size.0 != 0 {
        palette_count = buf.get_var_i32();
    }

    let mut palette = Palette::new(palette_size, vec![0u32; palette_count as usize]);
    for i in 0..palette_count {
        palette.values[i as usize] = decode_block_palette(buf, registry)?;
    }
    if palette_count == 0 {
        return Err("invalid palette entry count: found 0, but palette with 0 bits per block must have at least 1 value".to_string());
    }
    Ok(palette)
}

pub fn decode_block_palette(buf: &mut Reader, registry: &BlockRegistry) -> Result<u32, String> {
    let mut offset = buf.offset();
    let mut nbt_serializer = NBTReader::new_network();
    let nbt_root = nbt_serializer.read(buf.get_buffer(), &mut offset, 0);
    buf.set_offset(offset);

    // CompoundTag { value: {"name": String(StringTag { value: "air" }), "version": Int(IntTag { value: 18166272 }), "states": CompoundTag { value: {} } })
    let compound_tag = nbt_root.must_get_compound_tag().expect("Decode Block Palette TreeRoot to CompoundTag conversion error");

    let name = compound_tag.get_string("name").unwrap();
    //let version = compound_tag.get_int("version").unwrap_or(17_694_724); // LOL
    let mut state = compound_tag.get_compound_tag("states".to_string());

    /*if version < 17_694_723 {
        // This entry is a pre-1.13 block state,
        // so decode the meta value instead.
        /*let meta = compound_tag.get_short("val").unwrap_or(0);

        // Upgrade the pre-1.13 state into a post-1.13 state.
        let legacy = upgrade_legacy_entry(&name, meta).ok_or_else(|| {
            format!("cannot find mapping for legacy block entry: {}, {}", name, meta)
        })?;

        // Update the name, state, and version.
        name = legacy.name;
        state = Some(legacy.state);
        version = legacy.version;*/

    } else */if state.is_none() {
        // The state is a post-1.13 block state,
        // but the states field is missing.
        state = Some(CompoundTag::new(LinkedHashMap::new()));
    }

    let state = state.ok_or_else(|| { "invalid state in block entry".to_string() })?;

    /*
    // Upgrade the block state if necessary.
    let upgraded = blockupgrader::upgrade(BlockState {
        name,
        properties: state,
        version,
    });


    let runtime_id = self
        .blocks
        .state_to_runtime_id(
            &upgraded.name,
            &upgraded.properties,
        )
        .ok_or_else(|| {
            format!(
                "cannot get runtime ID of block state {}{:?} {}",
                upgraded.name,
                upgraded.properties,
                upgraded.version
            )
        })?;
    */

    let mut custom_ct = CompoundTag::new(LinkedHashMap::new());
    custom_ct.set_string("name", name.clone());
    custom_ct.set_tag("states", Tag::Compound(state));

    let root = TreeRoot::new(Tag::Compound(custom_ct.clone()), "");
    let mut writer = NBTWriter::new_little_endian();
    let data = writer.write(root);

    // Hash'i hesapla (64-bit kullanmak çakışmaları önler)
    let state_hash = fnv1_64(data);

    // Registry'den gerçek Runtime ID'yi bul
    if let Some(&runtime_id) = registry.nbt_to_id.get(&state_hash) {
        Ok(runtime_id)
    } else {
        println!("Unknown block: {}", name);
        Ok(registry.air_id)
    }
}

pub fn get_dimension_chunk_bounds(dimension_id: i32) -> (isize, isize) {
    match dimension_id {
        0 => (-64, 319),  // OVER WORLD
        1 => (0, 112),    // NETHER
        2 => (0, 240),   // THE END
        _ => (0, 0),    // UNKNOWN
    }
}