#[doc = "Register `LEAPMCTL` reader"]
pub type R = crate::R<LeapmctlSpec>;
#[doc = "Register `LEAPMCTL` writer"]
pub type W = crate::W<LeapmctlSpec>;
#[doc = "Command enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacmden {
    #[doc = "0: Command triggering by writing to LEAPMCB is disabled"]
    Leacmden0 = 0,
    #[doc = "1: Command triggering by writing to LEAPMCB is enabled"]
    Leacmden1 = 1,
}
impl From<Leacmden> for bool {
    #[inline(always)]
    fn from(variant: Leacmden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACMDEN` reader - Command enable"]
pub type LeacmdenR = crate::BitReader<Leacmden>;
impl LeacmdenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacmden {
        match self.bits {
            false => Leacmden::Leacmden0,
            true => Leacmden::Leacmden1,
        }
    }
    #[doc = "Command triggering by writing to LEAPMCB is disabled"]
    #[inline(always)]
    pub fn is_leacmden_0(&self) -> bool {
        *self == Leacmden::Leacmden0
    }
    #[doc = "Command triggering by writing to LEAPMCB is enabled"]
    #[inline(always)]
    pub fn is_leacmden_1(&self) -> bool {
        *self == Leacmden::Leacmden1
    }
}
#[doc = "Field `LEACMDEN` writer - Command enable"]
pub type LeacmdenW<'a, REG> = crate::BitWriter<'a, REG, Leacmden>;
impl<'a, REG> LeacmdenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Command triggering by writing to LEAPMCB is disabled"]
    #[inline(always)]
    pub fn leacmden_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacmden::Leacmden0)
    }
    #[doc = "Command triggering by writing to LEAPMCB is enabled"]
    #[inline(always)]
    pub fn leacmden_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacmden::Leacmden1)
    }
}
#[doc = "Field `LEATRG` reader - Command trigger"]
pub type LeatrgR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Command enable"]
    #[inline(always)]
    pub fn leacmden(&self) -> LeacmdenR {
        LeacmdenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Command trigger"]
    #[inline(always)]
    pub fn leatrg(&self) -> LeatrgR {
        LeatrgR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command enable"]
    #[inline(always)]
    pub fn leacmden(&mut self) -> LeacmdenW<LeapmctlSpec> {
        LeacmdenW::new(self, 0)
    }
}
#[doc = "PM Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leapmctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leapmctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeapmctlSpec;
impl crate::RegisterSpec for LeapmctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leapmctl::R`](R) reader structure"]
impl crate::Readable for LeapmctlSpec {}
#[doc = "`write(|w| ..)` method takes [`leapmctl::W`](W) writer structure"]
impl crate::Writable for LeapmctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAPMCTL to value 0"]
impl crate::Resettable for LeapmctlSpec {}
