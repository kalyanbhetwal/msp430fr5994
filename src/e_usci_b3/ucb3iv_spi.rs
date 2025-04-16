#[doc = "Register `UCB3IV_SPI` reader"]
pub type R = crate::R<Ucb3ivSpiSpec>;
#[doc = "Register `UCB3IV_SPI` writer"]
pub type W = crate::W<Ucb3ivSpiSpec>;
#[doc = "eUSCI_B interrupt vector value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Uciv {
    #[doc = "0: No interrupt pending"]
    None = 0,
    #[doc = "2: Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    Ucrxifg = 2,
    #[doc = "4: Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    Uctxifg = 4,
}
impl From<Uciv> for u16 {
    #[inline(always)]
    fn from(variant: Uciv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Uciv {
    type Ux = u16;
}
impl crate::IsEnum for Uciv {}
#[doc = "Field `UCIV` reader - eUSCI_B interrupt vector value"]
pub type UcivR = crate::FieldReader<Uciv>;
impl UcivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Uciv> {
        match self.bits {
            0 => Some(Uciv::None),
            2 => Some(Uciv::Ucrxifg),
            4 => Some(Uciv::Uctxifg),
            _ => None,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Uciv::None
    }
    #[doc = "Interrupt Source: Data received; Interrupt Flag: UCRXIFG; Interrupt Priority: Highest"]
    #[inline(always)]
    pub fn is_ucrxifg(&self) -> bool {
        *self == Uciv::Ucrxifg
    }
    #[doc = "Interrupt Source: Transmit buffer empty; Interrupt Flag: UCTXIFG; Interrupt Priority: Lowest"]
    #[inline(always)]
    pub fn is_uctxifg(&self) -> bool {
        *self == Uciv::Uctxifg
    }
}
impl R {
    #[doc = "Bits 0:15 - eUSCI_B interrupt vector value"]
    #[inline(always)]
    pub fn uciv(&self) -> UcivR {
        UcivR::new(self.bits)
    }
}
impl W {}
#[doc = "eUSCI_Bx Interrupt Vector Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ucb3iv_spi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ucb3iv_spi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ucb3ivSpiSpec;
impl crate::RegisterSpec for Ucb3ivSpiSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ucb3iv_spi::R`](R) reader structure"]
impl crate::Readable for Ucb3ivSpiSpec {}
#[doc = "`write(|w| ..)` method takes [`ucb3iv_spi::W`](W) writer structure"]
impl crate::Writable for Ucb3ivSpiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UCB3IV_SPI to value 0"]
impl crate::Resettable for Ucb3ivSpiSpec {}
