/*
Copyright 2020 Martin Buck

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"),
to deal in the Software without restriction, including without limitation the
rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall
be included all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,
TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE
OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/

//! Module for load functions of the las file format

use crate::*;

use super::types::*;

use std::{
    convert::TryInto,
    io::{BufRead, Read, Seek},
};

//@todo enforce matching sizes, at compile time if possible
pub fn convert<F, T>(from: &[F], to: &mut [T])
where
    T: From<F>,
{
    for i in 0..from.len() {
        to[i] = from[i].into()
    }
}

//------------------------------------------------------------------------------

//@todo move, generalize
//@todo little/big endian
/*fn into_chars(buffer: &[u8], target: &mut [char]) -> LasResult<()> {
    let size = std::mem::size_of::<char>();
    let n = target.len();

    for i in 0..n {
        target[i] = char::from_le_bytes(buffer[i*size..(i+1)*size].try_into()?)
    }
}*/

//------------------------------------------------------------------------------

/// Loads points from .las file into IsPushable<IsBuildable3D>
pub fn load_las<IP, P, R>(read: &mut R, _ip: &mut IP) -> LasResult<()>
//@todo result
where
    IP: IsPushable<P>,
    P: IsBuildable3D,
    R: BufRead + Seek,
{
    let header = load_header(read)?;

    Ok(())
}

fn load_header<R>(read: &mut R) -> LasResult<Header>
where
    R: Read,
{
    //@todo result
    let mut buffer = [0u8; 375];
    read.read_exact(&mut buffer)?;

    //@todo le or be?

    Ok(Header {
        file_signature: {
            let result = ['0'; 4];
            convert(&buffer[0..4], &mut result);
            result
        },
        file_source_id: u16::from_le_bytes(buffer[4..6].try_into()?), //2 6
        global_encoding: u16::from_le_bytes(buffer[6..8].try_into()?), //2 8
        guid1: u32::from_le_bytes(buffer[8..12].try_into()?),         //4 12
        guid2: u16::from_le_bytes(buffer[12..14].try_into()?),        //2 14
        guid3: u16::from_le_bytes(buffer[14..16].try_into()?),        //2 16
        guid4: buffer[16..24].try_into()?,  //8 24
        version_major: u8::from_le_bytes(buffer[24..25].try_into()?), //1 25
        version_minor: u8::from_le_bytes(buffer[25..26].try_into()?), //1 26
        system_identifier: <[char; 32]>::from_le_bytes(buffer[26..58].try_into()?), //32 58
        generating_software: <[char; 32]>::from_le_bytes(buffer[58..90].try_into()?), //32 90
        file_creation_day: u16::from_le_bytes(buffer[90..92].try_into()?), //2 92
        file_creation_year: u16::from_le_bytes(buffer[92..94].try_into()?), //2 94
        header_size: u16::from_le_bytes(buffer[94..96].try_into()?),  //2 96
        offset_point_data: u32::from_le_bytes(buffer[96..100].try_into()?), //4 100
        n_variable_length_records: u32::from_le_bytes(buffer[100..104].try_into()?), //4 104
        point_record_format: u8::from_le_bytes(buffer[104..105].try_into()?), //1 105
        point_record_length: u16::from_le_bytes(buffer[105..107].try_into()?), //2 107
        legacy_n_point_records: u32::from_le_bytes(buffer[107..111].try_into()?), //4 111
        legacy_n_point_return: <[u32; 5]>::from_le_bytes(buffer[111..131].try_into()?), //20 131
        scale_factor_x: f64::from_le_bytes(buffer[131..139].try_into()?), //8 139
        scale_factor_y: f64::from_le_bytes(buffer[139..147].try_into()?), //8 147
        scale_factor_z: f64::from_le_bytes(buffer[147..155].try_into()?), //8 155
        offset_x: f64::from_le_bytes(buffer[155..163].try_into()?),   //8 163
        offset_y: f64::from_le_bytes(buffer[163..171].try_into()?),   //8 171
        offset_z: f64::from_le_bytes(buffer[171..179].try_into()?),   //8 179
        max_x: f64::from_le_bytes(buffer[179..187].try_into()?),      //8 187
        min_x: f64::from_le_bytes(buffer[187..195].try_into()?),      //8 195
        max_y: f64::from_le_bytes(buffer[195..203].try_into()?),      //8 203
        min_y: f64::from_le_bytes(buffer[203..211].try_into()?),      //8 211
        max_z: f64::from_le_bytes(buffer[211..219].try_into()?),      //8 219
        min_z: f64::from_le_bytes(buffer[219..227].try_into()?),      //8 227
        start_wavefront_data: u64::from_le_bytes(buffer[227..235].try_into()?), //8 235
        start_extended_variable_length: u64::from_le_bytes(buffer[235..243].try_into()?), //8 243
        n_extended_variable_length: u32::from_le_bytes(buffer[243..247].try_into()?), //4 247
        n_point_records: u64::from_le_bytes(buffer[247..255].try_into()?), //8 255
        n_points_return: <[u64; 15]>::from_le_bytes(buffer[255..375].try_into()?), //120 375
    })
}