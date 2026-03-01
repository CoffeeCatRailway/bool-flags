use std::fmt;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign};
use minipaste::paste;

macro_rules! create_flags {
	($name:ident, $vtype:tt, $n:expr) => {
		paste! {
			#[doc = concat!("Condenses ", stringify!($n), " booleans into a single ", stringify!($vtype), ".")]
			#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
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
					self.0 & (1 << (index % $n)) != 0
				}
				
				#[doc = "Flip the n'th bit (flag)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn flip(&mut self, index: $vtype) {
					self.0 ^= 1 << (index % $n);
				}
				
				#[doc = "Reset the n'th bit (flag) to 0 (false)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn clear(&mut self, index: $vtype) {
					self.0 &= !(1 << (index % $n));
				}
				
				#[doc = "Set the n'th bit (flag) to 1 (true)"]
				#[cfg_attr(feature = "inline", inline)]
				pub fn set(&mut self, index: $vtype) {
					self.0 |= 1 << (index % $n);
				}
			}
			
			impl fmt::Display for $name {
				fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
					write!(f, "0b{:0width$b}", self.0, width = size_of::<$vtype>() * 8)
				}
			}
			
			impl BitAnd for $name {
				type Output = $name;
				
				fn bitand(self, rhs: Self) -> Self::Output {
					Self(self.0 & rhs.0)
				}
			}
			
			impl BitAnd<$vtype> for $name {
				type Output = $name;
				
				fn bitand(self, rhs: $vtype) -> Self::Output {
					Self(self.0 & rhs)
				}
			}
			
			impl BitAndAssign for $name {
				fn bitand_assign(&mut self, rhs: Self) {
					self.0 &= rhs.0;
				}
			}
			
			impl BitAndAssign<$vtype> for $name {
				fn bitand_assign(&mut self, rhs: $vtype) {
					self.0 &= rhs;
				}
			}
			
			impl BitOr for $name {
				type Output = $name;
				
				fn bitor(self, rhs: Self) -> Self::Output {
					Self(self.0 | rhs.0)
				}
			}
			
			impl BitOr<$vtype> for $name {
				type Output = $name;
				
				fn bitor(self, rhs: $vtype) -> Self::Output {
					Self(self.0 | rhs)
				}
			}
			
			impl BitOrAssign for $name {
				fn bitor_assign(&mut self, rhs: Self) {
					self.0 |= rhs.0;
				}
			}
			
			impl BitOrAssign<$vtype> for $name {
				fn bitor_assign(&mut self, rhs: $vtype) {
					self.0 |= rhs;
				}
			}
			
			impl BitXor for $name {
				type Output = $name;
				
				fn bitxor(self, rhs: Self) -> Self::Output {
					Self(self.0 ^ rhs.0)
				}
			}
			
			impl BitXor<$vtype> for $name {
				type Output = $name;
				
				fn bitxor(self, rhs: $vtype) -> Self::Output {
					Self(self.0 ^ rhs)
				}
			}
			
			impl BitXorAssign for $name {
				fn bitxor_assign(&mut self, rhs: Self) {
					self.0 ^= rhs.0;
				}
			}
			
			impl BitXorAssign<$vtype> for $name {
				fn bitxor_assign(&mut self, rhs: $vtype) {
					self.0 ^= rhs;
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
		macro_rules! check_functions {
			($ftype:tt, $vtype:tt) => {
				// Check from_TYPE
				paste! {
					assert_eq!($ftype::[<from_ $vtype>](1).0, 1);
					assert_eq!($ftype::[<from_ $vtype>](2).0, 2);
					assert_eq!($ftype::[<from_ $vtype>](1 | 2).0, 1 | 2);
				}
				
				// Check none and all
				assert_eq!($ftype::none().0, $vtype::MIN);
				assert_eq!($ftype::all().0, $vtype::MAX);
				
				// Check get, flip, clear, set
				let mut flags = $ftype::none();
				assert_eq!(flags.get(0), false);
				flags.flip(0);
				assert_eq!(flags.get(0), true);
				flags.clear(0);
				assert_eq!(flags.get(0), false);
				flags.set(0);
				assert_eq!(flags.get(0), true);
				
				println!("{} functions passed", stringify!($ftype));
			};
		}
		check_functions!(Flags8, u8);
		check_functions!(Flags16, u16);
		check_functions!(Flags32, u32);
		check_functions!(Flags64, u64);
		check_functions!(Flags128, u128);
		#[cfg(feature = "usize")]
		check_functions!(FlagsUSize, usize);
    }
	
	#[test]
	fn display() { // Check Display is correct
		assert_eq!(format!("{}", Flags8::none()), "0b00000000");
		assert_eq!(format!("{}", Flags8::all()), "0b11111111");
		println!("Flags8 display passed");
		
		assert_eq!(format!("{}", Flags16::none()), "0b0000000000000000");
		assert_eq!(format!("{}", Flags16::all()), "0b1111111111111111");
		println!("Flags16 display passed");
		
		assert_eq!(format!("{}", Flags32::none()), "0b00000000000000000000000000000000");
		assert_eq!(format!("{}", Flags32::all()), "0b11111111111111111111111111111111");
		println!("Flags32 display passed");
		
		assert_eq!(format!("{}", Flags64::none()), "0b0000000000000000000000000000000000000000000000000000000000000000");
		assert_eq!(format!("{}", Flags64::all()), "0b1111111111111111111111111111111111111111111111111111111111111111");
		println!("Flags64 display passed");
		
		assert_eq!(format!("{}", Flags128::none()), "0b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000");
		assert_eq!(format!("{}", Flags128::all()), "0b11111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111111");
		println!("Flags128 display passed");
		
		#[cfg(feature = "usize")]
		{
			assert_eq!(format!("{}", FlagsUSize::none()), format!("0b{:0width$b}", usize::MIN, width = size_of::<usize>() * 8));
			assert_eq!(format!("{}", FlagsUSize::all()), format!("0b{:0width$b}", usize::MAX, width = size_of::<usize>() * 8));
			println!("FlagsUSize display passed");
		}
	}
	
	#[test]
	fn bit_ops() {
		macro_rules! check_bit_ops {
			($ftype:tt, $vtype:tt) => {
				let all = $ftype::all();
				paste! {
					let one = $ftype::[<from_ $vtype>](1);
					let not_one = $ftype::[<from_ $vtype>](($vtype::MAX - 1));
				}
				assert_eq!(all & one, one);
				assert_eq!(all & one.0, one);
				
				assert_eq!(all | one, all);
				assert_eq!(all | one.0, all);
				
				assert_eq!(all ^ one, not_one);
				assert_eq!(all ^ one.0, not_one);
				
				// And assign
				let mut and_assign = $ftype::all();
				and_assign &= one;
				assert_eq!(and_assign, one);
				
				let mut and_assign = $ftype::all();
				and_assign &= one.0;
				assert_eq!(and_assign, one);
				
				// Or assign
				let mut or_assign = $ftype::all();
				or_assign |= one;
				assert_eq!(or_assign, all);
				
				let mut or_assign = $ftype::all();
				or_assign |= one.0;
				assert_eq!(or_assign, all);
				
				// Xor assign
				let mut xor_assign = $ftype::all();
				xor_assign ^= one;
				assert_eq!(xor_assign, not_one);
				
				let mut xor_assign = $ftype::all();
				xor_assign ^= one.0;
				assert_eq!(xor_assign, not_one);
				
				println!("{} bit ops passed", stringify!($ftype));
			};
		}
		check_bit_ops!(Flags8, u8);
		check_bit_ops!(Flags16, u16);
		check_bit_ops!(Flags32, u32);
		check_bit_ops!(Flags64, u64);
		check_bit_ops!(Flags128, u128);
		#[cfg(feature = "usize")]
		check_bit_ops!(FlagsUSize, usize);
	}
}
