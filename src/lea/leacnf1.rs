#[doc = "Register `LEACNF1` reader"]
pub type R = crate::R<Leacnf1Spec>;
#[doc = "Register `LEACNF1` writer"]
pub type W = crate::W<Leacnf1Spec>;
#[doc = "This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leabusy {
    #[doc = "0: LEA is in Ready can accept new commands"]
    Ready = 0,
    #[doc = "1: LEA is busy right now and cannot accept any commands"]
    Busy = 1,
}
impl From<Leabusy> for bool {
    #[inline(always)]
    fn from(variant: Leabusy) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEABUSY` reader - This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)"]
pub type LeabusyR = crate::BitReader<Leabusy>;
impl LeabusyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leabusy {
        match self.bits {
            false => Leabusy::Ready,
            true => Leabusy::Busy,
        }
    }
    #[doc = "LEA is in Ready can accept new commands"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Leabusy::Ready
    }
    #[doc = "LEA is busy right now and cannot accept any commands"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Leabusy::Busy
    }
}
#[doc = "These Bits indicate the actual operation mode LEA is in. Other = Reserved\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leamode {
    #[doc = "0: Off (implicit)"]
    Off = 0,
    #[doc = "1: Ready"]
    Ready = 1,
    #[doc = "2: RunS (SUSPEND)"]
    Runs = 2,
    #[doc = "3: RunR (RESUME)"]
    Runr = 3,
    #[doc = "4: RunA (regular command operation )"]
    Runa = 4,
    #[doc = "5: Notify"]
    Notify = 5,
    #[doc = "6: Sleep"]
    Sleep = 6,
    #[doc = "7: RunL"]
    Runl = 7,
}
impl From<Leamode> for u8 {
    #[inline(always)]
    fn from(variant: Leamode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leamode {
    type Ux = u8;
}
impl crate::IsEnum for Leamode {}
#[doc = "Field `LEAMODE` reader - These Bits indicate the actual operation mode LEA is in. Other = Reserved"]
pub type LeamodeR = crate::FieldReader<Leamode>;
impl LeamodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Leamode> {
        match self.bits {
            0 => Some(Leamode::Off),
            1 => Some(Leamode::Ready),
            2 => Some(Leamode::Runs),
            3 => Some(Leamode::Runr),
            4 => Some(Leamode::Runa),
            5 => Some(Leamode::Notify),
            6 => Some(Leamode::Sleep),
            7 => Some(Leamode::Runl),
            _ => None,
        }
    }
    #[doc = "Off (implicit)"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Leamode::Off
    }
    #[doc = "Ready"]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Leamode::Ready
    }
    #[doc = "RunS (SUSPEND)"]
    #[inline(always)]
    pub fn is_runs(&self) -> bool {
        *self == Leamode::Runs
    }
    #[doc = "RunR (RESUME)"]
    #[inline(always)]
    pub fn is_runr(&self) -> bool {
        *self == Leamode::Runr
    }
    #[doc = "RunA (regular command operation )"]
    #[inline(always)]
    pub fn is_runa(&self) -> bool {
        *self == Leamode::Runa
    }
    #[doc = "Notify"]
    #[inline(always)]
    pub fn is_notify(&self) -> bool {
        *self == Leamode::Notify
    }
    #[doc = "Sleep"]
    #[inline(always)]
    pub fn is_sleep(&self) -> bool {
        *self == Leamode::Sleep
    }
    #[doc = "RunL"]
    #[inline(always)]
    pub fn is_runl(&self) -> bool {
        *self == Leamode::Runl
    }
}
#[doc = "Field `LEAPWST` reader - These bits indicate the current power consumption as a relative value. The value zero indicated only static operation (usually clock less). This register might be read out for statistical power estimation of an application. These bits are also reflected in J-STATE when debugging"]
pub type LeapwstR = crate::FieldReader;
#[doc = "Field `LEAASST` reader - These bits are used to store the internal state of the application specific processor (ASIP) inside the accelerator core. The specific meaning of those bit patterns is not shown in this document."]
pub type LeaasstR = crate::FieldReader;
#[doc = "Field `LEADONEC` reader - LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
pub type LeadonecR = crate::BitReader;
#[doc = "Field `LEADONEC` writer - LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
pub type LeadonecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEAFREEC` reader - LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
pub type LeafreecR = crate::BitReader;
#[doc = "Field `LEAFREEC` writer - LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
pub type LeafreecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEATIMFLTC` reader - LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect.."]
pub type LeatimfltcR = crate::BitReader;
#[doc = "Field `LEATIMFLTC` writer - LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect.."]
pub type LeatimfltcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "LEA command fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacfltc {
    #[doc = "0: No command fault occurred since this bit was cleared"]
    Leacfltc0 = 0,
    #[doc = "1: At least one command fault occurred since this bit was cleared"]
    Leacfltc1 = 1,
}
impl From<Leacfltc> for bool {
    #[inline(always)]
    fn from(variant: Leacfltc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACFLTC` reader - LEA command fault"]
pub type LeacfltcR = crate::BitReader<Leacfltc>;
impl LeacfltcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacfltc {
        match self.bits {
            false => Leacfltc::Leacfltc0,
            true => Leacfltc::Leacfltc1,
        }
    }
    #[doc = "No command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn is_leacfltc_0(&self) -> bool {
        *self == Leacfltc::Leacfltc0
    }
    #[doc = "At least one command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn is_leacfltc_1(&self) -> bool {
        *self == Leacfltc::Leacfltc1
    }
}
#[doc = "Field `LEACFLTC` writer - LEA command fault"]
pub type LeacfltcW<'a, REG> = crate::BitWriter<'a, REG, Leacfltc>;
impl<'a, REG> LeacfltcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacfltc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacfltc::Leacfltc0)
    }
    #[doc = "At least one command fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leacfltc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacfltc::Leacfltc1)
    }
}
#[doc = "LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leamemfltc {
    #[doc = "0: No memory fault occurred since this bit was cleared"]
    Leamemfltc0 = 0,
    #[doc = "1: At least one memory fault since this bit was cleared"]
    Leamemfltc1 = 1,
}
impl From<Leamemfltc> for bool {
    #[inline(always)]
    fn from(variant: Leamemfltc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEAMEMFLTC` reader - LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect."]
pub type LeamemfltcR = crate::BitReader<Leamemfltc>;
impl LeamemfltcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leamemfltc {
        match self.bits {
            false => Leamemfltc::Leamemfltc0,
            true => Leamemfltc::Leamemfltc1,
        }
    }
    #[doc = "No memory fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn is_leamemfltc_0(&self) -> bool {
        *self == Leamemfltc::Leamemfltc0
    }
    #[doc = "At least one memory fault since this bit was cleared"]
    #[inline(always)]
    pub fn is_leamemfltc_1(&self) -> bool {
        *self == Leamemfltc::Leamemfltc1
    }
}
#[doc = "Field `LEAMEMFLTC` writer - LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect."]
pub type LeamemfltcW<'a, REG> = crate::BitWriter<'a, REG, Leamemfltc>;
impl<'a, REG> LeamemfltcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No memory fault occurred since this bit was cleared"]
    #[inline(always)]
    pub fn leamemfltc_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leamemfltc::Leamemfltc0)
    }
    #[doc = "At least one memory fault since this bit was cleared"]
    #[inline(always)]
    pub fn leamemfltc_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leamemfltc::Leamemfltc1)
    }
}
#[doc = "Field `LEARDSTAT` reader - Read Status. This bit field keeps the VBUS read status lines from the last bus error condition."]
pub type LeardstatR = crate::FieldReader;
#[doc = "Field `LEAWRSTAT` reader - Write Status. This bit field keeps the VBUS write status lines from the last bus error condition."]
pub type LeawrstatR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit indicate if LEA is able to accept new Commands (SUSPEND is always accepted)"]
    #[inline(always)]
    pub fn leabusy(&self) -> LeabusyR {
        LeabusyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:7 - These Bits indicate the actual operation mode LEA is in. Other = Reserved"]
    #[inline(always)]
    pub fn leamode(&self) -> LeamodeR {
        LeamodeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - These bits indicate the current power consumption as a relative value. The value zero indicated only static operation (usually clock less). This register might be read out for statistical power estimation of an application. These bits are also reflected in J-STATE when debugging"]
    #[inline(always)]
    pub fn leapwst(&self) -> LeapwstR {
        LeapwstR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - These bits are used to store the internal state of the application specific processor (ASIP) inside the accelerator core. The specific meaning of those bit patterns is not shown in this document."]
    #[inline(always)]
    pub fn leaasst(&self) -> LeaasstR {
        LeaasstR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadonec(&self) -> LeadonecR {
        LeadonecR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafreec(&self) -> LeafreecR {
        LeafreecR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 21 - LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect.."]
    #[inline(always)]
    pub fn leatimfltc(&self) -> LeatimfltcR {
        LeatimfltcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - LEA command fault"]
    #[inline(always)]
    pub fn leacfltc(&self) -> LeacfltcR {
        LeacfltcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leamemfltc(&self) -> LeamemfltcR {
        LeamemfltcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Read Status. This bit field keeps the VBUS read status lines from the last bus error condition."]
    #[inline(always)]
    pub fn leardstat(&self) -> LeardstatR {
        LeardstatR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Write Status. This bit field keeps the VBUS write status lines from the last bus error condition."]
    #[inline(always)]
    pub fn leawrstat(&self) -> LeawrstatR {
        LeawrstatR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - LEA done event indication and clear flag. This bit indicated the done event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leadonec(&mut self) -> LeadonecW<Leacnf1Spec> {
        LeadonecW::new(self, 16)
    }
    #[doc = "Bit 17 - LEA free event indication and clear flag. This bit indicated the free event for LEA. This bit is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leafreec(&mut self) -> LeafreecW<Leacnf1Spec> {
        LeafreecW::new(self, 17)
    }
    #[doc = "Bit 21 - LEA timeout fault indication and clear flag; This bits indicates that a timer timeout occurred. This fault is cleared by writing a one to it. Writing a zero has no effect.."]
    #[inline(always)]
    pub fn leatimfltc(&mut self) -> LeatimfltcW<Leacnf1Spec> {
        LeatimfltcW::new(self, 21)
    }
    #[doc = "Bit 22 - LEA command fault"]
    #[inline(always)]
    pub fn leacfltc(&mut self) -> LeacfltcW<Leacnf1Spec> {
        LeacfltcW::new(self, 22)
    }
    #[doc = "Bit 23 - LEA memory fault indication and clear flag. This bit indicates that a fault in the memory VBUS interface occurred. The exact fault reason may be identified by checking LEAWRSTAT and LEARDSTAT. This fault is also signaled to the SYS-module as bus error when enabled (LEACNF0.LEAMEMFLTE=1). Only one fault condition is signaled until this bit is cleared. Leaving this bit set will not cause any further faults. This fault is cleared by writing a one to it. Writing a zero has no effect."]
    #[inline(always)]
    pub fn leamemfltc(&mut self) -> LeamemfltcW<Leacnf1Spec> {
        LeamemfltcW::new(self, 23)
    }
}
#[doc = "Configuration Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`leacnf1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacnf1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leacnf1Spec;
impl crate::RegisterSpec for Leacnf1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacnf1::R`](R) reader structure"]
impl crate::Readable for Leacnf1Spec {}
#[doc = "`write(|w| ..)` method takes [`leacnf1::W`](W) writer structure"]
impl crate::Writable for Leacnf1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACNF1 to value 0"]
impl crate::Resettable for Leacnf1Spec {}
