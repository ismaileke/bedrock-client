use std::collections::HashMap;
use binary_utils::binary::Stream;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::little_endian_nbt_serializer::LittleEndianNBTSerializer;
use mojang_nbt::tag::byte_tag::ByteTag;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::int_tag::IntTag;
use mojang_nbt::tag::string_tag::StringTag;
use mojang_nbt::tag::tag::Tag;
use mojang_nbt::tree_root::TreeRoot;
use crate::client;
use crate::client::{hash_identifier, PropertyValue};

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
    pub palette: Palette,
    pub indices: Vec<u32>
}

#[derive(Clone)]
pub struct Palette {
    // Values is a map of values. A PalettedStorage points to the index to this value.
    pub values: Vec<u32>
}

impl PalettedStorage {

    fn new(indices: Vec<u32>, palette: Palette) -> Self {
        let bits_per_index = (indices.len() / 32 / 4) as u16;

        let index_mask = (1u32 << bits_per_index) - 1;

        let mut filled_bits_per_index: u16 = 0;
        if bits_per_index != 0 {
            filled_bits_per_index = 32 / bits_per_index * bits_per_index;
        }

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
            sub: vec![],
            biomes: vec![]
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
    pub fn value(&self, value: u16) -> u32 {
        self.values[value as usize]
    }
}

pub fn network_decode(air: u32, data: Vec<u8>, sub_chunk_count: u32, range: (isize, isize)) -> Chunk {
    let mut chunk = Chunk::new(air, range);
    let mut buf = Stream::new(data, 0);
    let n = (((range.1 - range.0) >> 4) + 1) as u8;

    for _ in 0..n {
        //chunk.sub.push(SubChunk::new(air));
        chunk.sub.push(SubChunk{air, storages: vec![PalettedStorage::new(vec![], Palette{ values: vec![air] })], block_light: vec![], sky_light: vec![] });
    }

    for i in 0..sub_chunk_count {
        let mut index = i as u8;

        let sub_chunk = decode_sub_chunk(&mut buf, &chunk, &mut index);

        if index > n {
            panic!("index out of range");
        }
        chunk.sub[index as usize] = sub_chunk;
    }

    /*chunk.biomes.resize(chunk.sub.len(), PalettedStorage::new(vec![], Palette{ values: vec![air] }
    ));

    let mut last: PalettedStorage = PalettedStorage::new(vec![], Palette{ values: vec![air] });

    for i in 0..chunk.sub.len() {
        let b = decode_paletted_storage(&mut buf);

        let b = if let storage = b {
            last = storage.clone();
            storage
        } else {
            if i == 0 {
                panic!("First biome storage pointed to previous one");
            }
            last.clone()
        };

        chunk.biomes[i] = b;
    }*/

    chunk
}

pub fn decode_sub_chunk(buf: &mut Stream, chunk: &Chunk, index: &mut u8) -> SubChunk {
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
                let y_index = buf.get_byte();

                *index = (y_index as i8 - (chunk.r.0 >> 4) as i8) as u8;
            }

            /*for _ in 0..storage_count {
                let storage = PalettedStorage::new(vec![], Palette{ values: vec![air] });
                sub_chunk.storages.push(storage);
            }*/

            for _ in 0..storage_count {
                let storage = decode_paletted_storage(buf);
                sub_chunk.storages.push(storage);
            }
        }
        _ => {}
    }
    sub_chunk
}

pub fn decode_paletted_storage(buf: &mut Stream) -> PalettedStorage {
    let (bits_per_index, nbt_palette) = {
        let temp = buf.get_byte();
        (temp >> 1, temp & 1 != 1)
    };


    let index_u32_count = uint32s(bits_per_index);
    let mut u32s = Vec::<u32>::with_capacity(index_u32_count as usize);

    let byte_count = index_u32_count as usize * 4;

    let data = buf.get(byte_count as u32).expect("buffer can't read 213. line error");

    if data.len() != byte_count {
        panic!("cannot read paletted storage (size={}) : not enough block data present: expected {} bytes", index_u32_count, byte_count);
    }

    for i in 0..index_u32_count as usize {
        // Explicitly don't use the binary package to greatly improve the performance of reading the uint32s
        let offset = i * 4;
        u32s.push((data[offset] as u32)
            | ((data[offset + 1] as u32) << 8)
            | ((data[offset + 2] as u32) << 16)
            | ((data[offset + 3] as u32) << 24));
    }


    /*let index_u32_count: i32 = uint32s(bits_per_index);

    let mut u32s = Vec::new();
    for _ in 0..index_u32_count {
        let u32_data = buf.get_l_int();
        u32s.push(u32_data);                                                                               // or l_int
    }*/



    /*let mut palette_size = 1;
    if bits_per_index != 0 {
        palette_size = buf.get_var_int();
        if palette_size <= 0 {
            panic!("invalid palette entry count {}", palette_size);
        }
    }*/

    let mut palette_size = 1;
    if bits_per_index != 0 {
        palette_size = buf.get_var_int() as usize;
    }

    let mut palette = Vec::<u32>::with_capacity(palette_size);
    if !nbt_palette {
        // In most cases, the palette is just encoded as a vector of `var_i32`s.
        for _ in 0..palette_size {
            let data = buf.get_var_int();
            palette.push(data as u32);
        }
    } else {
        // The palette can be encoded with nbt. In this case, each entry is a compound tag with
        // the namespaced block id and the block state.
        for _ in 0..palette_size {
            let mut offset = buf.get_offset();
            let mut serializer = LittleEndianNBTSerializer::new();
            let root = serializer.read(buf.get_buffer(), &mut offset, 0);
            buf.set_offset(offset);
            let ct = root.must_get_compound_tag().unwrap();
            let name = ct.get_string("name").unwrap();
            let states = ct.get_compound_tag("states".to_string()).unwrap();
            let properties = states.get_list_tag("properties".to_string());

            let mut properties_map = HashMap::new();

            if properties.is_some() {
                properties.unwrap().get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap().iter().for_each(|property| {
                    let mut property_enums_map: Vec<PropertyValue> = vec![];

                    let pct = property.as_any().downcast_ref::<CompoundTag>().unwrap();
                    let property_name = pct.get_string("name").unwrap();
                    let property_enums = pct.get_list_tag("enum".to_string()).unwrap();
                    // Blok Özellikleri ve Alabileceği Değerler
                    //println!("property name: {}", property_name);
                    property_enums.get_value().downcast_ref::<Vec<Box<dyn Tag>>>().unwrap().iter().for_each(|property_enum| {
                        let id = property_enum.as_any().type_id();
                        if id == std::any::TypeId::of::<IntTag>() {
                            let pce = property_enum.as_any().downcast_ref::<IntTag>().unwrap().clone();
                            let any_value = pce.get_value();
                            let value = any_value.downcast_ref::<u32>().unwrap();
                            property_enums_map.push(PropertyValue::Int(value.clone()));
                        } else if id == std::any::TypeId::of::<StringTag>() {
                            let pce = property_enum.as_any().downcast_ref::<StringTag>().unwrap().clone();
                            let any_value = pce.get_value();
                            let value = any_value.downcast_ref::<String>().unwrap();
                            property_enums_map.push(PropertyValue::Str(value.clone()));
                        } else if id == std::any::TypeId::of::<ByteTag>() {
                            let pce = property_enum.as_any().downcast_ref::<ByteTag>().unwrap().clone();
                            let any_value = pce.get_value();
                            let value = any_value.downcast_ref::<u8>().unwrap();
                            property_enums_map.push(PropertyValue::Byte(value.clone()));
                        } else {
                            println!("Undefined Tag Type");
                        }
                    });
                    properties_map.insert(property_name, property_enums_map);

                });
            }

            let combinations = client::cartesian_product_enum(&properties_map);
            for combo in combinations {
                let mut state = CompoundTag::new(HashMap::new());
                for (k, v) in &combo {
                    match v {
                        PropertyValue::Int(i) => {
                            state.set_int(k.clone(), *i);
                        },
                        PropertyValue::Str(s) => {
                            state.set_string(k.clone(), s.clone());
                        },
                        PropertyValue::Byte(b) => {
                            state.set_byte(k.clone(), *b as i8);
                        }
                    }
                }

                let mut custom_ct = CompoundTag::new(HashMap::new());
                custom_ct.set_string("name".to_string(), name.clone());
                custom_ct.set_tag("states".to_string(), Box::new(state.clone()));

                let root = TreeRoot::new(Box::new(custom_ct.clone()), "".to_string());
                let mut serializer = LittleEndianNBTSerializer::new();
                let binding = serializer.write(root);
                let data = binding.as_slice();

                palette.push(hash_identifier(data));
            }
        }
    }

    PalettedStorage::new(u32s, Palette{ values: palette })
}

pub fn uint32s(bits_per_index: u8) -> i32 {
    let mut index_u32_count: i32 = 0;
    if bits_per_index != 0 {

        let indices_per_u32 = 32 / bits_per_index as i32;

        index_u32_count = 4096 / indices_per_u32;
    }

    if bits_per_index == 3 || bits_per_index == 5 || bits_per_index == 6 {
        index_u32_count += 1;
    }
    index_u32_count
}

pub fn get_dimension_chunk_bounds(dimension_id: i32) -> (isize, isize) {
    match dimension_id {
        0 => (-64, 319),  // OVER WORLD
        1 => (0, 112),    // NETHER
        2 => (0, 240),   // THE END
        _ => (0, 0),    // UNKNOWN
    }
}
