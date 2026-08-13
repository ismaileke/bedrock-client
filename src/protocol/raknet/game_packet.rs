use std::error::Error;
use binary_utils::binary::{Reader, Writer};
use libdeflater::{CompressionLvl, Compressor};
use miniz_oxide::inflate::{decompress_slice_iter_to_slice, TINFLStatus};
use snap::raw::{max_compress_len, Decoder, Encoder};
use crate::protocol::bedrock::network_settings::{SNAPPY, ZLIB};
use crate::protocol::bedrock::packet::Packet;
use crate::protocol::raknet::packet_ids::PacketType;
use crate::utils::encryption::Encryption;

pub struct GamePacket {
    pub encryption: Option<Encryption>,
    pub compress_enabled: bool,
    pub compression_type: u8
}

impl GamePacket {
    pub fn new(encryption: Option<Encryption>, compress_enabled: bool, compression_type: u8) -> GamePacket {
        GamePacket { encryption, compress_enabled, compression_type }
    }

    pub fn encode(&mut self, pk: &mut dyn Packet, out: &mut Writer) -> Result<(), Box<dyn Error>> {
        // encrypt/not [ 0xfe + compress/not [ packet_total_length + [ packet_id + packet_content ] ] ] (iirc)
        let mut pk_encoded = Writer::new();

        pk_encoded.put_var_u32(pk.id() as u32);
        pk.encode(&mut pk_encoded);

        let mut payload = Writer::new();
        payload.put_var_u32(pk_encoded.len() as u32);
        payload.put(pk_encoded.as_slice());


        out.clear();
        out.put_u8(PacketType::Game.get_u8());

        if self.compress_enabled {
            match self.compression_type {
                ZLIB   => Self::compress_zlib(payload.as_slice(), out),
                SNAPPY => Self::compress_snappy(payload.as_slice(), out),
                _      => out.put(payload.as_slice()),
            }
        } else {
            out.put(payload.as_slice());
        }

        if let Some(ref mut enc) = self.encryption {
            enc.encrypt_in_place(out, 1)?;
        }

        Ok(())
    }

    pub fn decode<'a>(&mut self, payload: &'a mut [u8], scratch: &'a mut [u8]) -> Result<Reader<'a>, Box<dyn Error>> {
        let data: &'a [u8] = match self.encryption {
            Some(ref mut e) => e.decrypt(payload)?,
            None => payload,
        };

        if !self.compress_enabled {
            return Ok(Reader::new(data));
        }

        let (&kind, body) = data.split_first().ok_or("empty packet")?;

        let out: &'a [u8] = match kind {
            ZLIB   => Self::decompress_zlib(body, scratch),
            SNAPPY => Self::decompress_snappy(body, scratch),
            _      => body,
        };

        Ok(Reader::new(out))
    }

    /*pub fn encrypt(&mut self, payload: &Vec<u8>) -> Vec<u8> {
        let mut main_stream = Stream::new(vec![0xfe], 0);
        let compressed = GamePacket::compress(payload);
        let encrypted = self.encryption.encrypt(&compressed).expect("GamePacket Encrypt Error");
        main_stream.put(encrypted);
        main_stream.get_buffer()
    }*/

    pub fn decrypt<'a>(&mut self, payload: &'a mut [u8]) -> &'a [u8] {
        if let Some(ref mut encryption) = self.encryption {
            return encryption.decrypt(payload).expect("Decrypt Error GamePacket");
        }
        payload
    }

    pub fn compress_zlib<'a>(payload: &[u8], out: &mut Writer) {
        let level = if payload.len() >= 256 { 7 } else { 0 };
        let mut compressor = Compressor::new(CompressionLvl::new(level).expect("Invalid level"));

        out.put_u8(0x00); // ZLIB
        let start = out.len();
        out.resize(start + payload.len() + 64, 0);

        let n = compressor.deflate_compress(payload, &mut out.as_mut_slice()[start..]).expect("Compression failed");
        out.truncate(start + n);
    }

    pub fn compress_snappy<'a>(payload: &[u8], out: &mut Writer) {
        out.put_u8(0x01); // SNAPPY
        let start = out.len();
        out.resize(start + max_compress_len(payload.len()), 0);

        let n = Encoder::new().compress(payload, &mut out.as_mut_slice()[start..])
            .expect("Snappy Compress Error");
        out.truncate(start + n);
    }

    pub fn decompress_zlib<'a>(payload: &[u8], out: &'a mut [u8]) -> &'a[u8] {
        Self::decompress(payload, out).expect("ZLIB Decompress Error")
    }

    pub fn decompress_snappy<'a>(payload: &[u8], out: &'a mut [u8]) -> &'a [u8] {
        let n = Decoder::new().decompress(payload, out).expect("Snappy Decompress Error");
        &out[..n]
    }

    fn decompress<'a>(payload: &[u8], out: &'a mut [u8]) -> Result<&'a [u8], TINFLStatus> {
        let n = decompress_slice_iter_to_slice(out, std::iter::once(payload), false, true)?;
        Ok(&out[..n])
    }
}
