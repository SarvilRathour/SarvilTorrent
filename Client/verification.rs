use sha2::Digest;
use sha2::Sha256;
pub fn verify_hash(chunk:&[u8],hash:&str)->bool{
    // let chunk_to_string=String::from_utf8(chunk.to_vec()).expect("failure");
   let mut hash_str=String::from("");
   for c in hash.chars(){
    if c!='"'{
        hash_str.push(c);
   }
}
  let mut hasher = Sha256::new();
    hasher.update(chunk);
    let result = hasher.finalize();
    let check_hash_string=hex::encode(result);
    if check_hash_string==hash_str{
        true
    }    else{
        false
    }
}