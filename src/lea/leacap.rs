#[doc = "Register `LEACAP` reader"]
pub type R = crate::R<LeacapSpec>;
#[doc = "Register `LEACAP` writer"]
pub type W = crate::W<LeacapSpec>;
#[doc = "LEA Code Memory Size. This register identifies the size of available code RAM.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leamsiz {
    #[doc = "0: no code RAM"]
    Leamsiz0 = 0,
    #[doc = "1: 1KB Code RAM"]
    Leamsiz1 = 1,
}
impl From<Leamsiz> for u8 {
    #[inline(always)]
    fn from(variant: Leamsiz) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leamsiz {
    type Ux = u8;
}
impl crate::IsEnum for Leamsiz {}
#[doc = "Field `LEAMSIZ` reader - LEA Code Memory Size. This register identifies the size of available code RAM."]
pub type LeamsizR = crate::FieldReader<Leamsiz>;
impl LeamsizR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Leamsiz> {
        match self.bits {
            0 => Some(Leamsiz::Leamsiz0),
            1 => Some(Leamsiz::Leamsiz1),
            _ => None,
        }
    }
    #[doc = "no code RAM"]
    #[inline(always)]
    pub fn is_leamsiz_0(&self) -> bool {
        *self == Leamsiz::Leamsiz0
    }
    #[doc = "1KB Code RAM"]
    #[inline(always)]
    pub fn is_leamsiz_1(&self) -> bool {
        *self == Leamsiz::Leamsiz1
    }
}
impl R {
    #[doc = "Bits 0:3 - LEA Code Memory Size. This register identifies the size of available code RAM."]
    #[inline(always)]
    pub fn leamsiz(&self) -> LeamsizR {
        LeamsizR::new((self.bits & 0x0f) as u8)
    }
}
impl W {}
#[doc = "LEA Capability Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacap::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacap::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeacapSpec;
impl crate::RegisterSpec for LeacapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacap::R`](R) reader structure"]
impl crate::Readable for LeacapSpec {}
#[doc = "`write(|w| ..)` method takes [`leacap::W`](W) writer structure"]
impl crate::Writable for LeacapSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACAP to value 0"]
impl crate::Resettable for LeacapSpec {}
