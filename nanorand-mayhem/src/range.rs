use nanorand::{tls_rng, Rng};
use std::env;

fn range_test(a: u16, b: u16, range_type: bool) {
	let (lower, upper) = if a > b { (b, a) } else { (a, b) };
	if range_type {
		let number = tls_rng().generate_range(lower..=upper);
		if upper == lower {
			assert_eq!(
				number, upper,
				"{} was outside of range {}=={}",
				number, lower, upper
			);
		} else {
			assert!(
				number >= lower,
				"{} was bigger than range {}..={}",
				number,
				lower,
				upper
			);
			assert!(
				number <= upper,
				"{} was smaller than range {}..={}",
				number,
				lower,
				upper
			);
		}
	} else {
		let number = tls_rng().generate_range(lower..upper);
		if upper == lower {
			assert_eq!(
				number, upper,
				"{} was outside of range {}=={}",
				number, lower, upper
			);
		} else {
			assert!(
				number >= lower,
				"{} was bigger than range {}..{}",
				number,
				lower,
				upper
			);
			assert!(
				number < upper,
				"{} was smaller than range {}..{}",
				number,
				lower,
				upper
			);
		}
	};
}

fn unpack_arg(arg: Option<String>) -> String {
   match arg {
       None => String::from(""),
       Some(s) => s,
   }
}

fn unpack_int(arg: Option<String>) -> Option<u16> {
   match arg {
       None => None,
       Some(s) => match s.parse() { Err(_) => None,
                                  Ok(u) => Some(u),
       }
   }
}

fn main() {
    let mut args = env::args();
    unpack_arg(args.next());
    let a_o = unpack_int(args.next());
    let b_o = unpack_int(args.next());
    let dbg_s = "First two arguments form an invalid range";
    match (a_o,b_o) {
        (None,_) => println!("{}",dbg_s),
        (_,None) => println!("{}",dbg_s),
        (Some(a),Some(b)) => {range_test(a,b,true); range_test(a,b,false)},
    }
    return {};
}
