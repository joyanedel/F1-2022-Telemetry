// iuse f1_structs defined in f1_structs.rs
mod f1_structs;

use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;

use f1_structs::F1PacketHeader;
