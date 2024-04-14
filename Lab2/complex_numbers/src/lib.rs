pub mod solution {
    
    use std::hash::Hash;
    use std::ops::{Add, AddAssign};
    use std::default::Default;
    use std::convert::TryInto;

    #[derive(Copy, Clone, Debug)]
    pub struct ComplexNumber{
        real: f64,
        imag: f64,
    }

    impl ComplexNumber {
        pub fn new(real: f64, imag: f64) -> Self {
            Self{real, imag}
        }
        pub fn from_real(real: f64) -> Self {
            Self{real, imag: 0.0}
        }
        pub fn real(&self) -> f64 {
            self.real
        }
        pub fn imag(&self) -> f64 {
            self.imag
        }
        pub fn to_tuple(&self) -> (f64, f64) {
            (self.real,self.imag)
        }
        pub fn into(self) -> f64 {
            if self.imag == 0.0 {self.real} else {panic!("The imag need to be 0.0 to convert a ComplexNumber to a Real Number")}
        }
    }

    
    impl TryInto<f64> for ComplexNumber{
        type Error = String;
    
        fn try_into(self) -> Result<f64, Self::Error> {
            if self.imag == 0.0 {
                Ok(self.real)
            } else {
                Err("To convert a ComplexNumber into a f64 the imag of the number has to be 0.0!".to_string())
            } 
        }
    }
     
    

    impl Into<ComplexNumber> for f64 {
        fn into(self) -> ComplexNumber {
            ComplexNumber::from_real(self)
        }
    }

    

    impl Add<ComplexNumber> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self{
                real: self.real + rhs.real(),
                imag: self.imag + rhs.imag()
            }
        }
    }
    impl Add<&ComplexNumber> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: &Self) -> Self::Output {
            ComplexNumber::add(self,*rhs)
        }
    }
    impl Add for &ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: Self) -> Self::Output {
            ComplexNumber::add(*self,*rhs)
        }
    }
    
    impl Add<f64> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: f64) -> Self::Output {
            Self{
                real: self.real + rhs,
                imag: self.imag 
            }
        }
    }

    impl AddAssign for ComplexNumber {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                real: self.real + other.real(),
                imag: self.imag + other.imag(),
            };
        }
    }

    impl Default for ComplexNumber {
        fn default() -> Self {
            Self { real: 0.0, imag: 0.0 }
        }
    }


    impl PartialEq for ComplexNumber {
        fn eq(&self, other: &Self) -> bool {
            self.real == other.real && self.imag == other.imag
        }
    }

    impl PartialOrd for ComplexNumber{
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Eq for ComplexNumber {
        
    }

    impl Ord for ComplexNumber {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.real.abs().total_cmp(&other.real.abs())
        }
    }


    impl AsRef<f64> for ComplexNumber {
        fn as_ref(&self) -> &f64 {
            &self.real
        }
    }

    impl AsMut<f64> for ComplexNumber {
        fn as_mut(&mut self) -> &mut f64 {
            &mut self.real
        }
    }

    impl Hash for ComplexNumber {
        fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
            hasher.write_u64(self.real.to_bits());
            hasher.write_u64(self.imag.to_bits());
        }
    }
}