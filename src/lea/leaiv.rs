#[doc = "Register `LEAIV` reader"]
pub type R = crate::R<LeaivSpec>;
#[doc = "Register `LEAIV` writer"]
pub type W = crate::W<LeaivSpec>;
#[doc = "LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leaiv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: LEA command overflow"]
    Covlifg = 2,
    #[doc = "4: LEA timer interrupt"]
    Tifg = 4,
    #[doc = "6: LEA out of range interrupt"]
    Oorifg = 6,
    #[doc = "8: LEA scalar data inconsistency"]
    Sdiifg = 8,
    #[doc = "10: PMCMD complete interrupt"]
    Pmcmdifg = 10,
}
impl From<Leaiv> for u8 {
    #[inline(always)]
    fn from(variant: Leaiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leaiv {
    type Ux = u8;
}
impl crate::IsEnum for Leaiv {}
#[doc = "Field `LEAIV` reader - LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary"]
pub type LeaivR = crate::FieldReader<Leaiv>;
impl LeaivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Leaiv> {
        match self.bits {
            0 => Some(Leaiv::None),
            2 => Some(Leaiv::Covlifg),
            4 => Some(Leaiv::Tifg),
            6 => Some(Leaiv::Oorifg),
            8 => Some(Leaiv::Sdiifg),
            10 => Some(Leaiv::Pmcmdifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Leaiv::None
    }
    #[doc = "LEA command overflow"]
    #[inline(always)]
    pub fn is_covlifg(&self) -> bool {
        *self == Leaiv::Covlifg
    }
    #[doc = "LEA timer interrupt"]
    #[inline(always)]
    pub fn is_tifg(&self) -> bool {
        *self == Leaiv::Tifg
    }
    #[doc = "LEA out of range interrupt"]
    #[inline(always)]
    pub fn is_oorifg(&self) -> bool {
        *self == Leaiv::Oorifg
    }
    #[doc = "LEA scalar data inconsistency"]
    #[inline(always)]
    pub fn is_sdiifg(&self) -> bool {
        *self == Leaiv::Sdiifg
    }
    #[doc = "PMCMD complete interrupt"]
    #[inline(always)]
    pub fn is_pmcmdifg(&self) -> bool {
        *self == Leaiv::Pmcmdifg
    }
}
#[doc = "Field `LEAIV` writer - LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary"]
pub type LeaivW<'a, REG> = crate::FieldWriter<'a, REG, 8, Leaiv>;
impl<'a, REG> LeaivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Leaiv::None)
    }
    #[doc = "LEA command overflow"]
    #[inline(always)]
    pub fn covlifg(self) -> &'a mut crate::W<REG> {
        self.variant(Leaiv::Covlifg)
    }
    #[doc = "LEA timer interrupt"]
    #[inline(always)]
    pub fn tifg(self) -> &'a mut crate::W<REG> {
        self.variant(Leaiv::Tifg)
    }
    #[doc = "LEA out of range interrupt"]
    #[inline(always)]
    pub fn oorifg(self) -> &'a mut crate::W<REG> {
        self.variant(Leaiv::Oorifg)
    }
    #[doc = "LEA scalar data inconsistency"]
    #[inline(always)]
    pub fn sdiifg(self) -> &'a mut crate::W<REG> {
        self.variant(Leaiv::Sdiifg)
    }
    #[doc = "PMCMD complete interrupt"]
    #[inline(always)]
    pub fn pmcmdifg(self) -> &'a mut crate::W<REG> {
        self.variant(Leaiv::Pmcmdifg)
    }
}
impl R {
    #[doc = "Bits 0:7 - LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary"]
    #[inline(always)]
    pub fn leaiv(&self) -> LeaivR {
        LeaivR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LEA interrupt vector. This is a generated value that can be used as address offset for fast interrupt service routine handling. Reading this register clears the highest pending LEA interrupt (displaying this register with an IDE does not affect its content). Writing to this register clears al pending interrupt flags.This value is always aligned to a 20 bit address offset boundary"]
    #[inline(always)]
    pub fn leaiv(&mut self) -> LeaivW<LeaivSpec> {
        LeaivW::new(self, 0)
    }
}
#[doc = "Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeaivSpec;
impl crate::RegisterSpec for LeaivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leaiv::R`](R) reader structure"]
impl crate::Readable for LeaivSpec {}
#[doc = "`write(|w| ..)` method takes [`leaiv::W`](W) writer structure"]
impl crate::Writable for LeaivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAIV to value 0"]
impl crate::Resettable for LeaivSpec {}
