
use wasm_bindgen::prelude::*; 

use sha2::{Sha256,Sha512, Digest};
//use rand::{distributions::Alphanumeric, Rng};





#[wasm_bindgen]
pub fn hash(mut password:String)->String{
      // create a Sha256 object
      const SALT:&str="a31158c6-9cb0-11ec-b909-0242ac120002";

      let mut hasher = Sha256::new();
      
      password.push_str(SALT);
      // write input message
      hasher.update(password);
  
      // read hash digest and consume hasher
      let result = hasher.finalize();
      
      return format!("{:x}", result);
}

#[wasm_bindgen]
pub fn second_hash(mut first_hash:String)->String{
    const SALT:&str="4e0e2c19-c33f-4aaa-bf98-ac2e7a545657";
    let mut hasher=Sha512::new();
    first_hash.push_str(SALT);
    hasher.update(first_hash);
    let result=hasher.finalize();
    return format!("{:x}",result);
}
