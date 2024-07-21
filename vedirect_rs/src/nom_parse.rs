use nom::bytes::complete::{tag, take, take_until};
use nom::multi::many0;
use nom::sequence::{pair, preceded, separated_pair};
use nom::IResult;
use std::str;
use nom::Err;

#[derive(Debug)]
pub struct Block {
    pub kvs: Vec<(Vec<u8>, Vec<u8>)>,
}

pub fn parse_block(input: &[u8]) -> IResult<&[u8], Vec<Block>> {
    let mut checksum_calc: u32 = 0;
    let mut temp_vec: Vec<(Vec<u8>, Vec<u8>)> = vec![];
    let mut blocks: Vec<Block> = vec![];

    let (rest, (kvs, checksum)) = match pair(parse_kvs, parse_checksum)(input) {
        Ok((a, (b,c))) => (a,(b,c)),
        Err(e) => {
            match e {
                Err::Incomplete(_) => { return Err(e);},
                Err::Error(_) => {  return Err(e);}
                Err::Failure(_) => {return Err(e);}
            }
        }
    };

    // separate kvs into actual blocks and checksum them
    for (idx, (k, v)) in kvs.iter().enumerate() {
        let k_str: &str;
        match str::from_utf8(k.as_slice()) {
            Ok(k) => k_str = k,
            Err(_) => {
                continue;
            }
        };
        // first we add a \r and an \n
        checksum_calc = checksum_calc + 13_u32 + 10_u32;

        // then we add up the key bytes
        for kc in k.iter() {
            checksum_calc = checksum_calc + *kc as u32;
        }
        // tab char
        checksum_calc = checksum_calc + 9_u32;
        // then value
        for vc in v.iter() {
            checksum_calc = checksum_calc + *vc as u32;
        }
        if (idx == kvs.len() - 1) && k_str != "Checksum" {
            temp_vec.push((k.clone(), v.clone()));
            // we are on the last kv but no checksum was found
            let checksum_sum = 67 + 104 + 101 + 99 + 107 + 115 + 117 + 109;
            checksum_calc = checksum_calc + 13_u32 + 10_u32 + checksum_sum as u32 + 9_u32 + checksum as u32;
            let checksum_check = checksum_calc % 256;
            if checksum_check == 0 {
                blocks.push(Block { kvs: temp_vec.clone() });
                temp_vec.clear();
                continue;
            } else {
                debug!("checksum did not match: {checksum_check} should be 0");
                temp_vec.clear();
                checksum_calc = 0;
                continue;
            }
        }
        if k_str == "Checksum" {
            let checksum_check = checksum_calc % 256;
            if checksum_check == 0 {
                blocks.push(Block { kvs: temp_vec.clone() });
                temp_vec.clear();
                continue;
            } else {
                debug!("checksum did not match: {checksum_check} should be 0");
                temp_vec.clear();
                checksum_calc = 0;
                continue;
            }
        }
        temp_vec.push((k.clone(), v.clone()));
    }


    Ok((rest, blocks))
}

fn parse_kvs(input: &[u8]) -> IResult<&[u8], Vec<(Vec<u8>, Vec<u8>)>> {
    // the list of kvs is just many kvs
    many0(parse_kv)(input)
}

fn parse_kv(input: &[u8]) -> IResult<&[u8], (Vec<u8>, Vec<u8>)> {
    // the KV format is:
    //   - \r\n
    //   - field label
    //   - \t
    //   - field value
    preceded(
        tag(b"\r\n"),
        separated_pair(parse_field_label, tag(b"\t"), parse_field_value),
    )(input)
}

fn parse_field_label(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (rest, label) = take_until("\t")(input)?;
    Ok((rest, label.to_vec()))
}

fn parse_field_value(input: &[u8]) -> IResult<&[u8], Vec<u8>> {
    let (rest, value) = take_until("\r")(input)?;
    Ok((rest, value.to_vec()))
}

fn parse_checksum(input: &[u8]) -> IResult<&[u8], u8> {
    preceded(
        tag(b"\r\n"),
        // We don't actually care about the field label here, just the value. The value is always exactly 1 byte.
        preceded(tag("Checksum\t"), parse_checksum_byte),
    )(input)
}

fn parse_checksum_byte(input: &[u8]) -> IResult<&[u8], u8> {
    let (rest, bytes) = take(1usize)(input)?;
    Ok((rest, bytes[0]))
}

