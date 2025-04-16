#[doc = "Register `LEACMCTL` reader"]
pub type R = crate::R<LeacmctlSpec>;
#[doc = "Register `LEACMCTL` writer"]
pub type W = crate::W<LeacmctlSpec>;
#[doc = "This bit controls access to LEA code memory.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Leacmae {
    #[doc = "0: Code memory access disabled. Accesses to LEA code memory are not possible. LEA does accept commands for execution. Reads to LEA code memory will return zeroes and writes are ignored."]
    Leacmae0 = 0,
    #[doc = "1: Code memory access enabled. Accesses to LEA code memory are possible. LEA does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication."]
    Leacmae1 = 1,
}
impl From<Leacmae> for bool {
    #[inline(always)]
    fn from(variant: Leacmae) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LEACMAE` reader - This bit controls access to LEA code memory."]
pub type LeacmaeR = crate::BitReader<Leacmae>;
impl LeacmaeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacmae {
        match self.bits {
            false => Leacmae::Leacmae0,
            true => Leacmae::Leacmae1,
        }
    }
    #[doc = "Code memory access disabled. Accesses to LEA code memory are not possible. LEA does accept commands for execution. Reads to LEA code memory will return zeroes and writes are ignored."]
    #[inline(always)]
    pub fn is_leacmae_0(&self) -> bool {
        *self == Leacmae::Leacmae0
    }
    #[doc = "Code memory access enabled. Accesses to LEA code memory are possible. LEA does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication."]
    #[inline(always)]
    pub fn is_leacmae_1(&self) -> bool {
        *self == Leacmae::Leacmae1
    }
}
#[doc = "Field `LEACMAE` writer - This bit controls access to LEA code memory."]
pub type LeacmaeW<'a, REG> = crate::BitWriter<'a, REG, Leacmae>;
impl<'a, REG> LeacmaeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code memory access disabled. Accesses to LEA code memory are not possible. LEA does accept commands for execution. Reads to LEA code memory will return zeroes and writes are ignored."]
    #[inline(always)]
    pub fn leacmae_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacmae::Leacmae0)
    }
    #[doc = "Code memory access enabled. Accesses to LEA code memory are possible. LEA does not accept commands during this mode (command is ignored). Coprocessor interface accesses by the CPU cause a Coprocessor not available indication."]
    #[inline(always)]
    pub fn leacmae_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacmae::Leacmae1)
    }
}
#[doc = "Field `LEAINC` reader - This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP."]
pub type LeaincR = crate::BitReader;
#[doc = "Field `LEAINC` writer - This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP."]
pub type LeaincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEADEC` reader - This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP."]
pub type LeadecR = crate::BitReader;
#[doc = "Field `LEADEC` writer - This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP."]
pub type LeadecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Leacroff {
    #[doc = "0: Contents of LEA code RAM are retained in LPM3/LPM4."]
    Leacroff0 = 0,
    #[doc = "1: Turns off the LEA code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM."]
    Leacroff1 = 1,
    #[doc = "2: Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM."]
    Leacroff2 = 2,
    #[doc = "3: Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)"]
    Leacroff3 = 3,
}
impl From<Leacroff> for u8 {
    #[inline(always)]
    fn from(variant: Leacroff) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Leacroff {
    type Ux = u8;
}
impl crate::IsEnum for Leacroff {}
#[doc = "Field `LEACROFF` reader - Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0"]
pub type LeacroffR = crate::FieldReader<Leacroff>;
impl LeacroffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Leacroff {
        match self.bits {
            0 => Leacroff::Leacroff0,
            1 => Leacroff::Leacroff1,
            2 => Leacroff::Leacroff2,
            3 => Leacroff::Leacroff3,
            _ => unreachable!(),
        }
    }
    #[doc = "Contents of LEA code RAM are retained in LPM3/LPM4."]
    #[inline(always)]
    pub fn is_leacroff_0(&self) -> bool {
        *self == Leacroff::Leacroff0
    }
    #[doc = "Turns off the LEA code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM."]
    #[inline(always)]
    pub fn is_leacroff_1(&self) -> bool {
        *self == Leacroff::Leacroff1
    }
    #[doc = "Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM."]
    #[inline(always)]
    pub fn is_leacroff_2(&self) -> bool {
        *self == Leacroff::Leacroff2
    }
    #[doc = "Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)"]
    #[inline(always)]
    pub fn is_leacroff_3(&self) -> bool {
        *self == Leacroff::Leacroff3
    }
}
#[doc = "Field `LEACROFF` writer - Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0"]
pub type LeacroffW<'a, REG> = crate::FieldWriter<'a, REG, 2, Leacroff, crate::Safe>;
impl<'a, REG> LeacroffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Contents of LEA code RAM are retained in LPM3/LPM4."]
    #[inline(always)]
    pub fn leacroff_0(self) -> &'a mut crate::W<REG> {
        self.variant(Leacroff::Leacroff0)
    }
    #[doc = "Turns off the LEA code RAM in LPM3/LPM4, re-activates it on wake-up. All data of the code RAM is lost after wakeup from LPM3/LPM4. See the device specific data sheet for presence and size of Code RAM."]
    #[inline(always)]
    pub fn leacroff_1(self) -> &'a mut crate::W<REG> {
        self.variant(Leacroff::Leacroff1)
    }
    #[doc = "Turns off the code RAM entering LPM3/LPM4, the code RAM sector remains off after wake-up. All data of the code RAM is lost. See the device-specific data sheet for presence and size of Code RAM."]
    #[inline(always)]
    pub fn leacroff_2(self) -> &'a mut crate::W<REG> {
        self.variant(Leacroff::Leacroff2)
    }
    #[doc = "Reserved (Future: Turns off the code RAM immediately. All data of the Code RAM is lost. See the device-specific data sheet for presence and size of Code RAM.)"]
    #[inline(always)]
    pub fn leacroff_3(self) -> &'a mut crate::W<REG> {
        self.variant(Leacroff::Leacroff3)
    }
}
#[doc = "Field `LEACRACTION` reader - Code RAM action"]
pub type LeacractionR = crate::BitReader;
#[doc = "Field `LEACRACTION` writer - Code RAM action"]
pub type LeacractionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LEACMAP` reader - LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP"]
pub type LeacmapR = crate::FieldReader<u16>;
#[doc = "Field `LEACMAP` writer - LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP"]
pub type LeacmapW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - This bit controls access to LEA code memory."]
    #[inline(always)]
    pub fn leacmae(&self) -> LeacmaeR {
        LeacmaeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leainc(&self) -> LeaincR {
        LeaincR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leadec(&self) -> LeadecR {
        LeadecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0"]
    #[inline(always)]
    pub fn leacroff(&self) -> LeacroffR {
        LeacroffR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Code RAM action"]
    #[inline(always)]
    pub fn leacraction(&self) -> LeacractionR {
        LeacractionR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 16:31 - LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP"]
    #[inline(always)]
    pub fn leacmap(&self) -> LeacmapR {
        LeacmapR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This bit controls access to LEA code memory."]
    #[inline(always)]
    pub fn leacmae(&mut self) -> LeacmaeW<LeacmctlSpec> {
        LeacmaeW::new(self, 0)
    }
    #[doc = "Bit 2 - This bit when set causes the code memory address port field LEACMAP to increment each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double increment is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leainc(&mut self) -> LeaincW<LeacmctlSpec> {
        LeaincW::new(self, 2)
    }
    #[doc = "Bit 3 - This bit when set causes the code memory address port field LEACMAP to decrement each time after an access to LEACMDP is performed.The decrement is by value two on 16B accesses on lower word of LEACMA.The decrement is by value two on 32B accesses on LEACMA.#b#Note:#/b# A double decrement is performed when read modify write accesses are done on LEACMDP."]
    #[inline(always)]
    pub fn leadec(&mut self) -> LeadecW<LeacmctlSpec> {
        LeadecW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Control bits only available if LEA code RAM is available. Otherwise reserved. Reserved. Read back as 0"]
    #[inline(always)]
    pub fn leacroff(&mut self) -> LeacroffW<LeacmctlSpec> {
        LeacroffW::new(self, 4)
    }
    #[doc = "Bit 6 - Code RAM action"]
    #[inline(always)]
    pub fn leacraction(&mut self) -> LeacractionW<LeacmctlSpec> {
        LeacractionW::new(self, 6)
    }
    #[doc = "Bits 16:31 - LEA code memory address port. This bit field points to the memory location that is accessible via LEACMDP"]
    #[inline(always)]
    pub fn leacmap(&mut self) -> LeacmapW<LeacmctlSpec> {
        LeacmapW::new(self, 16)
    }
}
#[doc = "Code Memory Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`leacmctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacmctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeacmctlSpec;
impl crate::RegisterSpec for LeacmctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacmctl::R`](R) reader structure"]
impl crate::Readable for LeacmctlSpec {}
#[doc = "`write(|w| ..)` method takes [`leacmctl::W`](W) writer structure"]
impl crate::Writable for LeacmctlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACMCTL to value 0"]
impl crate::Resettable for LeacmctlSpec {}
