#[doc = "Register `LEAIFG` reader"]
pub type R = crate::R<LeaifgSpec>;
#[doc = "Register `LEAIFG` writer"]
pub type W = crate::W<LeaifgSpec>;
#[doc = "LEA command overflow interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacovlifg {
    #[doc = "0: No interrupt pending"]
    Leacovlifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Leacovlifg1 = 1,
}
impl From<Leacovlifg> for bool {
    #[inline(always)]
    fn from(variant: Leacovlifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACOVLIFG` reader - LEA command overflow interrupt flag"]
pub type LeacovlifgR = crate::BitReader<Leacovlifg>;
impl LeacovlifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacovlifg {
        match self.bits {
            false => Leacovlifg::Leacovlifg0,
            true => Leacovlifg::Leacovlifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leacovlifg_0(&self) -> bool {
        *self == Leacovlifg::Leacovlifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leacovlifg_1(&self) -> bool {
        *self == Leacovlifg::Leacovlifg1
    }
}
#[doc = "Field `LEACOVLIFG` writer - LEA command overflow interrupt flag"]
pub type LeacovlifgW<'a, REG> = crate::BitWriter<'a, REG, Leacovlifg>;
impl<'a, REG> LeacovlifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leacovlifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacovlifg::Leacovlifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leacovlifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacovlifg::Leacovlifg1)
    }
}
#[doc = "LEA timer interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leatifg {
    #[doc = "0: No interrupt pending"]
    Leatifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Leatifg1 = 1,
}
impl From<Leatifg> for bool {
    #[inline(always)]
    fn from(variant: Leatifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEATIFG` reader - LEA timer interrupt flag"]
pub type LeatifgR = crate::BitReader<Leatifg>;
impl LeatifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leatifg {
        match self.bits {
            false => Leatifg::Leatifg0,
            true => Leatifg::Leatifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leatifg_0(&self) -> bool {
        *self == Leatifg::Leatifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leatifg_1(&self) -> bool {
        *self == Leatifg::Leatifg1
    }
}
#[doc = "Field `LEATIFG` writer - LEA timer interrupt flag"]
pub type LeatifgW<'a, REG> = crate::BitWriter<'a, REG, Leatifg>;
impl<'a, REG> LeatifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leatifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leatifg::Leatifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leatifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leatifg::Leatifg1)
    }
}
#[doc = "LEA out of address range interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leaoorifg {
    #[doc = "0: No interrupt pending"]
    Leaoorifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Leaoorifg1 = 1,
}
impl From<Leaoorifg> for bool {
    #[inline(always)]
    fn from(variant: Leaoorifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAOORIFG` reader - LEA out of address range interrupt flag."]
pub type LeaoorifgR = crate::BitReader<Leaoorifg>;
impl LeaoorifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leaoorifg {
        match self.bits {
            false => Leaoorifg::Leaoorifg0,
            true => Leaoorifg::Leaoorifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leaoorifg_0(&self) -> bool {
        *self == Leaoorifg::Leaoorifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leaoorifg_1(&self) -> bool {
        *self == Leaoorifg::Leaoorifg1
    }
}
#[doc = "Field `LEAOORIFG` writer - LEA out of address range interrupt flag."]
pub type LeaoorifgW<'a, REG> = crate::BitWriter<'a, REG, Leaoorifg>;
impl<'a, REG> LeaoorifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leaoorifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leaoorifg::Leaoorifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leaoorifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leaoorifg::Leaoorifg1)
    }
}
#[doc = "LEA scalar data inconsistency interrupt flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leasdiifg {
    #[doc = "0: No interrupt pending"]
    Leasdiifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Leasdiifg1 = 1,
}
impl From<Leasdiifg> for bool {
    #[inline(always)]
    fn from(variant: Leasdiifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEASDIIFG` reader - LEA scalar data inconsistency interrupt flag"]
pub type LeasdiifgR = crate::BitReader<Leasdiifg>;
impl LeasdiifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leasdiifg {
        match self.bits {
            false => Leasdiifg::Leasdiifg0,
            true => Leasdiifg::Leasdiifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leasdiifg_0(&self) -> bool {
        *self == Leasdiifg::Leasdiifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leasdiifg_1(&self) -> bool {
        *self == Leasdiifg::Leasdiifg1
    }
}
#[doc = "Field `LEASDIIFG` writer - LEA scalar data inconsistency interrupt flag"]
pub type LeasdiifgW<'a, REG> = crate::BitWriter<'a, REG, Leasdiifg>;
impl<'a, REG> LeasdiifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leasdiifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leasdiifg::Leasdiifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leasdiifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leasdiifg::Leasdiifg1)
    }
}
#[doc = "PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leapmcmdifg {
    #[doc = "0: No interrupt pending"]
    Leapmcmdifg0 = 0,
    #[doc = "1: Interrupt pending"]
    Leapmcmdifg1 = 1,
}
impl From<Leapmcmdifg> for bool {
    #[inline(always)]
    fn from(variant: Leapmcmdifg) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAPMCMDIFG` reader - PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag."]
pub type LeapmcmdifgR = crate::BitReader<Leapmcmdifg>;
impl LeapmcmdifgR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leapmcmdifg {
        match self.bits {
            false => Leapmcmdifg::Leapmcmdifg0,
            true => Leapmcmdifg::Leapmcmdifg1,
        }
    }
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn is_leapmcmdifg_0(&self) -> bool {
        *self == Leapmcmdifg::Leapmcmdifg0
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn is_leapmcmdifg_1(&self) -> bool {
        *self == Leapmcmdifg::Leapmcmdifg1
    }
}
#[doc = "Field `LEAPMCMDIFG` writer - PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag."]
pub type LeapmcmdifgW<'a, REG> = crate::BitWriter<'a, REG, Leapmcmdifg>;
impl<'a, REG> LeapmcmdifgW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdifg_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leapmcmdifg::Leapmcmdifg0)
    }
    #[doc = "Interrupt pending"]
    #[inline(always)]
    pub fn leapmcmdifg_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leapmcmdifg::Leapmcmdifg1)
    }
}
impl R {
    #[doc = "Bit 0 - LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlifg(&self) -> LeacovlifgR {
        LeacovlifgR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatifg(&self) -> LeatifgR {
        LeatifgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaoorifg(&self) -> LeaoorifgR {
        LeaoorifgR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiifg(&self) -> LeasdiifgR {
        LeasdiifgR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdifg(&self) -> LeapmcmdifgR {
        LeapmcmdifgR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LEA command overflow interrupt flag"]
    #[inline(always)]
    pub fn leacovlifg(&mut self) -> LeacovlifgW<LeaifgSpec> {
        LeacovlifgW::new(self, 0)
    }
    #[doc = "Bit 1 - LEA timer interrupt flag"]
    #[inline(always)]
    pub fn leatifg(&mut self) -> LeatifgW<LeaifgSpec> {
        LeatifgW::new(self, 1)
    }
    #[doc = "Bit 2 - LEA out of address range interrupt flag."]
    #[inline(always)]
    pub fn leaoorifg(&mut self) -> LeaoorifgW<LeaifgSpec> {
        LeaoorifgW::new(self, 2)
    }
    #[doc = "Bit 3 - LEA scalar data inconsistency interrupt flag"]
    #[inline(always)]
    pub fn leasdiifg(&mut self) -> LeasdiifgW<LeaifgSpec> {
        LeasdiifgW::new(self, 3)
    }
    #[doc = "Bit 4 - PMCMD when hardware trigger is available. Peripheral memory triggered Command completed interrupt flag."]
    #[inline(always)]
    pub fn leapmcmdifg(&mut self) -> LeapmcmdifgW<LeaifgSpec> {
        LeapmcmdifgW::new(self, 4)
    }
}
#[doc = "Interrupt Flag and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leaifg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leaifg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeaifgSpec;
impl crate::RegisterSpec for LeaifgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leaifg::R`](R) reader structure"]
impl crate::Readable for LeaifgSpec {}
#[doc = "`write(|w| ..)` method takes [`leaifg::W`](W) writer structure"]
impl crate::Writable for LeaifgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEAIFG to value 0"]
impl crate::Resettable for LeaifgSpec {}
