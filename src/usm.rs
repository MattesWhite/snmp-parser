//! RFC2274 - User-based Security Model (USM) for version 3 of the Simple Network Management Protocol (SNMPv3)

use crate::parse_ber_octetstring_as_str;
use asn1_rs::FromBer;
use der_parser::ber::*;
use der_parser::error::BerError;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct UsmSecurityParameters<'a> {
    pub msg_authoritative_engine_id: &'a [u8],
    pub msg_authoritative_engine_boots: u32,
    pub msg_authoritative_engine_time: u32,
    pub msg_user_name: String,
    pub msg_authentication_parameters: &'a [u8],
    pub msg_privacy_parameters: &'a [u8],
}

pub fn parse_usm_security_parameters(i: &[u8]) -> IResult<&[u8], UsmSecurityParameters, BerError> {
    parse_ber_sequence_defined_g(|i, _| {
        let (i, msg_authoritative_engine_id) = <&[u8]>::from_ber(i)?;
        let (i, msg_authoritative_engine_boots) = parse_ber_u32(i)?;
        let (i, msg_authoritative_engine_time) = parse_ber_u32(i)?;
        let (i, msg_user_name) = parse_ber_octetstring_as_str(i)?;
        let (i, msg_authentication_parameters) = <&[u8]>::from_ber(i)?;
        let (i, msg_privacy_parameters) = <&[u8]>::from_ber(i)?;
        let usm = UsmSecurityParameters {
            msg_authoritative_engine_id,
            msg_authoritative_engine_boots,
            msg_authoritative_engine_time,
            msg_user_name: msg_user_name.to_string(),
            msg_authentication_parameters,
            msg_privacy_parameters,
        };
        Ok((i, usm))
    })(i)
}
