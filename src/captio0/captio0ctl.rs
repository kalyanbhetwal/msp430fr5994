#[doc = "Register `CAPTIO0CTL` reader"]
pub type R = crate::R<Captio0ctlSpec>;
#[doc = "Register `CAPTIO0CTL` writer"]
pub type W = crate::W<Captio0ctlSpec>;
#[doc = "Capacitive Touch IO pin select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captiopisel0 {
    #[doc = "0: Px.0"]
    Captiopisel0 = 0,
    #[doc = "1: Px.1"]
    Captiopisel1 = 1,
    #[doc = "2: Px.2"]
    Captiopisel2 = 2,
    #[doc = "3: Px.3"]
    Captiopisel3 = 3,
    #[doc = "4: Px.4"]
    Captiopisel4 = 4,
    #[doc = "5: Px.5"]
    Captiopisel5 = 5,
    #[doc = "6: Px.6"]
    Captiopisel6 = 6,
    #[doc = "7: Px.7"]
    Captiopisel7 = 7,
}
impl From<Captiopisel0> for u8 {
    #[inline(always)]
    fn from(variant: Captiopisel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captiopisel0 {
    type Ux = u8;
}
impl crate::IsEnum for Captiopisel0 {}
#[doc = "Field `CAPTIOPISEL0` reader - Capacitive Touch IO pin select"]
pub type Captiopisel0R = crate::FieldReader<Captiopisel0>;
impl Captiopisel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captiopisel0 {
        match self.bits {
            0 => Captiopisel0::Captiopisel0,
            1 => Captiopisel0::Captiopisel1,
            2 => Captiopisel0::Captiopisel2,
            3 => Captiopisel0::Captiopisel3,
            4 => Captiopisel0::Captiopisel4,
            5 => Captiopisel0::Captiopisel5,
            6 => Captiopisel0::Captiopisel6,
            7 => Captiopisel0::Captiopisel7,
            _ => unreachable!(),
        }
    }
    #[doc = "Px.0"]
    #[inline(always)]
    pub fn is_captiopisel_0(&self) -> bool {
        *self == Captiopisel0::Captiopisel0
    }
    #[doc = "Px.1"]
    #[inline(always)]
    pub fn is_captiopisel_1(&self) -> bool {
        *self == Captiopisel0::Captiopisel1
    }
    #[doc = "Px.2"]
    #[inline(always)]
    pub fn is_captiopisel_2(&self) -> bool {
        *self == Captiopisel0::Captiopisel2
    }
    #[doc = "Px.3"]
    #[inline(always)]
    pub fn is_captiopisel_3(&self) -> bool {
        *self == Captiopisel0::Captiopisel3
    }
    #[doc = "Px.4"]
    #[inline(always)]
    pub fn is_captiopisel_4(&self) -> bool {
        *self == Captiopisel0::Captiopisel4
    }
    #[doc = "Px.5"]
    #[inline(always)]
    pub fn is_captiopisel_5(&self) -> bool {
        *self == Captiopisel0::Captiopisel5
    }
    #[doc = "Px.6"]
    #[inline(always)]
    pub fn is_captiopisel_6(&self) -> bool {
        *self == Captiopisel0::Captiopisel6
    }
    #[doc = "Px.7"]
    #[inline(always)]
    pub fn is_captiopisel_7(&self) -> bool {
        *self == Captiopisel0::Captiopisel7
    }
}
#[doc = "Field `CAPTIOPISEL0` writer - Capacitive Touch IO pin select"]
pub type Captiopisel0W<'a, REG> = crate::FieldWriter<'a, REG, 3, Captiopisel0, crate::Safe>;
impl<'a, REG> Captiopisel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Px.0"]
    #[inline(always)]
    pub fn captiopisel_0(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel0)
    }
    #[doc = "Px.1"]
    #[inline(always)]
    pub fn captiopisel_1(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel1)
    }
    #[doc = "Px.2"]
    #[inline(always)]
    pub fn captiopisel_2(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel2)
    }
    #[doc = "Px.3"]
    #[inline(always)]
    pub fn captiopisel_3(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel3)
    }
    #[doc = "Px.4"]
    #[inline(always)]
    pub fn captiopisel_4(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel4)
    }
    #[doc = "Px.5"]
    #[inline(always)]
    pub fn captiopisel_5(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel5)
    }
    #[doc = "Px.6"]
    #[inline(always)]
    pub fn captiopisel_6(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel6)
    }
    #[doc = "Px.7"]
    #[inline(always)]
    pub fn captiopisel_7(self) -> &'a mut crate::W<REG> {
        self.variant(Captiopisel0::Captiopisel7)
    }
}
#[doc = "Capacitive Touch IO port select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Captioposel0 {
    #[doc = "0: Px = PJ"]
    Pj = 0,
    #[doc = "1: Px = P1"]
    P1 = 1,
    #[doc = "2: Px = P2"]
    P2 = 2,
    #[doc = "3: Px = P3"]
    P3 = 3,
    #[doc = "4: Px = P4"]
    P4 = 4,
    #[doc = "5: Px = P5"]
    P5 = 5,
    #[doc = "6: Px = P6"]
    P6 = 6,
    #[doc = "7: Px = P7"]
    P7 = 7,
    #[doc = "8: Px = P8"]
    P8 = 8,
    #[doc = "9: Px = P9"]
    P9 = 9,
    #[doc = "10: Px = P10"]
    P10 = 10,
    #[doc = "11: Px = P11"]
    P11 = 11,
    #[doc = "12: Px = P12"]
    P12 = 12,
    #[doc = "13: Px = P13"]
    P13 = 13,
    #[doc = "14: Px = P14"]
    P14 = 14,
    #[doc = "15: Px = P15"]
    P15 = 15,
}
impl From<Captioposel0> for u8 {
    #[inline(always)]
    fn from(variant: Captioposel0) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Captioposel0 {
    type Ux = u8;
}
impl crate::IsEnum for Captioposel0 {}
#[doc = "Field `CAPTIOPOSEL0` reader - Capacitive Touch IO port select"]
pub type Captioposel0R = crate::FieldReader<Captioposel0>;
impl Captioposel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captioposel0 {
        match self.bits {
            0 => Captioposel0::Pj,
            1 => Captioposel0::P1,
            2 => Captioposel0::P2,
            3 => Captioposel0::P3,
            4 => Captioposel0::P4,
            5 => Captioposel0::P5,
            6 => Captioposel0::P6,
            7 => Captioposel0::P7,
            8 => Captioposel0::P8,
            9 => Captioposel0::P9,
            10 => Captioposel0::P10,
            11 => Captioposel0::P11,
            12 => Captioposel0::P12,
            13 => Captioposel0::P13,
            14 => Captioposel0::P14,
            15 => Captioposel0::P15,
            _ => unreachable!(),
        }
    }
    #[doc = "Px = PJ"]
    #[inline(always)]
    pub fn is_pj(&self) -> bool {
        *self == Captioposel0::Pj
    }
    #[doc = "Px = P1"]
    #[inline(always)]
    pub fn is_p1(&self) -> bool {
        *self == Captioposel0::P1
    }
    #[doc = "Px = P2"]
    #[inline(always)]
    pub fn is_p2(&self) -> bool {
        *self == Captioposel0::P2
    }
    #[doc = "Px = P3"]
    #[inline(always)]
    pub fn is_p3(&self) -> bool {
        *self == Captioposel0::P3
    }
    #[doc = "Px = P4"]
    #[inline(always)]
    pub fn is_p4(&self) -> bool {
        *self == Captioposel0::P4
    }
    #[doc = "Px = P5"]
    #[inline(always)]
    pub fn is_p5(&self) -> bool {
        *self == Captioposel0::P5
    }
    #[doc = "Px = P6"]
    #[inline(always)]
    pub fn is_p6(&self) -> bool {
        *self == Captioposel0::P6
    }
    #[doc = "Px = P7"]
    #[inline(always)]
    pub fn is_p7(&self) -> bool {
        *self == Captioposel0::P7
    }
    #[doc = "Px = P8"]
    #[inline(always)]
    pub fn is_p8(&self) -> bool {
        *self == Captioposel0::P8
    }
    #[doc = "Px = P9"]
    #[inline(always)]
    pub fn is_p9(&self) -> bool {
        *self == Captioposel0::P9
    }
    #[doc = "Px = P10"]
    #[inline(always)]
    pub fn is_p10(&self) -> bool {
        *self == Captioposel0::P10
    }
    #[doc = "Px = P11"]
    #[inline(always)]
    pub fn is_p11(&self) -> bool {
        *self == Captioposel0::P11
    }
    #[doc = "Px = P12"]
    #[inline(always)]
    pub fn is_p12(&self) -> bool {
        *self == Captioposel0::P12
    }
    #[doc = "Px = P13"]
    #[inline(always)]
    pub fn is_p13(&self) -> bool {
        *self == Captioposel0::P13
    }
    #[doc = "Px = P14"]
    #[inline(always)]
    pub fn is_p14(&self) -> bool {
        *self == Captioposel0::P14
    }
    #[doc = "Px = P15"]
    #[inline(always)]
    pub fn is_p15(&self) -> bool {
        *self == Captioposel0::P15
    }
}
#[doc = "Field `CAPTIOPOSEL0` writer - Capacitive Touch IO port select"]
pub type Captioposel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Captioposel0, crate::Safe>;
impl<'a, REG> Captioposel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Px = PJ"]
    #[inline(always)]
    pub fn pj(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::Pj)
    }
    #[doc = "Px = P1"]
    #[inline(always)]
    pub fn p1(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P1)
    }
    #[doc = "Px = P2"]
    #[inline(always)]
    pub fn p2(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P2)
    }
    #[doc = "Px = P3"]
    #[inline(always)]
    pub fn p3(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P3)
    }
    #[doc = "Px = P4"]
    #[inline(always)]
    pub fn p4(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P4)
    }
    #[doc = "Px = P5"]
    #[inline(always)]
    pub fn p5(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P5)
    }
    #[doc = "Px = P6"]
    #[inline(always)]
    pub fn p6(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P6)
    }
    #[doc = "Px = P7"]
    #[inline(always)]
    pub fn p7(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P7)
    }
    #[doc = "Px = P8"]
    #[inline(always)]
    pub fn p8(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P8)
    }
    #[doc = "Px = P9"]
    #[inline(always)]
    pub fn p9(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P9)
    }
    #[doc = "Px = P10"]
    #[inline(always)]
    pub fn p10(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P10)
    }
    #[doc = "Px = P11"]
    #[inline(always)]
    pub fn p11(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P11)
    }
    #[doc = "Px = P12"]
    #[inline(always)]
    pub fn p12(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P12)
    }
    #[doc = "Px = P13"]
    #[inline(always)]
    pub fn p13(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P13)
    }
    #[doc = "Px = P14"]
    #[inline(always)]
    pub fn p14(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P14)
    }
    #[doc = "Px = P15"]
    #[inline(always)]
    pub fn p15(self) -> &'a mut crate::W<REG> {
        self.variant(Captioposel0::P15)
    }
}
#[doc = "Capacitive Touch IO enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captioen {
    #[doc = "0: All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    Off = 0,
    #[doc = "1: Selected Capacitive Touch IO is enabled"]
    On = 1,
}
impl From<Captioen> for bool {
    #[inline(always)]
    fn from(variant: Captioen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTIOEN` reader - Capacitive Touch IO enable"]
pub type CaptioenR = crate::BitReader<Captioen>;
impl CaptioenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captioen {
        match self.bits {
            false => Captioen::Off,
            true => Captioen::On,
        }
    }
    #[doc = "All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Captioen::Off
    }
    #[doc = "Selected Capacitive Touch IO is enabled"]
    #[inline(always)]
    pub fn is_on(&self) -> bool {
        *self == Captioen::On
    }
}
#[doc = "Field `CAPTIOEN` writer - Capacitive Touch IO enable"]
pub type CaptioenW<'a, REG> = crate::BitWriter<'a, REG, Captioen>;
impl<'a, REG> CaptioenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "All Capacitive Touch IOs are disabled. Signal towards timers is 0."]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Captioen::Off)
    }
    #[doc = "Selected Capacitive Touch IO is enabled"]
    #[inline(always)]
    pub fn on(self) -> &'a mut crate::W<REG> {
        self.variant(Captioen::On)
    }
}
#[doc = "Capacitive Touch IO state\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Captio {
    #[doc = "0: Curent state 0 or Capacitive Touch IO is disabled"]
    Captio0 = 0,
    #[doc = "1: Current state 1"]
    Captio1 = 1,
}
impl From<Captio> for bool {
    #[inline(always)]
    fn from(variant: Captio) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAPTIO` reader - Capacitive Touch IO state"]
pub type CaptioR = crate::BitReader<Captio>;
impl CaptioR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Captio {
        match self.bits {
            false => Captio::Captio0,
            true => Captio::Captio1,
        }
    }
    #[doc = "Curent state 0 or Capacitive Touch IO is disabled"]
    #[inline(always)]
    pub fn is_captio_0(&self) -> bool {
        *self == Captio::Captio0
    }
    #[doc = "Current state 1"]
    #[inline(always)]
    pub fn is_captio_1(&self) -> bool {
        *self == Captio::Captio1
    }
}
impl R {
    #[doc = "Bits 1:3 - Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopisel0(&self) -> Captiopisel0R {
        Captiopisel0R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposel0(&self) -> Captioposel0R {
        Captioposel0R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&self) -> CaptioenR {
        CaptioenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capacitive Touch IO state"]
    #[inline(always)]
    pub fn captio(&self) -> CaptioR {
        CaptioR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 1:3 - Capacitive Touch IO pin select"]
    #[inline(always)]
    pub fn captiopisel0(&mut self) -> Captiopisel0W<Captio0ctlSpec> {
        Captiopisel0W::new(self, 1)
    }
    #[doc = "Bits 4:7 - Capacitive Touch IO port select"]
    #[inline(always)]
    pub fn captioposel0(&mut self) -> Captioposel0W<Captio0ctlSpec> {
        Captioposel0W::new(self, 4)
    }
    #[doc = "Bit 8 - Capacitive Touch IO enable"]
    #[inline(always)]
    pub fn captioen(&mut self) -> CaptioenW<Captio0ctlSpec> {
        CaptioenW::new(self, 8)
    }
}
#[doc = "Capacitive Touch IO 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`captio0ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`captio0ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Captio0ctlSpec;
impl crate::RegisterSpec for Captio0ctlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`captio0ctl::R`](R) reader structure"]
impl crate::Readable for Captio0ctlSpec {}
#[doc = "`write(|w| ..)` method takes [`captio0ctl::W`](W) writer structure"]
impl crate::Writable for Captio0ctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPTIO0CTL to value 0"]
impl crate::Resettable for Captio0ctlSpec {}
