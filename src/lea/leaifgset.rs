#[doc = "Register `LEAIFGSET` reader"]
pub type R = crate::R<LeaifgsetSpec>;
#[doc = "Register `LEAIFGSET` writer"]
pub type W = crate::W<LeaifgsetSpec>;
#[doc = "LEA command overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacovlis {
    #[doc = "0: No interrupt pending"]
    Leacovlis0 = 0,
    #[doc = "1: Interrupt pending"]
    Leacovlis1 = 1,
}
impl From<Leacovlis> for bool {
    #[inline(always)]
    fn from(variant: Leacovlis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACOVLIS` reader - LEA command overflow interrupt flag"]
pub type LeacovlisR = crate::BitReader<Leacovlis>;
impl LeacovlisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacovlis {
        match self.bits {
            false => Leacovlis::Leacovlis0,
            true => Leacovlis::Leacovlis1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leacovlis_0(&self) -> bool {
        *self == Leacovlis::Leacovlis0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leacovlis_1(&self) -> bool {
        *self == Leacovlis::Leacovlis1
    }
}
#[doc = "Field `LEACOVLIS` writer - LEA command overflow interrupt flag"]
pub type LeacovlisW<'a, REG> = crate::BitWriter<'a, REG, Leacovlis>;
impl<'a, REG> LeacovlisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leacovlis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacovlis::Leacovlis0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leacovlis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacovlis::Leacovlis1)
    }
}
#[doc = "LEA timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leatis {
    #[doc = "0: No interrupt pending"]
    Leatis0 = 0,
    #[doc = "1: Interrupt pending"]
    Leatis1 = 1,
}
impl From<Leatis> for bool {
    #[inline(always)]
    fn from(variant: Leatis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEATIS` reader - LEA timer interrupt flag"]
pub type LeatisR = crate::BitReader<Leatis>;
impl LeatisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leatis {
        match self.bits {
            false => Leatis::Leatis0,
            true => Leatis::Leatis1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leatis_0(&self) -> bool {
        *self == Leatis::Leatis0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leatis_1(&self) -> bool {
        *self == Leatis::Leatis1
    }
}
#[doc = "Field `LEATIS` writer - LEA timer interrupt flag"]
pub type LeatisW<'a, REG> = crate::BitWriter<'a, REG, Leatis>;
impl<'a, REG> LeatisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leatis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leatis::Leatis0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leatis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leatis::Leatis1)
    }
}
#[doc = "LEA out of address range interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaooris {
    #[doc = "0: No interrupt pending"]
    Leaooris0 = 0,
    #[doc = "1: Interrupt pending"]
    Leaooris1 = 1,
}
impl From<Leaooris> for bool {
    #[inline(always)]
    fn from(variant: Leaooris) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAOORIS` reader - LEA out of address range interrupt flag."]
pub type LeaoorisR = crate::BitReader<Leaooris>;
impl LeaoorisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leaooris {
        match self.bits {
            false => Leaooris::Leaooris0,
            true => Leaooris::Leaooris1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leaooris_0(&self) -> bool {
        *self == Leaooris::Leaooris0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leaooris_1(&self) -> bool {
        *self == Leaooris::Leaooris1
    }
}
#[doc = "Field `LEAOORIS` writer - LEA out of address range interrupt flag."]
pub type LeaoorisW<'a, REG> = crate::BitWriter<'a, REG, Leaooris>;
impl<'a, REG> LeaoorisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leaooris_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leaooris::Leaooris0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leaooris_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leaooris::Leaooris1)
    }
}
#[doc = "LEA scalar data inconsistency interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leasdiis {
    #[doc = "0: No interrupt pending"]
    Leasdiis0 = 0,
    #[doc = "1: Interrupt pending"]
    Leasdiis1 = 1,
}
impl From<Leasdiis> for bool {
    #[inline(always)]
    fn from(variant: Leasdiis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEASDIIS` reader - LEA scalar data inconsistency interrupt flag"]
pub type LeasdiisR = crate::BitReader<Leasdiis>;
impl LeasdiisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leasdiis {
        match self.bits {
            false => Leasdiis::Leasdiis0,
            true => Leasdiis::Leasdiis1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leasdiis_0(&self) -> bool {
        *self == Leasdiis::Leasdiis0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leasdiis_1(&self) -> bool {
        *self == Leasdiis::Leasdiis1
    }
}
#[doc = "Field `LEASDIIS` writer - LEA scalar data inconsistency interrupt flag"]
pub type LeasdiisW<'a, REG> = crate::BitWriter<'a, REG, Leasdiis>;
impl<'a, REG> LeasdiisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leasdiis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leasdiis::Leasdiis0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leasdiis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leasdiis::Leasdiis1)
    }
}
#[doc = "PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leapmcmdis {
    #[doc = "0: No interrupt pending"]
    Leapmcmdis0 = 0,
    #[doc = "1: Interrupt pending"]
    Leapmcmdis1 = 1,
}
impl From<Leapmcmdis> for bool {
    #[inline(always)]
    fn from(variant: Leapmcmdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAPMCMDIS` reader - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag."]
pub type LeapmcmdisR = crate::BitReader<Leapmcmdis>;
impl LeapmcmdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leapmcmdis {
        match self.bits {
            false => Leapmcmdis::Leapmcmdis0,
            true => Leapmcmdis::Leapmcmdis1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leapmcmdis_0(&self) -> bool {
        *self == Leapmcmdis::Leapmcmdis0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leapmcmdis_1(&self) -> bool {
        *self == Leapmcmdis::Leapmcmdis1
    }
}
#[doc = "Field `LEAPMCMDIS` writer - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag."]
pub type LeapmcmdisW<'a, REG> = crate::BitWriter<'a, REG, Leapmcmdis>;
impl<'a, REG> LeapmcmdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdis_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leapmcmdis::Leapmcmdis0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdis_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leapmcmdis::Leapmcmdis1)
    }
}
impl R {
    #[doc = "Bit 0 - LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlis(&self) -> LeacovlisR {
        LeacovlisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatis(&self) -> LeatisR {
        LeatisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaooris(&self) -> LeaoorisR {
        LeaoorisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiis(&self) -> LeasdiisR {
        LeasdiisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdis(&self) -> LeapmcmdisR {
        LeapmcmdisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlis(&mut self) -> LeacovlisW<LeaifgsetSpec> {
        LeacovlisW::new(self, 0)
    }
    #[doc = "Bit 1 - LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatis(&mut self) -> LeatisW<LeaifgsetSpec> {
        LeatisW::new(self, 1)
    }
    #[doc = "Bit 2 - LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaooris(&mut self) -> LeaoorisW<LeaifgsetSpec> {
        LeaoorisW::new(self, 2)
    }
    #[doc = "Bit 3 - LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiis(&mut self) -> LeasdiisW<LeaifgsetSpec> {
        LeasdiisW::new(self, 3)
    }
    #[doc = "Bit 4 - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdis(&mut self) -> LeapmcmdisW<LeaifgsetSpec> {
        LeapmcmdisW::new(self, 4)
    }
}
#[doc = "Interrupt Flag and Set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaifgset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaifgset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeaifgsetSpec;
impl crate::RegisterSpec for LeaifgsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leaifgset::R`](R) reader structure"]
impl crate::Readable for LeaifgsetSpec {}
#[doc = "`write(|w| ..)` method takes [`leaifgset::W`](W) writer structure"]
impl crate::Writable for LeaifgsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAIFGSET to value 0"]
impl crate::Resettable for LeaifgsetSpec {}
