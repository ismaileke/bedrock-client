/*use crate::utils::color_format::{COLOR_GREEN, COLOR_LIGHT_PURPLE, COLOR_WHITE};
use binary_utils::binary::Stream;
use flate2::read::GzDecoder;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::big_endian_nbt_serializer::BigEndianNBTSerializer;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::tag::Tag;
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::format;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use log::error;
for (key, value) in bct.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
let mut data = String::from("???? Tag");

if value.get_type() == 9 {
data = String::from("List Tag");
} else if value.get_type() == 10 {
data = String::from("Compound Tag");
}
println ! (" - {} - {}", key, data);
}


let mut custom_blocks_compound: Vec<CompoundTag> = vec![];
for (block_data, properties) in custom_blocks {
let mut cct = CompoundTag::new(HashMap::new());
let parts: Vec<&str> = block_data.split(':').collect();
cct.set_string("name".to_string(), parts.get(1).unwrap().clone().to_string());
cct.set_int("block_id".to_string(), parts.get(0).unwrap().clone().parse::<u32>().unwrap());
cct.set_long("name_hash".to_string(), Self::hash_identifier(parts.get(1).unwrap()) as i64); // ??????????????????????
cct.set_int("version".to_string(), 18168865);
cct.set_int("network_id".to_string(), 1);
for (property_name, property_enums) in properties {

} // ya aslında .nbt dosyasındaki gibi yapmamam gerek tahminimce ordaki network idler hashed ve 0 1... gidenler hashed olmayan network id sırası
}





let mut compound_tag = CompoundTag::new(HashMap::new());
let mut states = CompoundTag::new(HashMap::new());
states.set_int("trpixel_skull:bits_a".to_string(), 2);
states.set_int("trpixel_skull:bits_b".to_string(), 2);
compound_tag.set_string("name".to_string(), "trpixel:player_skull_ea2d228a34b93c51fb374fbb9543dabb40b9969e101b267f8a6a36ba7096c5d9".to_string());
compound_tag.set_tag("states".to_string(), Box::new(states));

let root = TreeRoot::new(Box::new(compound_tag), "".to_string());

let mut serializer = LittleEndianNBTSerializer::new();
let binding = serializer.write(root);
let dat = binding.as_slice();

println!("HASH: {:?}", Self::hash_identifier(dat)); // 2256357901  2256357901



let mut compound_tag: CompoundTag = CompoundTag::new(HashMap::new());
let mut states: CompoundTag = CompoundTag::new(HashMap::new());
states.set_int("candles".to_string(), 1);
states.set_byte("lit".to_string(), 0);
compound_tag.set_string("name".to_string(), "minecraft:blue_candle".to_string());
compound_tag.set_tag("states".to_string(), Box::new(states));

let root: Box<TreeRoot> = TreeRoot::new(Box::new(compound_tag), "".to_string());

let mut serializer: LittleEndianNBTSerializer = LittleEndianNBTSerializer::new();
let binding: Vec<u8> = serializer.write(root);
let dat: &[u8] = binding.as_slice();








fn cartesian_product(
    properties: &HashMap<String, Vec<Box<dyn Any>>>
) -> Vec<HashMap<String, Box<dyn Any>>> {
    let mut results = vec![];

    let mut keys = properties.keys().cloned().collect::<Vec<_>>();
    keys.sort(); // sıralı iterasyon için

    fn helper(
        keys: &[String],
        index: usize,
        properties: &HashMap<String, Vec<Box<dyn Any>>>,
        current: &mut HashMap<String, Box<dyn Any>>,
        results: &mut Vec<HashMap<String, Box<dyn Any>>>
    ) {
        if index == keys.len() {
            results.push(current.clone());
            return;
        }

        let key = &keys[index];
        if let Some(values) = properties.get(key) {
            for value in values {
                current.insert(key.clone(), value.clone());
                helper(keys, index + 1, properties, current, results);
                current.remove(key);
            }
        }
    }

    let mut current = HashMap::new();
    helper(&keys, 0, properties, &mut current, &mut results);

    results
}





fn hash_identifier(data: &[u8]) -> u32 {
    let mut hash: u32 = 0x811c9dc5;

    for &byte in data {
        hash ^= byte as u32;
        hash = hash.wrapping_add(hash << 1)
            .wrapping_add(hash << 4)
            .wrapping_add(hash << 7)
            .wrapping_add(hash << 8)
            .wrapping_add(hash << 24);
    }
    hash
}























pub struct Chunk {
    // r holds the (vertical) range of the Chunk. It includes both the minimum and maximum coordinates. [-64, 320)
    pub r: Vec<i32>,
    // air is the runtime ID of air.
    pub air: u32,
    // recalculateHeightMap is true if the chunk's height map should be recalculated on the next call to the HeightMap
    // function.
    pub recalculate_height_map: bool,
    // heightMap is the height map of the chunk.
    pub height_map: ()/*HeightMap*/,
    // sub holds all sub chunks part of the chunk. The pointers held by the array are nil if no sub chunk is
    // allocated at the indices.
    pub sub: Vec<SubChunk>,
    // biomes is an array of biome IDs. There is one biome ID for every column in the chunk.
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
    pub size: u8,
    // values is a map of values. A PalettedStorage points to the index to this value.
    pub values: Vec<u32>
}

impl PalettedStorage {

    // newPalettedStorage creates a new block storage using the uint32 slice as the indices and the palette passed.
    // The bits per block are calculated using the length of the uint32 slice.
    fn new(indices: Vec<u32>, palette: Palette) -> Self {
        let bits_per_index = (indices.len() as u16) / 32/*u32 bit size*/ / 4/*u32 byte size*/;

        let index_mask = ((1u32) << bits_per_index) - 1;
        let indices_start = indices.as_ptr();

        let mut filled_bits_per_index: u16 = 0;

        if bits_per_index != 0 {
            filled_bits_per_index =  32/*u32 bit size*/ / bits_per_index * bits_per_index;
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
        let offset = ((x as u16) << 8) | ((z as u16) << 4) | ((y as u16) * self.bits_per_index);

        let u32_offset = offset / self.filled_bits_per_index;

        let bit_offset = offset % self.filled_bits_per_index;

        let mut w: u32 = 0;
        unsafe {
            let ptr = self.indices_start.add(u32_offset as usize);
            w = *ptr;
            ((w >> bit_offset) & self.index_mask) as u16 // suspect
        }
    }
}

impl Chunk {
    pub fn new(air: u32, r: Vec<i32>) -> Self {
        Chunk{
            r,
            air,
            recalculate_height_map: false,
            height_map: (),
            sub: vec![],
            biomes: vec![]
        }
    }

    // Block returns the runtime ID of the block at a given x, y and z in a chunk at the given layer. If no
    // sub chunk exists at the given y, the block is assumed to be air.

    pub fn get_block(&self, x: u8, y: i16, z: u8, layer: u8) -> u32 {
        let sub_chunk = self.sub.get(y as usize).unwrap();
        if sub_chunk.storages.len() as u8 <= layer {
            return self.air;
        }
        sub_chunk.storages.get(layer as usize).unwrap().at(x, y as u8, z)
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
    fn new(values: Vec<u32>, size: u8) -> Self {
        Self{
            last: u32::MAX,
            last_index: -1, // Go'da atanmadığı için default -1 olabilir
            size,
            values
        }
    }

    pub fn value(&self, value: u16) -> u32 {
        self.values[value as usize]
    }
}
fn network_decode(air: u32, data: Vec<u8>, sub_chunk_count: isize, r: Vec<i32>) -> Chunk {
    let mut chunk = Chunk::new(air, r);
    let mut buf = Stream::new(data, 0);

    for i in 0..sub_chunk_count {
        let mut index = i as u8;
        chunk.sub.insert(index as usize, decode_sub_chunk(&mut buf, &chunk, &mut index));
    }

    let mut last: Option<PalettedStorage> = None;

    for i in 0..chunk.sub.len() {
        let b = decode_paletted_storage(&mut buf); // NetworkEncoding, BiomePaletteEncoding
        last = Option::from(b.clone());

        chunk.biomes.insert(i, b);
    }
    
    chunk
}

fn decode_sub_chunk(buf: &mut Stream, chunk: &Chunk, index: &mut u8) -> SubChunk {
    let version = buf.get_byte();

    let mut sub_chunk = SubChunk::new(chunk.air);

    match version {
        1 => {
            // Version 1 only has one layer for each sub chunk but uses the format with palettes.
            let storage = decode_paletted_storage(buf); // NetworkEncoding, BlockPaletteEncoding

            sub_chunk.storages.push(storage);

        },
        8 | 9 => {
            // Version 8 allows up to 256 layers for one sub chunk.
            let storage_count = buf.get_byte();

            if version == 9 {
                let u_index = buf.get_byte();

                // The index as written here isn't the actual index of the sub-chunk within the chunk. Rather, it is the Y
                // value of the sub-chunk. This means that we need to translate it to an index.

                *index = ((u_index as i8) - ((chunk.r.get(0).unwrap() >> 4) as i8)) as u8;
            }

            for i in 0..storage_count {
                sub_chunk.storages.push(decode_paletted_storage(buf));  // NetworkEncoding, BlockPaletteEncoding
            }
        },
        _ => {}
    }
    sub_chunk
}

fn decode_paletted_storage(buf: &mut Stream) -> PalettedStorage {
    let mut block_size = buf.get_byte();

    block_size = block_size >> 1;
    if block_size == 0x7f {
        error!("Block size is 0x7f");
    }

    let size = block_size; // palette size
    if size > 32 {
        error!("cannot read paletted storage (size={}) : size too large", block_size);
    }

    let u32_count = uint32s(size);

    let mut u32s = vec![];
    let byte_count = u32_count * 4;

    let data = buf.get(byte_count).expect(format!("cannot read paletted storage (size={}) : not enough block data present: expected {} bytes", block_size, byte_count).as_str());

    for i in 0..u32_count {
        // Explicitly don't use the binary package to greatly improve performance of reading the uint32s.
        u32s.insert(i as usize, data[i*4] as u32 | ((data[i*4+1]) << 8) as u32 | ((data[i*4+2]) << 16) as u32 | ((data[i*4+3]) << 24) as u32);
    }


    let p = decode_palette(buf, block_size);
    PalettedStorage::new(u32s, p)

}

fn decode_palette(buf: &mut Stream, block_size: u8) -> Palette {
    let palette_count: i32 = 1;
    if block_size != 0 {
        let palette_count = buf.get_var_int();
        if palette_count <= 0 {
            println!("invalid palette entry count {}", palette_count);
            return Palette::new(vec![], block_size);
        }
    }

    let mut blocks = Vec::with_capacity(palette_count as usize);
    let mut temp: i32 = 0;

    for i in 0..palette_count {
        temp = buf.get_var_int();
        blocks.insert(i as usize, temp as u32);
    }
    Palette::new(blocks, block_size)
}

fn uint32s(size: u8) -> u32{
    let mut u32_count = 0;
    if size != 0 {
        // indices_per_u32 is the amount of indices that may be stored in a single uint32.
        let indices_per_u32 = 32 / size as u32;
        // u32_count is the amount of uint32s required to store all indices: 4096 indices need to be stored in
        // total.
        u32_count = 4096 / indices_per_u32;
    }
    if size == 3 || size == 5 || size == 6 { // if padded
        // We've got one of the padded sizes, so the storage has another uint32 to be able to store
        // every index.
        u32_count += 1;
    }
    u32_count
}

































































#[derive(Eq, PartialEq, Clone)]
pub struct BlockType {
    pub name: String,
    pub properties: BTreeMap<String, PropertyValues>,
    pub base_runtime_id: Option<u32>,
}

impl Hash for BlockType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.base_runtime_id.hash(state);
        // `properties` alanı hash işlemine dahil edilmiyor
    }
}

impl BlockType {
    pub fn new(name: String) -> Self {
        Self{
            name,
            properties: BTreeMap::new(),
            base_runtime_id: None,
        }
    }

    pub fn variant_count(&self) -> usize {
        let mut count = 1;
        for (_, prop) in &self.properties {
            count *= prop.variant_count();
        }
        count
    }

}

pub struct BlockMapBuilder {
    blocks: HashSet<BlockType>,
    components: HashMap<TypeId, Box<dyn Fn(usize) -> Box<dyn Any>>>,
    build_functions: Vec<Box<dyn FnOnce(&mut BlockMap)>>,
}

impl BlockMapBuilder {

    pub fn new() -> Self {
        Self{ blocks: HashSet::new(), components: HashMap::new(), build_functions: vec![] }
    }
    pub fn insert_block(&mut self, b: BlockType) -> Option<BlockType> {
        self.blocks.replace(b)
    }

    pub fn build(mut self) -> BlockMap {
        self.blocks.shrink_to_fit();

        let runtime_id_count = self.blocks.iter().map(|v| v.variant_count()).sum();

        let mut variant_map = Vec::with_capacity(runtime_id_count);
        let mut block_rid_map = HashMap::with_capacity(self.blocks.len());

        let mut blocks = Vec::with_capacity(self.blocks.len());
        for block in self.blocks {
            blocks.push(block);
        }
        blocks.sort_by(|a, b| {
            let a_hash = hash_identifier(a.name.as_ref());
            let b_hash = hash_identifier(b.name.as_ref());

            a_hash.cmp(&b_hash)
        });

        let mut current_rid = 0;
        for mut block_type in blocks {
            let variant_count = block_type.variant_count();

            for i in 0..block_type.variant_count() {
                variant_map.push((block_type.name.clone(), i as u32))
            }
            block_type.base_runtime_id = Some(current_rid as u32);
            block_rid_map.insert(block_type, current_rid as u32);

            current_rid += variant_count;
        }

        let mut components = HashMap::with_capacity(self.components.len());
        for (comp_type, storage_fn) in self.components {
            components.insert(comp_type, storage_fn(runtime_id_count));
        }

        let mut block_map = BlockMap {
            blocks_types: block_rid_map,
            runtime_id_count: runtime_id_count as u32,
            variant_map,
            components,
        };
        for f in self.build_functions {
            f(&mut block_map);
        }

        block_map
    }
}

pub struct BlockMap {
    pub blocks_types: HashMap<BlockType, u32>,
    pub runtime_id_count: u32,
    pub variant_map: Vec<(String, u32)>,
    pub components: HashMap<TypeId, Box<dyn Any>>,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PropertyValues {
    pub strings: Vec<String>,
    pub bools: Vec<bool>,
    pub ints: Vec<u32>,
}

impl PropertyValues {
    pub fn variant_count(&self) -> usize {
        self.strings.len() + self.bools.len() + self.ints.len()
        // PropertyValues::Bool => 2
    }
}

fn hash_identifier(id: &str) -> u64 {
    let mut hash = 14695981039346656037_u64;
    for byte in id.as_bytes() {
        hash = hash.wrapping_mul(1099511628211_u64);
        hash = hash ^ (*byte as u64);
    }
    hash
}

pub fn vanilla_block_map(is_hashed: bool, custom_block_states: &Vec<Box<dyn Tag>>)/* -> HashMap<usize, Box<dyn Tag>>*//* -> BlockMapBuilder */{

    //const BLOCK_STATES: &[u8] = include_bytes!("block_palette_776.nbt");

    let file = File::open("resources/block_palette_827.nbt").unwrap();
    let mut decoder = GzDecoder::new(file);


    let mut contents = Vec::new();
    decoder.read_to_end(&mut contents).unwrap();
    let mut stream = Stream::new(contents, 0);

    //let mut vanilla_block_states: HashMap<String, HashMap<String, PropertyValues>> = HashMap::new();


    let mut nbt_serializer = BigEndianNBTSerializer::new();
    let mut offset = stream.get_offset();
    let nbt_root = nbt_serializer.read(stream.get_buffer(), &mut offset, 0);
    stream.set_offset(offset);

    let ct = nbt_root.must_get_compound_tag().unwrap();

    let mut blocks = ct.get_list_tag("blocks".to_string()).unwrap();
    for custom_block_state in custom_block_states {
        blocks.set(blocks.count(), custom_block_state.clone());
    }


    if is_hashed {

    } else {
        let mut name_hashes = HashMap::new();
        for i in 0..blocks.count() {
            let tag = blocks.get(i);
            let compound_tag = tag.as_any().downcast_ref::<CompoundTag>().unwrap();
            /*for (key, value) in compound_tag.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                println!("{} - {}: {}{} {}", COLOR_GREEN, key, COLOR_LIGHT_PURPLE, value.get_type(), COLOR_WHITE);
            }*/

            name_hashes.insert(compound_tag.get_long("name_hash").unwrap(), i);

            println!("------{}------", i);
            println!("{}block id:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("block_id").unwrap(), COLOR_WHITE); //
            println!("{}name:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_string("name").unwrap(), COLOR_WHITE); //
            println!("{}name hash:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_long("name_hash").unwrap(), COLOR_WHITE); //
            println!("{}network id:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("network_id").unwrap() as i32, COLOR_WHITE);
            println!("{}version:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("version").unwrap(), COLOR_WHITE); //
            let states = compound_tag.get_compound_tag("states".to_string()).unwrap();
            for (key, value) in states.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
                println!("{} - {}: {}{} {}", COLOR_GREEN, key, COLOR_LIGHT_PURPLE, value.get_type(), COLOR_WHITE);
            }
        }

        /*let mut sorted_hashes: Vec<_> = name_hashes.keys().cloned().collect();
        sorted_hashes.sort();

        for key in &sorted_hashes {
            let index = name_hashes.get(key).unwrap();
            let tag = blocks.get(*index);
            let compound_tag = tag.as_any().downcast_ref::<CompoundTag>().unwrap();

        }*/

    }

    /*for i in 0..blocks.count() {
        let tag = blocks.get(i);
        let compound_tag = tag.as_any().downcast_ref::<CompoundTag>().unwrap();
        /*for (key, value) in compound_tag.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
            println!("{} - {}: {}{} {}", COLOR_GREEN, key, COLOR_LIGHT_PURPLE, value.get_type(), COLOR_WHITE);
        }*/
        println!("------{}------", i);
        println!("{}block id:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("block_id").unwrap(), COLOR_WHITE);
        println!("{}name:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_string("name").unwrap(), COLOR_WHITE);
        println!("{}name hash:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_long("name_hash").unwrap(), COLOR_WHITE);
        println!("{}network id:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("network_id").unwrap() as i32, COLOR_WHITE);
        println!("{}version:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("version").unwrap(), COLOR_WHITE);
        let states = compound_tag.get_compound_tag("states".to_string()).unwrap();
        for (key, value) in states.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
            println!("{} - {}: {}{} {}", COLOR_GREEN, key, COLOR_LIGHT_PURPLE, value.get_type(), COLOR_WHITE);
        }
    }*/


    /*for (key, value) in ct.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
        println!("{} - {} - {} {}", COLOR_BLUE, key, value.get_type(), COLOR_WHITE);
    }*/

    /*println!("------------");
    println!("{}name: {} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, ct.get_string("name").unwrap(), COLOR_WHITE);

    println!("{}version: {} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, ct.get_int("version").unwrap(), COLOR_WHITE);
    let states = ct.get_compound_tag("states".to_string()).unwrap();
    for (key, value) in states.get_value().downcast_ref::<HashMap<String, Box<dyn Tag>>>().unwrap() {
        println!("{} - {}: {}{} {}", COLOR_GREEN, key, COLOR_LIGHT_PURPLE, value.get_type(), COLOR_WHITE);
    }*/

    /*while !nbt_stream.is_empty() {
        let nbt: tag::Compound = NBTTag::read(&mut nbt_stream, &mut NetworkNBTSerializer)
            .expect("could not decode nbt")
            .try_into()
            .unwrap();

        let name: &str = if let NBTTag::String(s) = &nbt.0["name"] {
            s.as_str()
        } else {
            panic!("Disallowed tag type for `name` field");
        };

        if !vanilla_block_states.contains_key(name) {
            vanilla_block_states.insert(Box::from(name), HashMap::new());
        }
        let property_map = vanilla_block_states.get_mut(name).unwrap();

        let states_list: tag::Compound = nbt.0["states"].clone().try_into().unwrap();
        for (name, val) in states_list.0.iter().map(|(k, v)| (k.as_str(), v)) {
            if !property_map.contains_key(name) {
                property_map.insert(
                    Box::from(name),
                    match val {
                        NBTTag::Byte(_) => PropertyValues::Bool,
                        NBTTag::Int(_) => PropertyValues::Ints(Default::default()),
                        NBTTag::String(_) => PropertyValues::Strings(Default::default()),
                        default => panic!(
                            "Disallowed tag type for property value: `{}`",
                            default.tag_type()
                        ),
                    },
                );
            }

            match property_map.get_mut(name).unwrap() {
                PropertyValues::Strings(set) => {
                    if let NBTTag::String(val) = val {
                        set.push(Box::from(val.as_str()));
                    } else {
                        panic!(
                            "Disallowed tag type for property value: `{}`",
                            val.tag_type()
                        );
                    }
                }
                PropertyValues::Ints(set) => {
                    if let NBTTag::Int(val) = val {
                        set.push(val.0);
                    } else {
                        panic!(
                            "Disallowed tag type for property value: `{}`",
                            val.tag_type()
                        );
                    }
                }
                PropertyValues::Bool => {}
            }
        }
    }

    let mut block_map = BlockMapBuilder::empty();

    for (name, properties) in vanilla_block_states {
        let mut block_type = BlockType::new(name);
        for (name, values) in properties {
            block_type.insert_property(name, values);
        }
        block_map.insert_block(block_type);
    }

    block_map*/
}*/