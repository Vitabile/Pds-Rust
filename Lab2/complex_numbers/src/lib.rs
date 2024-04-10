pub mod solution {
    #[derive(Copy, Clone)]
    pub struct ComplexNumber{
        real: f32,
        imag: f32,
    }

    impl ComplexNumber {
        pub fn new(real: f32, imag: f32) -> Self {
            Self{real, imag}
        }
        pub fn from_real(real: f32) -> Self {
            Self{real, imag: 0.0}
        }
        pub fn real(&self) -> f32 {
            self.real
        }
        pub fn imag(&self) -> f32 {
            self.imag
        }
        pub fn to_tuple(&self) -> (f32, f32) {
            (self.real,self.imag)
        }
    }


    impl core::ops::Add<ComplexNumber> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            Self{
                real: self.real + rhs.real(),
                imag: self.imag + rhs.imag()
            }
        }
    }
    impl core::ops::Add<&ComplexNumber> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: &Self) -> Self::Output {
            ComplexNumber::add(self,*rhs)
        }
    }
    impl core::ops::Add for &ComplexNumber {
        type Output = ComplexNumber;
        fn add(self, rhs: Self) -> Self::Output {
            ComplexNumber::add(*self,*rhs)
        }
    }
    
    impl core::ops::Add<f32> for ComplexNumber {
        type Output = Self;
        fn add(self, rhs: f32) -> Self::Output {
            Self{
                real: self.real + rhs,
                imag: self.imag 
            }
        }
    }

    impl core::ops::AddAssign for ComplexNumber {
        fn add_assign(&mut self, other: Self) {
            *self = Self {
                real: self.real + other.real(),
                imag: self.imag + other.imag(),
            };
        }
    }
}