use std::fmt;
use minipaste::paste;

macro_rules! create_flags {
	($name:ident, $vtype:tt, $n:expr) => {
		paste! {
			#[doc = concat!("Condenses ", stringify!($n), " booleans into a single ", stringify!($vtype), ".")]
			#[derive(Debug, Copy, Clone)]
			pub struct $name($vtype);
			
			impl $name {
				#[doc = "Manually set all flags"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn [<from_ $vtype>](value: $vtype) -> $name {
					$name(value)
				}
				
				#[doc = "All flags are false"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn none() -> $name {
					$name($vtype::MIN)
				}
				
				#[doc = "All flags are true"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn all() -> $name {
					$name($vtype::MAX)
				}
				
				#[doc = "Get the n'th bit (flag)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn get(&self, index: $vtype) -> bool {
					let index = index % $n;
					self.0 & (1 << index) != 0
				}
				
				#[doc = "Flip the n'th bit (flag)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn flip(&mut self, index: usize) {
					let index = index % $n;
					self.0 ^= 1 << index;
				}
				
				#[doc = "Reset the n'th bit (flag) to 0 (false)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn clear(&mut self, index: usize) {
					let index = index % $n;
					self.0 &= !(1 << index);
				}
				
				#[doc = "Set the n'th bit (flag) to 1 (true)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn set(&mut self, index: usize) {
					let index = index % $n;
					self.0 |= 1 << index;
				}
			}
			
			impl fmt::Display for $name {
				fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
					write!(f, "0b{:0width$b}", self.0, width = size_of::<$vtype>() * 8)
				}
			}
		}
	};
}

create_flags!(Flags8, u8, 8);
create_flags!(Flags16, u16, 16);
create_flags!(Flags32, u32, 32);
create_flags!(Flags64, u64, 64);
create_flags!(Flags128, u128, 128);
#[cfg(feature = "usize")]
create_flags!(FlagsUSize, usize, usize::MAX);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn functions() {
		const FLAG_0: u8 = 0b00000001;
		const FLAG_1: u8 = 0b00000010;
		
		assert_eq!(Flags8::from_u8(FLAG_0).0, FLAG_0);
		assert_eq!(Flags8::from_u8(FLAG_1).0, FLAG_1);
		assert_eq!(Flags8::from_u8(FLAG_0 | FLAG_1).0, FLAG_0 | FLAG_1);
		
		assert_eq!(Flags8::none().0, 0b00000000);
		assert_eq!(Flags8::all().0, 0b11111111);
		
		let mut flags = Flags8::none();
		assert_eq!(flags.get(0), false);
		flags.flip(0);
		assert_eq!(flags.get(0), true);
		flags.clear(0);
		assert_eq!(flags.get(0), false);
		flags.set(0);
		assert_eq!(flags.get(0), true);
    }
	
	#[test]
	fn display() {
		assert_eq!(format!("{}", Flags8::none()), "0b00000000");
		assert_eq!(format!("{}", Flags8::all()), "0b11111111");
		
		assert_eq!(format!("{}", Flags16::none()), "0b0000000000000000");
		assert_eq!(format!("{}", Flags16::all()), "0b1111111111111111");
		
		assert_eq!(format!("{}", Flags32::none()), "0b00000000000000000000000000000000");
		assert_eq!(format!("{}", Flags32::all()), "0b11111111111111111111111111111111");
		
		assert_eq!(format!("{}", Flags64::none()), "0b0000000000000000000000000000000000000000000000000000000000000000");
		assert_eq!(format!("{}", Flags64::all()), "0b1111111111111111111111111111111111111111111111111111111111111111");
		
		assert_eq!(format!("{}", Flags128::none()), "0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
		assert_eq!(format!("{}", Flags128::all()), "0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
		
		#[cfg(feature = "usize")]
		{
			assert_eq!(format!("{}", FlagsUSize::none()), format!("0b{:0width$b}", usize::MIN, width = size_of::<usize>() * 8));
			assert_eq!(format!("{}", FlagsUSize::all()), format!("0b{:0width$b}", usize::MAX, width = size_of::<usize>() * 8));
		}
	}
}
