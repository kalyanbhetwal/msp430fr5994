#[doc = "Register `LEACMDSTAT` reader"]
pub type R = crate::R<LeacmdstatSpec>;
#[doc = "Register `LEACMDSTAT` writer"]
pub type W = crate::W<LeacmdstatSpec>;
#[doc = "LEA instruction handshake synchronization type flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leaitflg {
    #[doc = "0: LEA command without any further indication"]
    Leaitflg0 = 0,
    #[doc = "1: LEA command with explicit result update"]
    Leaitflg1 = 1,
    #[doc = "2: LEA command with interrupt upon completion"]
    Leaitflg2 = 2,
    #[doc = "3: LEA command with interrupt and explicit result update"]
    Leaitflg3 = 3,
}
impl From<Leaitflg> for u8 {
    #[inline(always)]
    fn from(variant: Leaitflg) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leaitflg {
    type Ux = u8;
}
impl crate::IsEnum for Leaitflg {}
#[doc = "Field `LEAITFLG` reader - LEA instruction handshake synchronization type flags"]
pub type LeaitflgR = crate::FieldReader<Leaitflg>;
impl LeaitflgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leaitflg {
        match self.bits {
            0 => Leaitflg::Leaitflg0,
            1 => Leaitflg::Leaitflg1,
            2 => Leaitflg::Leaitflg2,
            3 => Leaitflg::Leaitflg3,
            _ => unreachable!(),
        }
    }
    #[doc = "LEA command without any further indication"]
    #[inline(always)]
    pub fn is_leaitflg_0(&self) -> bool {
        *self == Leaitflg::Leaitflg0
    }
    #[doc = "LEA command with explicit result update"]
    #[inline(always)]
    pub fn is_leaitflg_1(&self) -> bool {
        *self == Leaitflg::Leaitflg1
    }
    #[doc = "LEA command with interrupt upon completion"]
    #[inline(always)]
    pub fn is_leaitflg_2(&self) -> bool {
        *self == Leaitflg::Leaitflg2
    }
    #[doc = "LEA command with interrupt and explicit result update"]
    #[inline(always)]
    pub fn is_leaitflg_3(&self) -> bool {
        *self == Leaitflg::Leaitflg3
    }
}
#[doc = "These bits represent the LEA command to be invoked. See also the command table\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leacmd {
    #[doc = "0: Suspends ongoing action an enters Ready"]
    Suspend = 0,
    #[doc = "2: Resumes an previously suspended command execution"]
    Resume = 2,
    #[doc = "4: Complex FFT on 16 bit fractional numbers fix scaling"]
    Fftcomplexfixedscaling = 4,
    #[doc = "6: Real FIR on 16 bit fractional numbers"]
    Fir = 6,
    #[doc = "8: Real vector polynomial calculations 16 args all fractional"]
    Polynomial = 8,
    #[doc = "10: Real FFT-extension on 16 bit fractional numbers"]
    Fft = 10,
    #[doc = "12: Real vector polynomial calculations 32 bit args all fractional"]
    Polynomiallong = 12,
    #[doc = "13: Real row oriented matrix multiply"]
    Mpymatrixrow = 13,
    #[doc = "15: Real matrix multiply 16 with 16 to 16 bit fractional"]
    Mpymatrix = 15,
    #[doc = "16: Real point wise matrix add of 16 and 16 to 16 bit number vector"]
    Addmatrix = 16,
    #[doc = "17: Real maximum value and position of 16 bit matrices"]
    Maxmatrix = 17,
    #[doc = "18: Real minimum value and position of 16 bit matrices"]
    Minmatrix = 18,
    #[doc = "19: Real second order biquad using DF1 with 16 bit fractional"]
    Iirbq1 = 19,
    #[doc = "21: Real matrix MAC short with 16Bt to 16B fract"]
    Mac = 21,
    #[doc = "22: Split 16B vector even to even words"]
    Deinterleaveeveneven = 22,
    #[doc = "23: Split 16Bt vector even to odd words"]
    Deinterleaveevenodd = 23,
    #[doc = "24: Split 16B vector odd to even words"]
    Deinterleaveoddeven = 24,
    #[doc = "25: Split 16B vector odd to odd words"]
    Deinterleaveoddodd = 25,
    #[doc = "26: Complex Dot Product"]
    Maccomplexmatrix = 26,
    #[doc = "27: Complex conjugate Dot Product"]
    Maccomplexconjugatematrix = 27,
    #[doc = "28: Real point wise matrix Subtraction of 16 and 16 to 16 bit"]
    Submatrix = 28,
    #[doc = "29: Real point wise matrix multiply 32 with 32 to 32 bit fractional"]
    Mpylongmatrix = 29,
    #[doc = "30: Complex point wise matrix multiply complex with complex"]
    Mpycomplexmatrix = 30,
    #[doc = "31: Real point wise matrix add of 32 and 32 to 32 bit number"]
    Addlongmatrix = 31,
    #[doc = "32: List move 32 to 32 bit"]
    Movelonglist = 32,
    #[doc = "33: Complex bit reversal for 16 bit fractional numbers even"]
    Bitreversecomplexeven = 33,
    #[doc = "34: Complex bit reversal for 16 bit fractional Numbers odd"]
    Bitreversecomplexodd = 34,
    #[doc = "36: Real second order biquad using DF2 with 16 bit fractional, extended to include bias and intermediate state min/max"]
    Iirbq2extended = 36,
    #[doc = "39: Complex FFT on 32B bit fractional numbers, fix scaling"]
    Fftcomplexlong = 39,
    #[doc = "41: Real FFT-extension on 32 bit fractional numbers"]
    Fftlong = 41,
    #[doc = "43: Complex bit reversal for 32 bit fractional numbers even"]
    Bitreversecomplexlongeven = 43,
    #[doc = "45: Complex bit reversal for 16 bit fractional numbers odd"]
    Bitreversecomplexlongodd = 45,
    #[doc = "47: Scalar Polynomial for math on 32bit fractional"]
    Polynomialscalar = 47,
    #[doc = "48: Complex FFT on 16B bit fractional numbers with auto scaling for enhanced accuracy"]
    Fftcomplexautoscaling = 48,
    #[doc = "50: Real FIR on 32 bit fractional numbers"]
    Firlong = 50,
    #[doc = "52: Real block MAC on 32B fractional numbers"]
    Maclongmatrix = 52,
    #[doc = "53: Real point wise matrix Subtraction of 32 and 32 to 32 bit"]
    Sublongmatrix = 53,
    #[doc = "54: Real maximum value and position of signed 32B matrices"]
    Maxlongmatrix = 54,
    #[doc = "55: Real minimum value and position of signed 32B matrices"]
    Minlongmatrix = 55,
    #[doc = "56: Complex FIR on 16B fractional numbers"]
    Fircomplex = 56,
    #[doc = "58: Real maximum value and position of unsigned 16B matrices"]
    Maxunsignedmatrix = 58,
    #[doc = "59: Real minimum value and position of unsigned 32B matrices"]
    Minunsignedmatrix = 59,
    #[doc = "64: Real Matrix MAC on 16B fractional"]
    Macmatrix = 64,
    #[doc = "65: Vector maximum on 16B signed numbers"]
    Max = 65,
    #[doc = "66: Vector minimum on 16B signed numbers"]
    Min = 66,
    #[doc = "67: Vector maximum on 16B unsigned numbers"]
    Maxunsigned = 67,
    #[doc = "68: Vector minimum on 16B unsigned numbers"]
    Minunsigned = 68,
    #[doc = "69: Matrix maximum on 32B unsigned numbers"]
    Maxunsignedlongmatrix = 69,
    #[doc = "70: Matrix minimum on 32B unsigned numbers"]
    Minunsignedlongmatrix = 70,
    #[doc = "71: Real second order biquad using DF2 with 16 bit fractional"]
    Iirbq2 = 71,
    #[doc = "73: Complex FIR on 32B fractional numbers"]
    Fircomplexlong = 73,
    #[doc = "75: Split Function on 32B Vectors/Matrices"]
    Deinterleavelong = 75,
    #[doc = "76: In-place symmetrical window on 16B fractional numbers"]
    Window = 76,
    #[doc = "77: Vector MAC at three points, real 16-bit with 32-bit result"]
    Mac3 = 77,
    #[doc = "78: Scaled vector multiply and accumulate (MAC)"]
    Scaledmac = 78,
    #[doc = "79: Scaled FIR, 16-bit real fractional numbers"]
    Scaledfir = 79,
}
impl From<Leacmd> for u8 {
    #[inline(always)]
    fn from(variant: Leacmd) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leacmd {
    type Ux = u8;
}
impl crate::IsEnum for Leacmd {}
#[doc = "Field `LEACMD` reader - These bits represent the LEA command to be invoked. See also the command table"]
pub type LeacmdR = crate::FieldReader<Leacmd>;
impl LeacmdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Leacmd> {
        match self.bits {
            0 => Some(Leacmd::Suspend),
            2 => Some(Leacmd::Resume),
            4 => Some(Leacmd::Fftcomplexfixedscaling),
            6 => Some(Leacmd::Fir),
            8 => Some(Leacmd::Polynomial),
            10 => Some(Leacmd::Fft),
            12 => Some(Leacmd::Polynomiallong),
            13 => Some(Leacmd::Mpymatrixrow),
            15 => Some(Leacmd::Mpymatrix),
            16 => Some(Leacmd::Addmatrix),
            17 => Some(Leacmd::Maxmatrix),
            18 => Some(Leacmd::Minmatrix),
            19 => Some(Leacmd::Iirbq1),
            21 => Some(Leacmd::Mac),
            22 => Some(Leacmd::Deinterleaveeveneven),
            23 => Some(Leacmd::Deinterleaveevenodd),
            24 => Some(Leacmd::Deinterleaveoddeven),
            25 => Some(Leacmd::Deinterleaveoddodd),
            26 => Some(Leacmd::Maccomplexmatrix),
            27 => Some(Leacmd::Maccomplexconjugatematrix),
            28 => Some(Leacmd::Submatrix),
            29 => Some(Leacmd::Mpylongmatrix),
            30 => Some(Leacmd::Mpycomplexmatrix),
            31 => Some(Leacmd::Addlongmatrix),
            32 => Some(Leacmd::Movelonglist),
            33 => Some(Leacmd::Bitreversecomplexeven),
            34 => Some(Leacmd::Bitreversecomplexodd),
            36 => Some(Leacmd::Iirbq2extended),
            39 => Some(Leacmd::Fftcomplexlong),
            41 => Some(Leacmd::Fftlong),
            43 => Some(Leacmd::Bitreversecomplexlongeven),
            45 => Some(Leacmd::Bitreversecomplexlongodd),
            47 => Some(Leacmd::Polynomialscalar),
            48 => Some(Leacmd::Fftcomplexautoscaling),
            50 => Some(Leacmd::Firlong),
            52 => Some(Leacmd::Maclongmatrix),
            53 => Some(Leacmd::Sublongmatrix),
            54 => Some(Leacmd::Maxlongmatrix),
            55 => Some(Leacmd::Minlongmatrix),
            56 => Some(Leacmd::Fircomplex),
            58 => Some(Leacmd::Maxunsignedmatrix),
            59 => Some(Leacmd::Minunsignedmatrix),
            64 => Some(Leacmd::Macmatrix),
            65 => Some(Leacmd::Max),
            66 => Some(Leacmd::Min),
            67 => Some(Leacmd::Maxunsigned),
            68 => Some(Leacmd::Minunsigned),
            69 => Some(Leacmd::Maxunsignedlongmatrix),
            70 => Some(Leacmd::Minunsignedlongmatrix),
            71 => Some(Leacmd::Iirbq2),
            73 => Some(Leacmd::Fircomplexlong),
            75 => Some(Leacmd::Deinterleavelong),
            76 => Some(Leacmd::Window),
            77 => Some(Leacmd::Mac3),
            78 => Some(Leacmd::Scaledmac),
            79 => Some(Leacmd::Scaledfir),
            _ => None,
        }
    }
    #[doc = "Suspends ongoing action an enters Ready"]
    #[inline(always)]
    pub fn is_suspend(&self) -> bool {
        *self == Leacmd::Suspend
    }
    #[doc = "Resumes an previously suspended command execution"]
    #[inline(always)]
    pub fn is_resume(&self) -> bool {
        *self == Leacmd::Resume
    }
    #[doc = "Complex FFT on 16 bit fractional numbers fix scaling"]
    #[inline(always)]
    pub fn is_fftcomplexfixedscaling(&self) -> bool {
        *self == Leacmd::Fftcomplexfixedscaling
    }
    #[doc = "Real FIR on 16 bit fractional numbers"]
    #[inline(always)]
    pub fn is_fir(&self) -> bool {
        *self == Leacmd::Fir
    }
    #[doc = "Real vector polynomial calculations 16 args all fractional"]
    #[inline(always)]
    pub fn is_polynomial(&self) -> bool {
        *self == Leacmd::Polynomial
    }
    #[doc = "Real FFT-extension on 16 bit fractional numbers"]
    #[inline(always)]
    pub fn is_fft(&self) -> bool {
        *self == Leacmd::Fft
    }
    #[doc = "Real vector polynomial calculations 32 bit args all fractional"]
    #[inline(always)]
    pub fn is_polynomiallong(&self) -> bool {
        *self == Leacmd::Polynomiallong
    }
    #[doc = "Real row oriented matrix multiply"]
    #[inline(always)]
    pub fn is_mpymatrixrow(&self) -> bool {
        *self == Leacmd::Mpymatrixrow
    }
    #[doc = "Real matrix multiply 16 with 16 to 16 bit fractional"]
    #[inline(always)]
    pub fn is_mpymatrix(&self) -> bool {
        *self == Leacmd::Mpymatrix
    }
    #[doc = "Real point wise matrix add of 16 and 16 to 16 bit number vector"]
    #[inline(always)]
    pub fn is_addmatrix(&self) -> bool {
        *self == Leacmd::Addmatrix
    }
    #[doc = "Real maximum value and position of 16 bit matrices"]
    #[inline(always)]
    pub fn is_maxmatrix(&self) -> bool {
        *self == Leacmd::Maxmatrix
    }
    #[doc = "Real minimum value and position of 16 bit matrices"]
    #[inline(always)]
    pub fn is_minmatrix(&self) -> bool {
        *self == Leacmd::Minmatrix
    }
    #[doc = "Real second order biquad using DF1 with 16 bit fractional"]
    #[inline(always)]
    pub fn is_iirbq1(&self) -> bool {
        *self == Leacmd::Iirbq1
    }
    #[doc = "Real matrix MAC short with 16Bt to 16B fract"]
    #[inline(always)]
    pub fn is_mac(&self) -> bool {
        *self == Leacmd::Mac
    }
    #[doc = "Split 16B vector even to even words"]
    #[inline(always)]
    pub fn is_deinterleaveeveneven(&self) -> bool {
        *self == Leacmd::Deinterleaveeveneven
    }
    #[doc = "Split 16Bt vector even to odd words"]
    #[inline(always)]
    pub fn is_deinterleaveevenodd(&self) -> bool {
        *self == Leacmd::Deinterleaveevenodd
    }
    #[doc = "Split 16B vector odd to even words"]
    #[inline(always)]
    pub fn is_deinterleaveoddeven(&self) -> bool {
        *self == Leacmd::Deinterleaveoddeven
    }
    #[doc = "Split 16B vector odd to odd words"]
    #[inline(always)]
    pub fn is_deinterleaveoddodd(&self) -> bool {
        *self == Leacmd::Deinterleaveoddodd
    }
    #[doc = "Complex Dot Product"]
    #[inline(always)]
    pub fn is_maccomplexmatrix(&self) -> bool {
        *self == Leacmd::Maccomplexmatrix
    }
    #[doc = "Complex conjugate Dot Product"]
    #[inline(always)]
    pub fn is_maccomplexconjugatematrix(&self) -> bool {
        *self == Leacmd::Maccomplexconjugatematrix
    }
    #[doc = "Real point wise matrix Subtraction of 16 and 16 to 16 bit"]
    #[inline(always)]
    pub fn is_submatrix(&self) -> bool {
        *self == Leacmd::Submatrix
    }
    #[doc = "Real point wise matrix multiply 32 with 32 to 32 bit fractional"]
    #[inline(always)]
    pub fn is_mpylongmatrix(&self) -> bool {
        *self == Leacmd::Mpylongmatrix
    }
    #[doc = "Complex point wise matrix multiply complex with complex"]
    #[inline(always)]
    pub fn is_mpycomplexmatrix(&self) -> bool {
        *self == Leacmd::Mpycomplexmatrix
    }
    #[doc = "Real point wise matrix add of 32 and 32 to 32 bit number"]
    #[inline(always)]
    pub fn is_addlongmatrix(&self) -> bool {
        *self == Leacmd::Addlongmatrix
    }
    #[doc = "List move 32 to 32 bit"]
    #[inline(always)]
    pub fn is_movelonglist(&self) -> bool {
        *self == Leacmd::Movelonglist
    }
    #[doc = "Complex bit reversal for 16 bit fractional numbers even"]
    #[inline(always)]
    pub fn is_bitreversecomplexeven(&self) -> bool {
        *self == Leacmd::Bitreversecomplexeven
    }
    #[doc = "Complex bit reversal for 16 bit fractional Numbers odd"]
    #[inline(always)]
    pub fn is_bitreversecomplexodd(&self) -> bool {
        *self == Leacmd::Bitreversecomplexodd
    }
    #[doc = "Real second order biquad using DF2 with 16 bit fractional, extended to include bias and intermediate state min/max"]
    #[inline(always)]
    pub fn is_iirbq2extended(&self) -> bool {
        *self == Leacmd::Iirbq2extended
    }
    #[doc = "Complex FFT on 32B bit fractional numbers, fix scaling"]
    #[inline(always)]
    pub fn is_fftcomplexlong(&self) -> bool {
        *self == Leacmd::Fftcomplexlong
    }
    #[doc = "Real FFT-extension on 32 bit fractional numbers"]
    #[inline(always)]
    pub fn is_fftlong(&self) -> bool {
        *self == Leacmd::Fftlong
    }
    #[doc = "Complex bit reversal for 32 bit fractional numbers even"]
    #[inline(always)]
    pub fn is_bitreversecomplexlongeven(&self) -> bool {
        *self == Leacmd::Bitreversecomplexlongeven
    }
    #[doc = "Complex bit reversal for 16 bit fractional numbers odd"]
    #[inline(always)]
    pub fn is_bitreversecomplexlongodd(&self) -> bool {
        *self == Leacmd::Bitreversecomplexlongodd
    }
    #[doc = "Scalar Polynomial for math on 32bit fractional"]
    #[inline(always)]
    pub fn is_polynomialscalar(&self) -> bool {
        *self == Leacmd::Polynomialscalar
    }
    #[doc = "Complex FFT on 16B bit fractional numbers with auto scaling for enhanced accuracy"]
    #[inline(always)]
    pub fn is_fftcomplexautoscaling(&self) -> bool {
        *self == Leacmd::Fftcomplexautoscaling
    }
    #[doc = "Real FIR on 32 bit fractional numbers"]
    #[inline(always)]
    pub fn is_firlong(&self) -> bool {
        *self == Leacmd::Firlong
    }
    #[doc = "Real block MAC on 32B fractional numbers"]
    #[inline(always)]
    pub fn is_maclongmatrix(&self) -> bool {
        *self == Leacmd::Maclongmatrix
    }
    #[doc = "Real point wise matrix Subtraction of 32 and 32 to 32 bit"]
    #[inline(always)]
    pub fn is_sublongmatrix(&self) -> bool {
        *self == Leacmd::Sublongmatrix
    }
    #[doc = "Real maximum value and position of signed 32B matrices"]
    #[inline(always)]
    pub fn is_maxlongmatrix(&self) -> bool {
        *self == Leacmd::Maxlongmatrix
    }
    #[doc = "Real minimum value and position of signed 32B matrices"]
    #[inline(always)]
    pub fn is_minlongmatrix(&self) -> bool {
        *self == Leacmd::Minlongmatrix
    }
    #[doc = "Complex FIR on 16B fractional numbers"]
    #[inline(always)]
    pub fn is_fircomplex(&self) -> bool {
        *self == Leacmd::Fircomplex
    }
    #[doc = "Real maximum value and position of unsigned 16B matrices"]
    #[inline(always)]
    pub fn is_maxunsignedmatrix(&self) -> bool {
        *self == Leacmd::Maxunsignedmatrix
    }
    #[doc = "Real minimum value and position of unsigned 32B matrices"]
    #[inline(always)]
    pub fn is_minunsignedmatrix(&self) -> bool {
        *self == Leacmd::Minunsignedmatrix
    }
    #[doc = "Real Matrix MAC on 16B fractional"]
    #[inline(always)]
    pub fn is_macmatrix(&self) -> bool {
        *self == Leacmd::Macmatrix
    }
    #[doc = "Vector maximum on 16B signed numbers"]
    #[inline(always)]
    pub fn is_max(&self) -> bool {
        *self == Leacmd::Max
    }
    #[doc = "Vector minimum on 16B signed numbers"]
    #[inline(always)]
    pub fn is_min(&self) -> bool {
        *self == Leacmd::Min
    }
    #[doc = "Vector maximum on 16B unsigned numbers"]
    #[inline(always)]
    pub fn is_maxunsigned(&self) -> bool {
        *self == Leacmd::Maxunsigned
    }
    #[doc = "Vector minimum on 16B unsigned numbers"]
    #[inline(always)]
    pub fn is_minunsigned(&self) -> bool {
        *self == Leacmd::Minunsigned
    }
    #[doc = "Matrix maximum on 32B unsigned numbers"]
    #[inline(always)]
    pub fn is_maxunsignedlongmatrix(&self) -> bool {
        *self == Leacmd::Maxunsignedlongmatrix
    }
    #[doc = "Matrix minimum on 32B unsigned numbers"]
    #[inline(always)]
    pub fn is_minunsignedlongmatrix(&self) -> bool {
        *self == Leacmd::Minunsignedlongmatrix
    }
    #[doc = "Real second order biquad using DF2 with 16 bit fractional"]
    #[inline(always)]
    pub fn is_iirbq2(&self) -> bool {
        *self == Leacmd::Iirbq2
    }
    #[doc = "Complex FIR on 32B fractional numbers"]
    #[inline(always)]
    pub fn is_fircomplexlong(&self) -> bool {
        *self == Leacmd::Fircomplexlong
    }
    #[doc = "Split Function on 32B Vectors/Matrices"]
    #[inline(always)]
    pub fn is_deinterleavelong(&self) -> bool {
        *self == Leacmd::Deinterleavelong
    }
    #[doc = "In-place symmetrical window on 16B fractional numbers"]
    #[inline(always)]
    pub fn is_window(&self) -> bool {
        *self == Leacmd::Window
    }
    #[doc = "Vector MAC at three points, real 16-bit with 32-bit result"]
    #[inline(always)]
    pub fn is_mac3(&self) -> bool {
        *self == Leacmd::Mac3
    }
    #[doc = "Scaled vector multiply and accumulate (MAC)"]
    #[inline(always)]
    pub fn is_scaledmac(&self) -> bool {
        *self == Leacmd::Scaledmac
    }
    #[doc = "Scaled FIR, 16-bit real fractional numbers"]
    #[inline(always)]
    pub fn is_scaledfir(&self) -> bool {
        *self == Leacmd::Scaledfir
    }
}
impl R {
    #[doc = "Bits 0:1 - LEA instruction handshake synchronization type flags"]
    #[inline(always)]
    pub fn leaitflg(&self) -> LeaitflgR {
        LeaitflgR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:9 - These bits represent the LEA command to be invoked. See also the command table"]
    #[inline(always)]
    pub fn leacmd(&self) -> LeacmdR {
        LeacmdR::new(((self.bits >> 2) & 0xff) as u8)
    }
}
impl W {}
#[doc = "LEA Command Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacmdstat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacmdstat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeacmdstatSpec;
impl crate::RegisterSpec for LeacmdstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacmdstat::R`](R) reader structure"]
impl crate::Readable for LeacmdstatSpec {}
#[doc = "`write(|w| ..)` method takes [`leacmdstat::W`](W) writer structure"]
impl crate::Writable for LeacmdstatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACMDSTAT to value 0"]
impl crate::Resettable for LeacmdstatSpec {}
