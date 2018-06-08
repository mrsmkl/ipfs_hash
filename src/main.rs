extern crate quick_protobuf;
extern crate multihash;
extern crate base58;

use multihash::{encode, Hash};

mod proto;

use std::borrow::Cow;
use quick_protobuf::Writer;

use proto::ipfs::PBLink;

fn data_link(a : &[u8]) -> PBLink {
    let hash = encode(Hash::SHA2256, a).unwrap();
    PBLink{Tsize: Some(a.len() as u64), Name:None, Hash:Some(Cow::from(hash))}
}

fn total_length(lst : &Vec<PBLink>) -> u64 {
    let mut len = 0;
    for i in lst {
        if let Some(sz) = i.Tsize { len += sz }
    };
    len
}

fn concat_links<'a>(lst : &Vec<PBLink<'a>>) -> PBLink<'a> {
    let mut out = Vec::new();
    {
      let nd = proto::ipfs::PBNode{Links:lst.to_vec(), Data:None};
      {
          let mut writer = Writer::new(&mut out);
          writer.write_message(&nd).expect("Cannot write message!");
      }
    }
    let hash = encode(Hash::SHA2256, &out).unwrap();
    PBLink{Tsize: Some(total_length(lst)), Name:None, Hash:Some(Cow::from(hash))}
}

fn hash_block(vec : &Vec<u8>) -> String {
    let hash = encode(Hash::SHA2256, vec).unwrap();
    base58::ToBase58::to_base58(hash.as_slice())
}

fn ipfs_tree(vec : &[u8]) -> PBLink {
    if vec.len() <= 1024 {
       data_link(vec)
    }
    else {
       let mut lst = Vec::new();
       let sz = vec.len();
       let part = sz/10;
       for i in 0..9 {
           let idx = i * part;
           lst.push(ipfs_tree(&vec[idx .. idx+part]));
       };
       lst.push(ipfs_tree(&vec[9*part .. ]));
       concat_links(&lst)
    }
}

// use std::env;
use std::fs::File;
use std::io::prelude::*;

fn read_file(filename : &str) -> Vec<u8> {
    println!("In file {}", filename);

    let mut f = File::open(filename).expect("file not found");

    let mut buffer = vec![];
    // read the whole file
    f.read_to_end(&mut buffer).expect("Blah");
    
    buffer

}

use quick_protobuf::{MessageRead, BytesReader};

fn process_block(dta : &Vec<u8>) {
    let mut reader = BytesReader::from_bytes(&dta);
    let nd = proto::ipfs::PBNode::from_reader(&mut reader, &dta).expect("Cannot read node");
    println!("Perhaps it will work {:?}", nd);
    for l in nd.Links {
       println!("Got link {:?}", base58::ToBase58::to_base58(l.Hash.unwrap().to_mut().as_slice()))
    }
}

fn main() {
    let dta = read_file("ipfsblock");
    process_block(&dta);
    // use proto::ipfs::PBNode::*;
    let crump : &[u8] = "asassasasa".as_bytes();
    let hash = encode(Hash::SHA2256, &crump.to_vec()).unwrap();
    println!("Hashed {:?}", hash);
    let v = Cow::Borrowed(crump);
    let nd = proto::ipfs::PBNode{Links:[].to_vec(), Data:Some(v)};
    let mut out = Vec::new();
    {
        let mut writer = Writer::new(&mut out);
        writer.write_message(&nd).expect("Cannot write message!");
    }
    println!("Serialized {:?} to {:?}", nd, out);
    let res = ipfs_tree("asasassaas".as_bytes());
    println!("Make link {:?}", res);
    println!("Hashing it {:?}", hash_block(& "dsdsdsdssdsdsdds".as_bytes().to_vec()));
}


