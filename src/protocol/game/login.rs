use crate::protocol::game::bedrock_packet_ids::BedrockPacketType;
use crate::protocol::game_packet;
use crate::utils::encryption::Encryption;
use binary_utils::binary::Stream;
use chrono::Utc;
use openssl::ecdsa::EcdsaSig;
use openssl::pkey::{PKey, Private};
use openssl::sign::Signer;
use serde_json::{json, to_vec, Value};

pub struct Login {
    client_protocol: u32,
    chain_data_jwt: String,
    client_data_jwt: String
}

pub fn new(client_protocol: u32, chain_data_jwt: String, client_data_jwt: String) -> Login {
    Login{ client_protocol, chain_data_jwt, client_data_jwt }
}

impl Login {
    pub fn encode(&self) -> Vec<u8> {
        let mut stream = Stream::new(Vec::new(), 0);
        stream.put_unsigned_var_int(BedrockPacketType::get_byte(BedrockPacketType::Login) as u32);

        stream.put_int(self.client_protocol);

        let mut jwt_stream = Stream::new(Vec::new(), 0);
        jwt_stream.put_l_int(self.chain_data_jwt.len() as u32);
        jwt_stream.put(self.chain_data_jwt.clone().into_bytes());
        jwt_stream.put_l_int(self.client_data_jwt.len() as u32);
        jwt_stream.put(self.client_data_jwt.clone().into_bytes());

        stream.put_unsigned_var_int(jwt_stream.get_buffer().len() as u32);
        stream.put(jwt_stream.get_buffer());

        let mut main_stream = Stream::new(vec![0xfe], 0);

        let mut compress_stream = Stream::new(Vec::new(), 0);
        compress_stream.put_unsigned_var_int(stream.get_buffer().len() as u32);
        compress_stream.put(stream.get_buffer());

        main_stream.put(game_packet::compress(&compress_stream.get_buffer()));

        main_stream.get_buffer()
    }
}

pub fn convert_login_chain(chain: &mut Vec<String>, pkey: PKey<Private>) -> Vec<String> {
    let chain_one: Vec<&str> = chain[0].split('.').collect();
    let chain_two: Vec<&str> = chain[1].split('.').collect();

    let chain_encoded = Encryption::b64_url_decode(chain_one[0]).unwrap();
    let chain_decoded: Value = serde_json::from_str(chain_encoded.as_str()).expect("Chain 1 can not decoded.");

    let chain_two_encoded = Encryption::b64_url_decode(chain_two[1]).unwrap();
    let chain_two_decoded: Value = serde_json::from_str(chain_two_encoded.as_str()).expect("Chain 2 can not decoded.");

    let identity_pub_key = chain_two_decoded.get("identityPublicKey").and_then(Value::as_str).unwrap().to_string();
    let extra_data = chain_two_decoded.get("extraData").unwrap();
    let display_name = extra_data.get("displayName").and_then(Value::as_str).unwrap().to_string();

    let x5u = chain_decoded.get("x5u").and_then(Value::as_str).unwrap().to_string();

    let header = json!({
        "alg": "ES384",
        "x5u": identity_pub_key
    });

    let current_time = Utc::now().timestamp();
    let payload = json!({
        "identityPublicKey": x5u,
        "exp": current_time + 21600,
        "nbf": current_time - 21600,
        "certificateAuthority": true
    });

    let address = format!("[{}]:{}", "127.0.0.1".to_string(), 19132);

    let payload_two = json!({
        "SelfSignedId": "5a3a3c37-7ddd-3cf2-bcce-a98dfa15b703", // edit
        "ServerAddress": address, // edit
        "ClientRandomId": 12345, // edit
        "SkinId": "ea8d8c0d-4e2a-49ec-9715-9adbb0625427.Skin2", // edit
        "SkinData": "fVMZ/31TGf//+qD///qg///Zdf/41wr/+NcK//jXCv/41wr///qg///6oP//+qD///qg///6oP//+qD/+NcK//5/D//+fw//9rEU//rPPf/6zz3/9rEU//5/D//+fw///n8P//rPPf/2sRT//n8P//5/D//+fw//7oAU/+6AFP////8A////APGeUgDnzGoA58xqAPGeUgD///8A////AP///wD///8A/94k///6oP//+qD//94k/////wD///8A////AP///wD///8A////AP///wD///8A792LAO/diwDl0XcA5dF3AOXRdwDl0XcA////AP///wD///8A////AH1TGf99Uxn///qg///6oP//2XX/75c2/++XNv/vlzb/+NcK///6oP/+oxv//qMb//6jG//+oxv///qg//jXCv/ycxj//qMb//axFP/6zz3/+s89//axFP/+oxv/8nMY//uJDv/6zz3/9rEU//6jG//7iQ7/+4kO/+6AFP/ugBT/////AP///wDxnlIA58xqAOfMagDxnlIA////AP///wD///8A////APrDHP//+qD///qg//rDHP////8A////AP///wD///8A////AP///wD///8A////AP///wDv3YsA5dF3AOXRdwDp23r/6dt6/////wD///8A////AP///wB9Uxn/fVMZ///eJP//3iT/+sMc/+leIv/pXiL/6V4i//rPPf/65m3/+4kO///6oP//+qD/+4kO//rmbf/6zz3//qMb/+vDY//6zz3/++Oy//vjsv/6zz3/68Nj//6jG//mpA//++Oy//rPPf/rw2P//qMb/+akD//ugBT/7oAU/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD6wxz/9OaO//Tmjv/6wxz/////AP///wD///8A////AP///wD///8A////AP///wD///8A792LAO/diwDv3YsA9OaO//Tmjv////8A////AP///wD///8AfVMZ/31TGf//3iT//94k//rDHP/pXiL/6V4i/+leIv/+oxv/+4kO//uJDv/6zz3/+s89//uJDv/7iQ7//qMb//rDHP/6wxz///5K//vjsv/747L///5K//rDHP/6wxz/5qQP//vjsv///kr/+sMc//rDHP/mpA//7oAU/+6AFP////8A////ANSNIQDUjSEA////AP///wD///8A1I0hAP///wD///8A+sMc//Tmjv/05o7/+sMc/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A792LAO/diwDv3YsA////AP///wD///8A////AH1TGf99Uxn//94k//rDHP/6wxz/+pEL//qRC//6kQv/+pEL//rPPf/6zz3/+uZt//rmbf/6zz3/+s89//qRC//+oxv//qMb//6jG//6zz3/+s89//6jG//+oxv//qMb/9l0HP/6zz3//qMb//6jG//+oxv/2XQc/+6AFP/ugBT/////AP///wDUjSEA1I0hANSNIQD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AO/diwDv3YsA792LAP///wD///8A////AP///wB9Uxn/fVMZ///eJP/6wxz/+sMc//6jG//+oxv//qMb//6jG//6zz3/+s89//rmbf/65m3/+s89//rPPf/+oxv/68Nj//rPPf/6zz3/++Oy//vjsv//3iT/+4kO/+vDY//apXb/++Oy///eJP/7iQ7/68Nj/9qldv/ugBT/7oAU/////wD///8A/pwtAP6cLQD+nC0A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A792LAO/diwD///8A////AP///wD///8Ae1JC/3tSQv99Uxn/fVMZ///6oP/65m3/+4kO//rPPf/6zz3//qMb//6jG//6zz3/+s89//6jG//+oxv/+s89/+GDDv/uhg7//qMb//6jG//4yBr//qMb/+6GDv/hgw7/2qV2//jIGv//3iT//qMb//rPPf/apXb/7oAU/+6AFP////8A////AP6cLQD+nC0A/pwtAP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AO/diwDv3YsA////AP///wD///8A////AHlLGv95Sxr//9l1//jXCv/41wr/+NcK//jXCv//3iT///Kk//vjsv/747L/++Oy//vjsv/747L/++Oy///ypP/khgn/+4kO//OAH//tdyH/7Xch//OAH//7iQ7/5IYJ/9qldv/khgn/5IYJ//uJDv/+oxv/2qV2/+6AFP/ugBT/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD65m3/+uZt/////wD///8A////AP///wD/+qD///qg///6oP//3iT//94k//rDHP/6wxz//94k//jXCv/747L/+NcK//vjsv/747L/+NcK//vjsv/41wr/+NcK//rDHP/6wxz//NAg//zQIP//+qD///qg///6oP//+qD/+uZt//6jG//65m3/+uZt//6jG//65m3///qg/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////APrmbf/65m3/////AP///wD///8A//qg///Zdf//2XX/+sMc//rDHP/6wxz/+sMc//rDHP/3pRL///Kk//jXCv/+oxv//qMb//jXCv//8qT/96US//elEv/6wxz/+sMc//rDHP/6wxz//9l1///Zdf//+qD///qg//rmbf//rC7//qMa//6jG///rC7/+uZt///6oP////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD65m3/+uZt/////wD///8A////AP/eJP/41wr/75c2/++XNv/vlzb/+pEL//6jG//7iQ7/+4kO///ypP/41wr//qMb//6jG//41wr///Kk//uJDv/pXiL//qMb//qRC//vlzb/75c2/++XNv/41wr//94k///eJP/65m3//8t8//6jG//+oxv//8t8//rmbf//3iT/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A//4m///+Jv////8A////AP///wD/3iT/+sMc/++XNv/pXiL/6V4i//qRC//+oxv//94k///eJP//8qT///Kk//6jG//+oxv///Kk///ypP//3iT//94k//6jG//6kQv/6V4i/+leIv/vlzb/+sMc///eJP//3iT/+uZt//+sLv/+oxv//qMb//+sLv/65m3//94k/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////APrPPf/6zz3/////AP///wD///8A+sMc//rDHP/vlzb/6V4i/+leIv/6kQv/+4kO///eJP9luOH/CiQ6///ypP/6wxz/+sMc///ypP8KJDr/Zbjh///eJP/7iQ7/+pEL/+leIv/pXiL/75c2//rDHP/6wxz//94k//rmbf//rC7//qMb//6jG///rC7/+uZt///eJP////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD65m3/+uZt/////wD///8A////APStKf/0rSn/75c2/+leIv/pXiL/+pEL//uJDv//3iT/0ev4/xhDcf//3iT/68Nj/+vDY///3iT/GENx/9Hr+P//3iT/+4kO//qRC//pXiL/6V4i/++XNv/0rSn/9K0p//e6Lf/61ET/7Nhf//rmbf/+oxv//8t8//rURP/3ui3/75c2AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A//4m///+Jv////8A////AP///wD0rSn/9K0p//uJDv/pXiL/+4kO//uJDv/+oxv//94k//jIGv/+oxv//qMb/+GDDv/hgw7//qMb//6jG//4yBr//94k//6jG//7iQ7/+4kO/+leIv/7iQ7/9K0p//StKf/0rSn/9K0p/+i7Fv/5zCf/+cwn/+i7Fv/0rSn/9K0p//StKQD///8A////AP///wD///8A////AP///wD///8A////AP///wA8gHT/PIB0/02Ufv9NlH7/////AP///wD///8A////AP///wD///8A////AP///wD///8A4bY8AP///wD///8A////APrPPf/6zz3/////AP///wD///8A+4kO//JzGP/ycxj/6V4i/+leIv/pXiL/+4kO/+SGCf/khgn/6V4i/+leIv/pXiL/6V4i/+leIv/pXiL/5IYJ/+SGCf/7iQ7/6V4i/+leIv/pXiL/8nMY//JzGP/7iQ7/+4kO//uJDv/ouxb/+cwn//nMJ//ouxb/+4kO//uJDv/0rSkA+cwnAP///wD///8A////AP///wD///8A////AP///wD///8APIB0/0VZVv9FWVb/TZR+/////wD///8A////AP///wD///8A////AP///wD///8A////ANynEwD///8A////AP///wD/ig7//4oO/////wD///8A////APStKQDpXiIA6V4iAOleIgD///8A////AP///wD///8A////AP///wDluj3/5bo9/////wD///8A////AP///wDv3YsA792LAO/diwDv3YsA/qMb//6jG//+oxv//qMb//6jG//+oxv//qMb//iePv/+oxv/33oR/++LA//qsxD/6rMQ/+qzEP/viwP/33oR/+S5agDv3YsA792LAO/diwBJUkn/SVJJ/0lSSf/2+NP/t6u///b40//2+NP//68A//+vAP//rwD//68A/////wBPT0//T09P/09PT/9PT0//HiY1AB4mNQAeJjUAT09PAE9PTwD///8A////AP///wD///8A6V4iAOleIgDpXiIA////AP///wD///8A////AP///wD///8A++Oy//vjsv////8A////AP///wD///8A792LAO/diwDv3YsA792LAPrDHP//z2P/+4kO//uJDv/7iQ7/+4kO///PY//6wxz/+sMc//rcov/5xC7/+cQu//nELv/5xC7/+tyi//m2Ff/kuWoA792LAO/diwDv3YsASVJJ/0lSSf9JUkn/9vjT/7erv//2+NP/9vjT//+DAP//gwD//68A//+vAP////8AT09P/09PT/9PT0//T09P/x4mNQAeJjUAHiY1AE9PTwBPT08A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AN2uhP/kqBz/////AP///wD///8A////AO/diwDv3YsA792LAO/diwD+oxv//qMb///PY///z2P//89j///PY//+oxv//qMb//axFP/mtk7/5rZO//nELv/5xC7/5rZO/+a2Tv/0oA//5LlqAO/diwDv3YsA/qMb/0lSSf9JUkn/SVJJ/5yNbv+cjW7/nI1u/5yNbv/8ygL///5K///+Sv///kr/////AE9PT/9PT0//T09P/09PT/9PT08AQ1NQ/0NTUP9DU1AAT09PAP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDnzGr/58xq/////wD///8A////AP///wDv3YsA792LAO/diwDv3YsA+4kO//6jG//+oxv//qMb//6jG//+oxv//qMb//uJDv/+oxv//pAU//6QFP/5xC7/+cQu//6QFP/+kBT//pAU/+S5agDv3YsA/qMb//6jGwDh4eP/4eHj/+Hh4/+3q7//4eHj/+Hh4/+3q7//+dpG///+Sv/8ygL//MoC/7erv//h4eP/t6u//7erv//h4eP/t6u//0NTUP9DU1D/Q1NQAE9PTwD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A+s89//rPPf/+oxv/+4kO//uJDv/7iQ7//qMb//6jG//+oxv//qMb//uJDv/7iQ7/+4kO//6jG//6zz3/+s89//rPPf/6zz3//qMb//uJDv/6zz3/+s89//rPPf/6zz3/f3do/1xSP/9cUj//t6u//+Hh4//h4eP/t6u//5yNbv9cUj//nI1u/5yNbv+3q7//4eHj/7erv/+3q7//4eHj/7erv/9DU1AAQ1NQAFRjZv9UY2YA////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////APrPPf/6zz3//9l1//uJDv/+fw///n8P//uJDv/+oxv//qMb//uJDv/+fw///n8P//uJDv//2XX/+s89//rPPf/6zz3/+s89///Zdf/7iQ7/+s89//rPPf/6zz3/+s89/08YP///3iT//94k/08YP//Oi0f/zotH/69SJv+EOgv/hDoL/4Q6C/+EOgv/r1Im/86LR//Oi0f/r1Im/86LR//Oi0f/Q1NQAFRjZv9UY2b/VGNm/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD+oxv/+4kO//5/D//+fw///n8P//5/D//2sRT/+s89//rPPf/2sRT//n8P//5/D//+fw///n8P//uJDv/+oxv//89j/+6AFP/+oxv//qMb//6jG//+oxv//qMb//6jG/9PGD///OG0//zhtP9PGD//hDoL/4Q6C/9PGD//Txg//08YP/9PGD//Txg//08YP/+EOgv/hDoL/08YP/+EOgv/hDoL/09PTwBUY2b/VGNm/1RjZgD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A/qMb//uJDv/7iQ7/+4kO//uJDv/+oxv/9rEU//rPPf/6zz3/9rEU//6jG//7iQ7/+4kO//uJDv/7iQ7//qMb//vjsv/ugBT//qMb//6jG//+oxv//qMb//6jG//+oxv/XFI//1xSP/9cUj//XFI//393aP9/d2j/XFI//1xSP/9cUj//XFI//1xSP/9cUj//f3do/393aP9cUj//f3do/393aP9PT08AVGNmAFRjZgBUY2YA////AP///wD///8A////AP///wD///8A////AP///wD///8AoVoN/6FaDf+hWg3/o4Is/6OCLP+jgiz/5bo9/7hhCP////8A////ANl0HP/ZdBz/2XQc/+akD//+oxv/68Nj//rPPf/747L/++Oy//rPPf/rw2P//qMb/+akD//ZdBz/2XQc/+a5J//muSf/5rkn/+a5J//muSf/5rkn/+a5J//muSf/5rkn/393aP/bzc3/7eHv/393aP/bzc3/7eHv/5yNbv9/d2j/nI1u/5yNbv9cUj//nI1u/9vNzf/t4e//nI1u/9vNzf/t4e//T09PAFRjZgBUY2YAVGNmAP///wD///8A////AP///wD///8A////AP///wD///8A////AKNkEP+jZBD/oVoN/6OCLP+YTwf/oncU/+KAEv+4YQj/////AP///wDkdgT/5HYE/+R2BP/mpA//+sMc//rDHP///kr/++Oy//vjsv///kr/+sMc//rDHP/mpA//5HYE/+R2BP/kdgT/5HYE/+R2BP/mpA//2qV2/+akD//kdgT/5HYE/+R2BP9/d2j/vrva/393aP9/d2j/vrva/393aP+cjW7/XFI//1xSP/+cjW7/XFI//5yNbv++u9r/f3do/5yNbv++u9r/f3do/09PTwBPT08AT09PAE9PTwD///8A////AP///wD///8A////AP///wD///8A////AP///wChWg3/oVoN/6FaDf+YTwf/aDsV/6J3FP/kqBz/uGEI/////wD///8A2XQc/9l0HP/ZdBz/2XQc//6jG//+oxv//qMb//rPPf/6zz3//qMb//6jG//+oxv/2XQc/9l0HP/ZdBz/5Y8X/+WPF//yngT/7sEW/+7BFv/uwRb/8p4E/+WPF//ljxf/f3do/7672v+3q7//f3do/7672v+3q7//nI1u/393aP+cjW7/nI1u/1xSP/+cjW7/vrva/7erv/+cjW7/vrva/7erv/9PT08AT09PAE9PTwBPT08A////AP///wD///8A////AP///wD///8A////AP///wD///8Av5Zy/8ewW//FkBj/v5Zy/35IGf9oOxX/5Kgc/8NuEP////8A////AOa5J//yngT/8p4E/9qldv/rw2P/+s89//rPPf/747L/++Oy//rPPf/6zz3/68Nj/9qldv/yngT/5rkn/++5Nv/myFX/7st8/+7Xov/u16L/7tei/+7LfP/vuTb/77k2/4Q6C/+EOgv/Txg//08YP/+EOgv/hDoL/4Q6C/9PGD//Txg//08YP/9PGD//hDoL/08YP/9PGD//hDoL/08YP/9PGD//T09PAE9PTwBPT08AT09PAP///wD///8A////AP///wD///8A////AP///wD///8A////AL+Wcv/HsFv/xZAY/8NuEP+bVRb/gEYS/+fMav/DbhD/////AP///wDeawX/3msF/95rBf/apXb/+s89///+Sv/747L/++Oy//vjsv/747L///5K//rPPf/apXb/3msF/95rBf/kdgT/5HYE/+R2BP/mpA//2qV2/+akD//kdgT/5HYE/+R2BP/52kb/+dpG//+6CP//ugj/+dpG//naRv/luj3//5AA//+QAP//sQn/+dpG//naRv/52kb//7oI//naRv/52kb//7oI/09PTwBPT08AT09PAE9PTwD///8A////AP///wD///8A////AP///wD///8A////AP///wDnzGr/58xq/+KAEv/igBL/3a6E/+SoHP/igBL/4oAS/////wD///8A2XQc/95rBf/eawX/2qV2//6jG//+oxv//qMb//rPPf/6zz3//qMb//6jG//+oxv/2qV2/95rBf/ZdBz/5Y8X/+WPF//yngT/7sEW/+7BFv/uwRb/8p4E/+WPF//ljxf/+dpG//naRv//ugj//7oI//+cAP/52kb//58B//92AP//dgD//3YA//naRv/52kb//7EJ//+6CP/52kb//7EJ//+6CP9PT08AT09PAE9PTwBPT08A////AP///wD///8A////AP///wD///8A////AP///wD///8A5Kgc/92uhP/luj3/5bo9/+fMav/kqBz/58xq/+W6Pf////8A////APKeBP/yngT/8p4E/+WPF//2sRT/68Nj/+vDY//6zz3/+s89/+vDY//rw2P/9rEU/+WPF//yngT/8p4E//KeBP/muSf/7st8/+7Xov/u16L/7tei/+7LfP/muSf/8p4E///+Sv///kr/+dpG//naRv//gQD/0n8B/6p0Av+qdAL/qnQC//92AP/luj3//6gE//+xCf//ugj//6gE//+xCf//ugj/T09PAE9PTwBPT08AT09PAP///wD///8A////AP///wD///8A////AP///wD///8A////AOfMav/nzGr/5bo9/+W6Pf/nzGr/5Kgc/+fMav/luj3/////AP///wDZdBz/2XQc/9l0HP/ljxf/+sMc//vjsv/6zz3/+s89//rPPf/6zz3/++Oy//rDHP/ljxf/2XQc/9l0HP/kdgT/5HYE/+R2BP/mpA//2qV2/+akD//kdgT/5HYE/+R2BP///kr/+dpG///+Sv/52kb/+dpG///+Sv//dgD//5AA//+QAP//qAT/+dpG//+oBP//sQn//7oI//+oBP//sQn//7oI/09PTwBPT08AT09PAE9PTwD///8A////AP///wD///8A////AP///wD///8A////AP///wDVcAn/5rkn/////wD///8A////AP///wDmuSf/5rkn/////wD///8A2XQcAN5rBQDeawUA////AP///wD///8A9OaO//Tmjv/05o7/9OaO/////wChXxUAoV8VAN5rBQD///8A////AP///wD///8A////AP///wD6zz3/68Nj/////wD///8A////AMpoTgDKaE4AymhOAMpoTgDKaE4AvUgyAP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A1XAJ/+R2BP////8A////AP///wD///8A5qQP/+R2BP////8A////APKeBADyngQA8p4EAP///wD///8A////APTmjv//+qD///qg//Tmjv////8A792LAP///wD///8A////AP///wD///8A////AP///wD///8A+uZt/+vDY/////8A////AP///wDKaE4AymhOAMpoTgDKaE4AymhOAL1IMgD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////ANVwCf/dbgL/////AP///wD///8A////AO7BFv/yngT/////AP///wDZdBwA2XQcANl0HAD///8A////APrPPf/05o7///qg///6oP/65m3/+s89//rPPQD///8A////AP///wD///8A////AP///wD///8A////APrPPf/rw2P/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDVcAn/3W4C/////wD///8A////AP///wDu16L/7st8/////wD///8A////AP///wD///8A////AP///wD///8A68Nj/+vDY//6zz3/+s89//rPPQD6zz0A////AP///wD///8A////AP///wD///8A////AP///wD/ig7//4oO/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDMkjz/zJI8/95rBf/eawX/5HYE/95rBf/apXb/5qQP/+R2BP////8A////AP///wD///8A////AP///wD///8A////APTmjgD/+qAA//qgAPrmbQD6zz0A+s89AP///wD///8A////AP///wD///8A////AOvDY//65m3///4m/+vDY/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A1XAJ/9VwCf/yngT/2XQc/+WPF//eawX/2qV2/+7BFv/yngT/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDrw2P/+s89//rPPf/rw2P/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AMRMEv/ZdBz/8p4E//KeBP/muSf/8p4E/+WPF//u16L/7st8/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A/4oO//+KDv//ig7//4oO/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDETBL/xEwS/9l0HP/ZdBz/5HYE/9l0HP/ljxf/5qQP/+R2BP////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AOvDY//65m3///4m/+vDY/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////ANl0HADZdBwA////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDrw2P/+s89//rPPf/rw2P/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A+s89//rPPf/65m3/+uZtAP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A1rfIAP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDrw2MA68NjAP///wD///8A////APrmbQD///8A////APrmbQD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A/dhlAP///wD///8A////AP///wD///8A////AP///wD92GUA////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A5rkn/9l0HP/mpA//1XAJ/+a5J//muSf/////AP///wD///8A////AP///wD///8A////AP///wDmuSf/5rkn/////wD///8A////AP///wD///8A////AP///wD///8A5bo9/+W6Pf+9ORoAz1crAP///wD///8A////AP///wBJUkn/SVJJ/0lSSf9JUkn/9vjT//b40/+3q7///68A//+vAP//rwD//68A//+vAP9PT0//T09P/09PT/9PT0//////AP///wD///8A////AP///wD///8A////AP///wD///8A////AHaKqAB2iqgAdoqoAP///wD///8A////AOR2BP/kdgT/5qQP/9VwCf/kdgT/5HYE/////wD///8A////AP///wD///8A////AP///wD///8A5HYE/+akD/////8A////AP///wD///8A////AP///wD///8A////APvjsv/747L/////AP///wD///8A////AP///wD///8ASVJJ/0lSSf9JUkn/SVJJ//b40//2+NP/t6u///+DAP//gwD//68A//+vAP///kr/T09P/09PT/9PT0//T09P/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wB2iqgAdoqoAHaKqAD///8A////AP///wDljxf/2XQc/9l0HP/VcAn/3W4C/9VwCf////8A////AP///wD///8A////AP///wD///8A////APKeBP/uwRb/////AP///wD///8A////AP///wD///8A////AP///wDkqBz/3a6E/388DwD///8A////AP///wD///8A////AElSSf9JUkn/SVJJ/0lSSf+cjW7/nI1u/5yNbv/8ygL///5K///+Sv///kr//MoC/09PT/9PT0//T09P/09PT/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8AdoqoAHaKqAB2iqgA////AP///wD///8A5shV//KeBP/apXb/1XAJ/91uAv/VcAn/////AP///wD///8A////AP///wD///8A////AP///wDuy3z/7tei/////wD///8A////AP///wD///8A////AP///wD///8A58xq/+fMav////8A////AP///wD///8A////AP///wC3q7//XFI//1xSP/+3q7//t6u//+Hh4//h4eP/+dpG///+Sv/8ygL//MoC/7erv//h4eP/t6u//7erv/9cUj//////AP///wD///8A////AP///wD///8A////AP///wD///8A////AHaKqAB2iqgAdoqoAP///wD///8A////AOR2BP/eawX/2qV2/95rBf/eawX/zJI8/8ySPP////8A////AP///wD///8A////AP///wD///8A5HYE/+akD/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8AnI1u/5yNbv9cUj//nI1u/7erv//h4eP/4eHj/7erv/9cUj//XFI//393aP+3q7//4eHj/7erv/+3q7//XFI//////wD///8A////AP///wD///8A////AP///wD///8A////AP///wB2iqgAdoqoAHaKqAD///8A////AP///wDljxf/3msF/9qldv/ZdBz/8p4E/9VwCf/VcAn/////AP///wD///8A////AP///wD///8A////APKeBP/uwRb/////AP///wD///8A////AC4uNQAuLjUA////AC4uNQD///8A////AP///wD///8A////AP///wD///8A////AIQ6C/+EOgv/hDoL/4Q6C/+vUib/zotH/86LR/9PGD///94k///eJP9PGD//zotH/86LR/+vUib/r1Im/0M5J/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AHaKqAB2iqgA////AP///wD///8A5rkn//KeBP/ljxf/8p4E//KeBP/ZdBz/xEwS/////wD///8A////AP///wD///8A////AP///wDuy3z/7tei/z8/PwA/Pz8APz8/AD8/PwApKjcALi41AC4uNQApKjcA////AP///wD///8A////AD8/PwA/Pz8A////AP///wBPGD//Txg//08YP/9PGD//Txg//4Q6C/+EOgv/Txg///zhtP/84bT/Txg//4Q6C/+EOgv/Txg//08YP/9cUj//////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wB2iqgAdoqoAP///wD///8A////AOR2BP/ZdBz/5Y8X/9l0HP/ZdBz/xEwS/8RMEv////8A////AP///wD///8A////AP///wD///8A5HYE/+akD/8pKjcAKSo3ACkqNwApKjcAKSo3AHMaDQAuLjUAKSo3AP///wD///8A////AP///wApKjcALi41AP///wD///8AXFI//1xSP/9cUj//XFI//1xSP/9/d2j/f3do/1xSP/9cUj//XFI//1xSP/9/d2j/f3do/1xSP/9cUj//Qzkn/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8AdoqoAHaKqAD///8A////AP///wD///8A////AP///wD///8A////ANl0HADZdBwA////AP///wD///8A////AP///wD///8A////AP///wD///8Ao4Is/6OCLP+jgiz/oVoN/6FaDf+hWg3/////AP///wD///8A////AP///wD///8A////AP///wC4YQj/5bo9/1xSP/+cjW7/nI1u/393aP+cjW7/7eHv/9vNzf9/d2j/7eHv/9vNzf9/d2j/7eHv/9vNzf+cjW7/nI1u/1xSP/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wB2iqgA////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AKJ3FP+YTwf/o4Is/6FaDf+jZBD/o2QQ/////wD///8A////AP///wD///8A////AP///wD///8AuGEI/+KAEv9cUj//nI1u/1xSP/9cUj//nI1u/393aP++u9r/f3do/393aP++u9r/f3do/393aP++u9r/nI1u/5yNbv9cUj//////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8AdoqoAP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD747IA1rfIAOjH2QD///8A////AP///wD///8A////AP///wCidxT/aDsV/5hPB/+hWg3/oVoN/6FaDf////8A////AP///wD///8A////AP///wD///8A////ALhhCP/kqBz/XFI//5yNbv+cjW7/f3do/5yNbv+3q7//vrva/393aP+3q7//vrva/393aP+3q7//vrva/5yNbv+cjW7/XFI//////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A1rfIAP///wD///8A////AP///wD///8A////AP///wD///8AaDsV/35IGf+/lnL/xZAY/7+Wcv/HsFv/////AP///wD///8A////AP///wD///8A////AP///wDDbhD/5Kgc/08YP/9PGD//Txg//08YP/+EOgv/hDoL/4Q6C/9PGD//Txg//4Q6C/+EOgv/Txg//08YP/+EOgv/hDoL/08YP/////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AIBGEv+bVRb/w24Q/8WQGP+/lnL/x7Bb/////wD///8A////AP///wD///8A////AP///wD///8Aw24Q/+fMav/52kb//7EJ//+QAP//kAD/5bo9//naRv/52kb//7oI//+6CP/52kb/+dpG//+6CP/52kb/+dpG//naRv//kAD/////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wDkqBz/3a6E/+KAEv/igBL/58xq/+fMav////8A////AP///wD///8A////AP///wD///8A////AOKAEv/igBL/+dpG//92AP//dgD//3YA//+fAf/52kb//5wA//+6CP//ugj/+dpG//naRv//ugj//7EJ//naRv//dgD//3YA/////wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A5Kgc/+fMav/luj3/5bo9/+SoHP/droT/////AP///wD///8A////AP///wD///8A////AP///wDluj3/58xq/+W6Pf//dgD/qnQC/6p0Av+qdAL/0n8B//+BAP/52kb/+dpG///+Sv///kr//7oI//+xCf//qAT//3YA/6p0Av////8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AOSoHP/nzGr/5bo9/+W6Pf/nzGr/58xq/////wD///8A////AP///wD///8A////AP///wD///8A5bo9/+fMav/52kb//6gE//+QAP//kAD//3YA///+Sv/52kb/+dpG///+Sv/52kb///5K//+6CP//sQn//6gE//+oBP//kAD/rVo/AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AP///wD///8A////AA==", // edit
        "SkinImageWidth": 64,
        "SkinImageHeight": 64,
        "CapeData": "",
        "CapeImageWidth": 0,
        "CapeImageHeight": 0,
        "SkinResourcePatch": "ewogICAiZ2VvbWV0cnkiIDogewogICAgICAiZGVmYXVsdCIgOiAiZ2VvbWV0cnkuaHVtYW5vaWQuY3VzdG9tU2xpbSIKICAgfQp9Cg==", // no edit
        "SkinGeometryData": "bnVsbAo=", // edit
        "SkinGeometryDataEngineVersion": "MC4wLjA=", // edit
        "SkinAnimationData": "",
        "PlayFabId": "a3561c5eacf46e1d", // edit
        "AnimatedImageData": [],
        "ArmSize": "wide",
        "SkinColor": "#0",
        "PersonaPieces": [],
        "PieceTintColors": [],
        "IsEditorMode": false,
        "GameVersion": "1.21.2", // edit
        "DeviceModel": "Samsung Galaxy A11",
        "DeviceOS": 1,
        "DefaultInputMode": 1,
        "CurrentInputMode": 1,
        "UIProfile": 0,
        "GuiScale": -1,
        "LanguageCode": "tr_TR",
        "PlatformUserId": "",
        "ThirdPartyName": display_name, // edit
        "ThirdPartyNameOnly": false,
        "PlatformOnlineId": "",
        "PlatformOfflineId": "",
        "DeviceId": "ebc40067-bfdb-3ad0-af9d-65248592acf1", // edit
        "TrustedSkin": true,
        "PremiumSkin": true,
        "PersonaSkin": false,
        "OverrideSkin": false,
        "CapeOnClassicSkin": false,
        "CapeId": "",
        "CompatibleWithClientSideChunkGen": true
    });

    let header_bytes = to_vec(&header).expect("Header don't convert the byte array");
    let encoded_header = Encryption::b64_url_encode(&header_bytes);

    let payload_bytes = to_vec(&payload).expect("Payload don't convert the byte array");
    let encoded_payload = Encryption::b64_url_encode(&payload_bytes);

    let payload_two_bytes = to_vec(&payload_two).expect("Payload Two don't convert the byte array");
    let encoded_payload_two = Encryption::b64_url_encode(&payload_two_bytes);


    let data_to_sign = format!("{}.{}", encoded_header, encoded_payload);
    let mut signer = Signer::new(openssl::hash::MessageDigest::sha384(), &pkey).expect("Signer not created.");
    signer.update(data_to_sign.as_bytes()).expect("Signer update error.");
    let signature = signer.sign_to_vec().expect("Signature creating error.");
    let ecdsa_sig = EcdsaSig::from_der(&signature).unwrap();
    let r = ecdsa_sig.r().to_vec();
    let s = ecdsa_sig.s().to_vec();
    let concatenated_signature = [r, s].concat();
    let encoded_signature = Encryption::b64_url_encode(&concatenated_signature);
    let jwt = format!("{}.{}.{}", encoded_header, encoded_payload, encoded_signature);
    chain.insert(0, jwt);

    let real_chain = json!({
        "chain": chain
    });


    let data_to_sign_two = format!("{}.{}", encoded_header, encoded_payload_two);
    let mut signer_two = Signer::new(openssl::hash::MessageDigest::sha384(), &pkey).expect("Signer not created.");
    signer_two.update(data_to_sign_two.as_bytes()).expect("Signer update error.");
    let signature_two = signer_two.sign_to_vec().expect("Signature creating error.");
    let ecdsa_sig = EcdsaSig::from_der(&signature_two).unwrap();
    let r = ecdsa_sig.r().to_vec();
    let s = ecdsa_sig.s().to_vec();
    let concatenated_signature_two = [r, s].concat();
    let encoded_signature_two = Encryption::b64_url_encode(&concatenated_signature_two);

    let skin_data = format!("{}.{}.{}", encoded_header, encoded_payload_two, encoded_signature_two);


    vec![real_chain.to_string(), skin_data]

}