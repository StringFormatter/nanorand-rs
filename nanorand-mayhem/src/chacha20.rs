use c2_chacha::{
	stream_cipher::{generic_array::GenericArray, NewStreamCipher, SyncStreamCipher},
	ChaCha20,
};
use nanorand::crypto::chacha;
use std::env;

const KEY_LEN: usize = 32;
const NONCE_LEN: usize = 8;

fn chacha_test(key: [u8; KEY_LEN], nonce: [u8; NONCE_LEN]) {
	let reference_keystream = {
		let mut state = ChaCha20::new(
			GenericArray::from_slice(&key),
			GenericArray::from_slice(&nonce),
		);
		let mut keystream = [0u8; 256];
		state.apply_keystream(&mut keystream);
		keystream
	};

	let our_keystream = {
		let mut state = chacha::chacha_init(key, nonce);
		let mut keystream: Vec<u8> = Vec::with_capacity(reference_keystream.len());

		while reference_keystream.len() > keystream.len() {
			chacha::chacha_block::<20>(state)
				.iter()
				.for_each(|packed| keystream.extend_from_slice(&packed.to_le_bytes()));
			chacha::chacha_increment_counter(&mut state);
		}
		keystream
	};

	assert_eq!(our_keystream, reference_keystream);
}

fn unpack_arg(arg: Option<String>) -> String {
   match arg {
       None => String::from(""),
       Some(s) => s,
   }
}

fn main() {
   let mut key = [0; KEY_LEN];
   let mut nonce = [0; NONCE_LEN];
   let mut args = env::args();
   let pname = unpack_arg(args.next());
   let key_s = unpack_arg(args.next());
   let klen = key_s.len();
   let nonce_s = unpack_arg(args.next());
   let nlen = nonce_s.len();

   if klen == 0 || nlen == 0 {
       println!("Usage: ./{} KEY NONCE", pname);
       return ();
   }
   let key_b = key_s.as_bytes();
   let nonce_b = nonce_s.as_bytes();

   let mut iter = 0;
   while (iter < KEY_LEN) && (iter < klen) {
      key[iter] = key_b[iter];
      iter += 1;
   }
   iter = 0;
   while (iter < NONCE_LEN) && (iter < nlen) {
      nonce[iter] = nonce_b[iter];
      iter += 1;
   }

   chacha_test(key, nonce);
   return ();
}
