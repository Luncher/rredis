use std::str;
use std::net::{TcpStream};
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::error::Error;

const PREFIX_BULK: u8 = b'$';
const PREFIX_ARRAY: u8 = b'*';
const PREFIX_ERROR: u8 = b'-';
const PREFIX_SIMPLE: u8 = b'+';
const PREFIX_INTEGER: u8 = b':';
const LINE_FEED: u8 = b'\n';
const CARRIAGE_RETURN: u8 = b'\r';

enum RespValue {
  Simple(String),
  Error(String),
  Integer(i64),
  Bulk(Vec<u8>),
  Array(Vec<RespValue>),
}

pub struct Resp {
    // command: String,
    payload: RespValue,
}

pub fn parse(stream: TcpStream) -> Resp  {
  let mut reader = BufReader::new(stream);
  Resp {
    payload: parse_payload(&mut reader).unwrap()
  }
}

fn parse_payload(reader: &mut BufReader<TcpStream>) -> Result<RespValue, Box<Error>> {
  let mut buf = vec![0u8; 1];
  reader.read_exact(&mut buf)?;
  match buf[0] {
    PREFIX_BULK => read_value_bulk(reader),
    PREFIX_SIMPLE => read_value_simple(reader),
    PREFIX_ARRAY => read_value_array(reader),
    PREFIX_ERROR => read_value_error(reader),
    PREFIX_INTEGER => read_value_integer(reader),
    _ => panic!(format!("Unknow Prefix: {}", buf[0])),
  }
}

fn read_value_array(reader: &mut BufReader<TcpStream>) -> Result<RespValue, Box<Error>> {
  println!("read_value_array");
  let mut buf = vec![];
  let num_bytes = reader.read_until(LINE_FEED, &mut buf);
  let length = str::from_utf8(&buf[0..(buf.len() - 2)]).unwrap().parse::<usize>().unwrap();
  println!("read_value_array length: {}", length);

  let mut array = vec![];
  for _ in 0..length {
    array.push(parse_payload(reader).unwrap());
  }

  Ok(RespValue::Array(array))
}

fn read_value_bulk(reader: &mut BufReader<TcpStream>) -> Result<RespValue, Box<Error>> {
  println!("read_value_bulk");
  let mut buf = vec![];
  reader.read_until(LINE_FEED, &mut buf)
    .expect("read_value_bulk fail");
  let length = str::from_utf8(&buf[0..(buf.len() - 2)]).unwrap().parse::<usize>().unwrap();
  println!("read_value_bulk length: {}", length);
  let mut bulk = vec![0; length];
  reader.read_exact(&mut bulk)?;
  reader.read_until(LINE_FEED, &mut buf)?;

  println!("bulk: {}", str::from_utf8(&bulk).unwrap());

  Ok(RespValue::Bulk(bulk))
}

fn read_value_simple(reader: &mut BufReader<TcpStream>) -> Result<RespValue, Box<Error>> {
  println!("read_value_simple");
  let mut buf = vec![];
  reader.read_until(LINE_FEED, &mut buf)
    .expect("read_value_simple fail");
  buf.pop();
  buf.pop();

  Ok(RespValue::Simple(String::from_utf8(buf).unwrap()))
}

fn read_value_error(reader: &mut BufReader<TcpStream>) -> Result<RespValue, Box<Error>> {
  println!("read_value_error");
  let mut buf = vec![];
  reader.read_until(LINE_FEED, &mut buf)
    .expect("read_value_error fail");
  buf.pop();
  buf.pop();

  Ok(RespValue::Error(String::from_utf8(buf).unwrap()))
}

fn read_value_integer(reader: &mut BufReader<TcpStream>) -> Result<RespValue, Box<Error>> {
  println!("read_value_integer");
    let mut buf = vec![];
  reader.read_until(LINE_FEED, &mut buf)
    .expect("read_value_integer fail");
  buf.pop();
  buf.pop();

  Ok(RespValue::Integer(String::from_utf8(buf).unwrap().parse::<i64>().unwrap()))
}