#[doc = "Register `LEAIE` reader"]
pub type R = crate::R<LeaieSpec>;
#[doc = "Register `LEAIE` writer"]
pub type W = crate::W<LeaieSpec>;
#[doc = "LEA command overflow interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacovlie {
    #[doc = "0: Interrupt disabled"]
    Leacovlie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Leacovlie1 = 1,
}
impl From<Leacovlie> for bool {
    #[inline(always)]
    fn from(variant: Leacovlie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACOVLIE` reader - LEA command overflow interrupt enable"]
pub type LeacovlieR = crate::BitReader<Leacovlie>;
impl LeacovlieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacovlie {
        match self.bits {
            false => Leacovlie::Leacovlie0,
            true => Leacovlie::Leacovlie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_leacovlie_0(&self) -> bool {
        *self == Leacovlie::Leacovlie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_leacovlie_1(&self) -> bool {
        *self == Leacovlie::Leacovlie1
    }
}
#[doc = "Field `LEACOVLIE` writer - LEA command overflow interrupt enable"]
pub type LeacovlieW<'a, REG> = crate::BitWriter<'a, REG, Leacovlie>;
impl<'a, REG> LeacovlieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leacovlie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacovlie::Leacovlie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leacovlie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacovlie::Leacovlie1)
    }
}
#[doc = "LEA timer event interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leatie {
    #[doc = "0: Interrupt disabled"]
    Leatie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Leatie1 = 1,
}
impl From<Leatie> for bool {
    #[inline(always)]
    fn from(variant: Leatie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEATIE` reader - LEA timer event interrupt enable"]
pub type LeatieR = crate::BitReader<Leatie>;
impl LeatieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leatie {
        match self.bits {
            false => Leatie::Leatie0,
            true => Leatie::Leatie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_leatie_0(&self) -> bool {
        *self == Leatie::Leatie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_leatie_1(&self) -> bool {
        *self == Leatie::Leatie1
    }
}
#[doc = "Field `LEATIE` writer - LEA timer event interrupt enable"]
pub type LeatieW<'a, REG> = crate::BitWriter<'a, REG, Leatie>;
impl<'a, REG> LeatieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leatie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leatie::Leatie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leatie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leatie::Leatie1)
    }
}
#[doc = "LEA out of address range interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaoorie {
    #[doc = "0: Interrupt disabled"]
    Leaoorie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Leaoorie1 = 1,
}
impl From<Leaoorie> for bool {
    #[inline(always)]
    fn from(variant: Leaoorie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAOORIE` reader - LEA out of address range interrupt enable."]
pub type LeaoorieR = crate::BitReader<Leaoorie>;
impl LeaoorieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leaoorie {
        match self.bits {
            false => Leaoorie::Leaoorie0,
            true => Leaoorie::Leaoorie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_leaoorie_0(&self) -> bool {
        *self == Leaoorie::Leaoorie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_leaoorie_1(&self) -> bool {
        *self == Leaoorie::Leaoorie1
    }
}
#[doc = "Field `LEAOORIE` writer - LEA out of address range interrupt enable."]
pub type LeaoorieW<'a, REG> = crate::BitWriter<'a, REG, Leaoorie>;
impl<'a, REG> LeaoorieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leaoorie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leaoorie::Leaoorie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leaoorie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leaoorie::Leaoorie1)
    }
}
#[doc = "LEA scalar data inconsistency interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leasdiie {
    #[doc = "0: Interrupt disabled"]
    Leasdiie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Leasdiie1 = 1,
}
impl From<Leasdiie> for bool {
    #[inline(always)]
    fn from(variant: Leasdiie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEASDIIE` reader - LEA scalar data inconsistency interrupt enable"]
pub type LeasdiieR = crate::BitReader<Leasdiie>;
impl LeasdiieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leasdiie {
        match self.bits {
            false => Leasdiie::Leasdiie0,
            true => Leasdiie::Leasdiie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_leasdiie_0(&self) -> bool {
        *self == Leasdiie::Leasdiie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_leasdiie_1(&self) -> bool {
        *self == Leasdiie::Leasdiie1
    }
}
#[doc = "Field `LEASDIIE` writer - LEA scalar data inconsistency interrupt enable"]
pub type LeasdiieW<'a, REG> = crate::BitWriter<'a, REG, Leasdiie>;
impl<'a, REG> LeasdiieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leasdiie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leasdiie::Leasdiie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leasdiie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leasdiie::Leasdiie1)
    }
}
#[doc = "PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leapmcmdie {
    #[doc = "0: Interrupt disabled"]
    Leapmcmdie0 = 0,
    #[doc = "1: Interrupt enabled"]
    Leapmcmdie1 = 1,
}
impl From<Leapmcmdie> for bool {
    #[inline(always)]
    fn from(variant: Leapmcmdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAPMCMDIE` reader - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable."]
pub type LeapmcmdieR = crate::BitReader<Leapmcmdie>;
impl LeapmcmdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leapmcmdie {
        match self.bits {
            false => Leapmcmdie::Leapmcmdie0,
            true => Leapmcmdie::Leapmcmdie1,
        }
    }
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn is_leapmcmdie_0(&self) -> bool {
        *self == Leapmcmdie::Leapmcmdie0
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn is_leapmcmdie_1(&self) -> bool {
        *self == Leapmcmdie::Leapmcmdie1
    }
}
#[doc = "Field `LEAPMCMDIE` writer - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable."]
pub type LeapmcmdieW<'a, REG> = crate::BitWriter<'a, REG, Leapmcmdie>;
impl<'a, REG> LeapmcmdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn leapmcmdie_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leapmcmdie::Leapmcmdie0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn leapmcmdie_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leapmcmdie::Leapmcmdie1)
    }
}
impl R {
    #[doc = "Bit 0 - LEA command overflow interrupt enable"]
    #[inline(always)]
    pub fn leacovlie(&self) -> LeacovlieR {
        LeacovlieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LEA timer event interrupt enable"]
    #[inline(always)]
    pub fn leatie(&self) -> LeatieR {
        LeatieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEA out of address range interrupt enable."]
    #[inline(always)]
    pub fn leaoorie(&self) -> LeaoorieR {
        LeaoorieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LEA scalar data inconsistency interrupt enable"]
    #[inline(always)]
    pub fn leasdiie(&self) -> LeasdiieR {
        LeasdiieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable."]
    #[inline(always)]
    pub fn leapmcmdie(&self) -> LeapmcmdieR {
        LeapmcmdieR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LEA command overflow interrupt enable"]
    #[inline(always)]
    pub fn leacovlie(&mut self) -> LeacovlieW<LeaieSpec> {
        LeacovlieW::new(self, 0)
    }
    #[doc = "Bit 1 - LEA timer event interrupt enable"]
    #[inline(always)]
    pub fn leatie(&mut self) -> LeatieW<LeaieSpec> {
        LeatieW::new(self, 1)
    }
    #[doc = "Bit 2 - LEA out of address range interrupt enable."]
    #[inline(always)]
    pub fn leaoorie(&mut self) -> LeaoorieW<LeaieSpec> {
        LeaoorieW::new(self, 2)
    }
    #[doc = "Bit 3 - LEA scalar data inconsistency interrupt enable"]
    #[inline(always)]
    pub fn leasdiie(&mut self) -> LeasdiieW<LeaieSpec> {
        LeasdiieW::new(self, 3)
    }
    #[doc = "Bit 4 - PMCMD as soon hardware trigger is avail. Peripheral memory triggered Command completed interrupt enable."]
    #[inline(always)]
    pub fn leapmcmdie(&mut self) -> LeapmcmdieW<LeaieSpec> {
        LeapmcmdieW::new(self, 4)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeaieSpec;
impl crate::RegisterSpec for LeaieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leaie::R`](R) reader structure"]
impl crate::Readable for LeaieSpec {}
#[doc = "`write(|w| ..)` method takes [`leaie::W`](W) writer structure"]
impl crate::Writable for LeaieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAIE to value 0"]
impl crate::Resettable for LeaieSpec {}
