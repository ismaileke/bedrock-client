use crate::utils::color_format::{COLOR_GREEN, COLOR_LIGHT_PURPLE, COLOR_WHITE};
use binary_utils::binary::Stream;
use flate2::read::GzDecoder;
use mojang_nbt::base_nbt_serializer::BaseNBTSerializer;
use mojang_nbt::big_endian_nbt_serializer::BigEndianNBTSerializer;
use mojang_nbt::tag::compound_tag::CompoundTag;
use mojang_nbt::tag::tag::Tag;
use std::any::{Any, TypeId};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;

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

    let file = File::open("src/utils/chunk/block_palette_800.nbt").unwrap();
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
            println!("{}block id:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("block_id").unwrap(), COLOR_WHITE);
            println!("{}name:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_string("name").unwrap(), COLOR_WHITE);
            println!("{}name hash:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_long("name_hash").unwrap(), COLOR_WHITE);
            println!("{}network id:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("network_id").unwrap() as i32, COLOR_WHITE);
            println!("{}version:{} {} {}", COLOR_GREEN, COLOR_LIGHT_PURPLE, compound_tag.get_int("version").unwrap(), COLOR_WHITE);
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
}
