#[macro_use]
extern crate tracing;

pub mod nom_parse;
pub mod enums;
pub mod structs;
pub mod ve_direct_parsing;

use crate::structs::VEDirectBlock;
use nom::bytes::streaming::take_until;
use nom_parse::Block;
use nom::Err;
use nom::error::Error;
use crate::enums::ExtractError;
use crate::nom_parse::parse_block;
use crate::ve_direct_parsing::block_to_vedirect;


pub fn extract_blocks(input: &[u8]) -> Result<Vec<Block>, ExtractError> {
    let (adj_input, _) = match take_until::<_, _, nom::error::Error<&[u8]>>("\r\n".as_bytes())(input) {
        Ok((a, b)) => (a, b),
        Err(e) => {
            match e {
                Err::Incomplete(ref ee) => { return Err(ExtractError::Incomplete); }
                Err::Error(ref err) => { return Err(ExtractError::NoMatch); }
                Err::Failure(ref fail) => { return Err(ExtractError::Failure); }
            }
        }
    };
    match parse_block(adj_input) {
        Ok((_, o)) => {
            debug!("Parse worked for {adj_input:#?}");
            return Ok(o);
        }
        Err(e) => {
            //debug!("Parse failed for {adj_input:#?}");
            match e {
                Err::Incomplete(_) => { return Err(ExtractError::Incomplete); }
                Err::Error(_ee) => { return Err(ExtractError::NoMatch); }
                Err::Failure(_) => { return Err(ExtractError::Failure); }
            }
        }
    }
}

pub fn get_vedirect_data(input: &[u8]) -> Result<Vec<VEDirectBlock>, ExtractError> {
    let mut ve_direct_blocks: Vec<VEDirectBlock> = vec![];
    let blocks = match extract_blocks(input) {
        Ok(b) => b,
        Err(e) => {
            warn!("no blocks found in input data on this call: {e}");
            return Err(ExtractError::NoMatch);
        }
    };
    for block in blocks.iter() {
        ve_direct_blocks.push(block_to_vedirect(block));
    }
    Ok(ve_direct_blocks)
}