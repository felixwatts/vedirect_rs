use crate::nom_parse::Block;
use crate::structs::VEDirectBlock;
use std::str;

pub fn block_to_vedirect(block: &Block) -> VEDirectBlock {
    let mut data: VEDirectBlock = VEDirectBlock::default();
    for (k,v) in block.kvs.iter() {
        // we can be preeety confident that since the block checksum validated, the key will
        // always be valid ascii (and therefore, UTF8)
        let key_name = str::from_utf8(k.as_slice()).unwrap();

        if key_name == "Checksum" {
            // We don't need to process the checksum, it is the last field of every block,
            // and its value_name is *not* guaranteed to be ascii, so we will bail out here.
            return data;
        }
        let value_name: &str;
        match str::from_utf8(v.as_slice()) {
            Ok(v) => value_name = v,
            Err(_) => {
                continue;
            }
        };
        match key_name {
            "V" => { data.battery_volts = value_name.parse::<i32>().unwrap_or(0) as f32 / 1000.0 }
            "V2" => { unimplemented!() }
            "V3" => { unimplemented!() }
            "VS" => { unimplemented!() }
            "VM" => { unimplemented!() }
            "DM" => { unimplemented!() }
            "VPV" => { data.panel_volts = value_name.parse::<i32>().unwrap_or(0) as f32 / 1000.0 }
            "PPV" => { data.panel_power = value_name.parse::<i32>().unwrap_or(0) as f32 / 1000.0 }
            "I" => {
                data.battery_current = value_name.parse::<i32>().unwrap_or(0) as f32 / 1000.0;
            }
            "I2" => { unimplemented!() }
            "I3" => { unimplemented!() }
            "IL" => { data.load_current = value_name.parse::<i32>().unwrap_or(0) as f32 / 1000.0; }
            "LOAD" => {
                if value_name == "ON" {
                    data.load_status = true;
                } else {
                    data.load_status = false;
                }
            }
            "T" => { unimplemented!() }
            "P" => { unimplemented!() }
            "CE" => { unimplemented!() }
            "SOC" => { unimplemented!() }
            "TTG" => { unimplemented!() }
            "Alarm" => { unimplemented!() }
            "Relay" => { unimplemented!() }
            "AR" => { unimplemented!() }
            "OR" => {
                let val = u8::from_str_radix(&value_name, 16).unwrap_or(0);
                let or = num::FromPrimitive::from_u8(val);
                match or {
                    Some(state) => data.off_reason = state,
                    None => error!("Error converting error to value.")
                }
            }
            "H1" => { unimplemented!() }
            "H2" => { unimplemented!() }
            "H3" => { unimplemented!() }
            "H4" => { unimplemented!() }
            "H5" => { unimplemented!() }
            "H6" => { unimplemented!() }
            "H7" => { unimplemented!() }
            "H8" => { unimplemented!() }
            "H9" => { unimplemented!() }
            "H10" => { unimplemented!() }
            "H11" => { unimplemented!() }
            "H12" => { unimplemented!() }
            "H13" => { unimplemented!() }
            "H14" => { unimplemented!() }
            "H15" => { unimplemented!() }
            "H16" => { unimplemented!() }
            "H17" => { unimplemented!() }
            "H18" => { unimplemented!() }
            "H19" => {
                data.yield_total = value_name.parse::<i32>().unwrap_or(0) as f32 / 100.0;
            }
            "H20" => {
                data.yield_today = value_name.parse::<i32>().unwrap_or(0) as f32 / 100.0;
            }
            "H21" => {
                data.maxpower_today = value_name.parse::<i32>().unwrap_or(0);
            }
            "H22" => {
                data.yield_yesterday = value_name.parse::<i32>().unwrap_or(0) as f32 / 100.0;
            }
            "H23" => {
                data.maxpower_yesterday = value_name.parse::<i32>().unwrap_or(0);
            }
            "ERR" => {
                let error = num::FromPrimitive::from_u8(value_name.parse::<u8>().unwrap_or(0));
                match error {
                    Some(state) => data.error_state = state,
                    None => error!("Error converting error to value.")
                }
            }
            "CS" => {
                let cs = num::FromPrimitive::from_u8(value_name.parse::<u8>().unwrap_or(0));
                match cs {
                    Some(state) => data.converter_state = state,
                    None => error!("Error converting error to value.")
                }
            }
            "BMV" => { unimplemented!() }
            "FW" => { data.fw = value_name.to_string(); }
            "FWE" => { unimplemented!() }
            "PID" => {
                data.pid = value_name.to_string();
            }
            "SER#" => { data.serial = value_name.to_string(); }
            "HSDS" => { data.day_sequence_number = value_name.parse::<usize>().unwrap_or(0) }
            "MODE" => {
                unimplemented!()
            }
            "AC_OUT_V" => { unimplemented!() }
            "AC_OUT_I" => { unimplemented!() }
            "AC_OUT_S" => { unimplemented!() }
            "WARN" => { unimplemented!() }
            "MPPT" => {
                let mppt = num::FromPrimitive::from_u8(value_name.parse::<u8>().unwrap_or(0));
                match mppt {
                    Some(state) => data.mppt_mode = state,
                    None => error!("Error converting error to value.")
                }
            }
            "MON" => { unimplemented!() }
            "DC_IN_V" => { unimplemented!() }
            "DC_IN_I" => { unimplemented!() }
            "DC_IN_P" => { unimplemented!() }
            _ => { warn! {"Unknown value received"} }
        }

    }
    data

}
