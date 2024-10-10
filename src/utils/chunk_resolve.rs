use crate::utils::chunk::chunk::Chunk;
use crate::utils::chunk::palette::PaletteSize;
use crate::utils::chunk::paletted_storage::PalettedStorage;
use crate::utils::chunk::sub_chunk::SubChunk;
use binary_utils::binary::Stream;

pub struct ChunkResolve {

}
pub fn get_dimension_chunk_bounds(dimension_id: i32) -> (isize, isize) {
    match dimension_id {
        0 => (-4, 19),  // OVER WORLD
        1 => (0, 7),    // NETHER
        2 => (0, 15),   // THE END
        _ => (0, 0),    // UNKNOWN
    }
}

pub fn network_decode(sub_chunk_count: isize, extra_payload: Vec<u8>, air: u32) -> Chunk {
    let mut chunk_stream = Stream::new(extra_payload, 0);

    let mut chunk = Chunk::new(air);

    for i in 0..sub_chunk_count {
        let mut index = i as u8;
        chunk.sub_chunks.insert(index as usize, decode_sub_chunk(&mut chunk_stream, &mut index, &chunk));

    }

    let mut last: PalettedStorage = PalettedStorage {}; /////////////////////////////////////////

    for i in 0..chunk.sub_chunks.len() {
        let mut b = decode_paletted_storage(&mut chunk_stream);

        if let Some(storage) = b {
            last = storage;
        } else {
            // b == nil means this paletted storage had the flag pointing to the previous one. It basically means we should
            // inherit whatever palette we decoded last.
            if i == 0 {
                // This should never happen and there is no way to handle this.
                eprintln!("first biome storage pointed to previous one");
            }
            b = Some(last);
        }
        chunk.biomes[i] = b.clone().expect("biomes clone error, fn name: network_decode");
    }

    chunk
}

pub fn decode_sub_chunk(chunk_stream: &mut Stream, index: &mut u8, chunk: &Chunk) -> SubChunk {
    let version = chunk_stream.get_byte();

    let mut sub_chunk = SubChunk::new(chunk.air);

    match version {
        1 => {
            // Version 1 only has one layer for each sub chunk, but uses the format with palettes.
            let storage = decode_paletted_storage(chunk_stream).unwrap();
            sub_chunk.storages.push(storage);
        },
        8 | 9 => {
            // Version 8 allows up to 256 layers for one sub chunk.
            let storage_count = chunk_stream.get_byte();

            if version == 9 {
                let u_index = chunk_stream.get_byte();
                let (min, _) = get_dimension_chunk_bounds(0);
                // The index as written here isn't the actual index of the sub-chunk within the chunk. Rather, it is the Y
                // value of the sub-chunk. This means that we need to translate it to an index.
                *index = ( (u_index as i8) - ((min >> 4) as i8) ) as u8;
            }

            for i in 0..storage_count {
                sub_chunk.storages.insert(i as usize, decode_paletted_storage(chunk_stream).unwrap());
            }

        },
        _ => {}
    }

    sub_chunk
}

pub fn decode_paletted_storage(chunk_stream: &mut Stream) -> Option<PalettedStorage> {
    let mut block_size = chunk_stream.get_byte();

    block_size >>= 1;

    if block_size == 0x7f {
        return None;
    }


    let size = PaletteSize(block_size);
    if block_size > 32 {
        eprintln!("cannot read paletted storage (size={}): size too large", block_size)
    }

    let u32_count = size.uint32s();
    let mut u32s: Vec<u32> = Vec::with_capacity(u32_count);
    let byte_count = u32_count * 4;

    let data = chunk_stream.get(byte_count as u32).expect("cannot read chunk stream data, fn name: decode_paletted_storage");

    if data.len() != byte_count {
        eprintln!("cannot read paletted storage (size={}): not enough block data present: expected {} bytes, got {}", block_size, byte_count, data.len());
    }

    for i in 0..u32_count {
        // Explicitly don't use the binary package to greatly improve performance of reading the uint32s.
        u32s.push((data[i*4] as u32) | (data[i*4+1] as u32) << 8 | (data[i*4+2] as u32) << 16 | (data[i*4+3] as u32) << 24);
    }
    /*p, err := e.decodePalette(buf, paletteSize(blockSize), pe)
    return newPalettedStorage(uint32s, p), err*/









    Option::from(PalettedStorage{})
}