#[doc = "Register `LEACNF2` reader"]
pub type R = crate::R<Leacnf2Spec>;
#[doc = "Register `LEACNF2` writer"]
pub type W = crate::W<Leacnf2Spec>;
#[doc = "Field `LEASPTR` reader - LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity"]
pub type LeasptrR = crate::FieldReader<u16>;
#[doc = "Field `LEASPTR` writer - LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity"]
pub type LeasptrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity"]
    #[inline(always)]
    pub fn leasptr(&self) -> LeasptrR {
        LeasptrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LEA stack pointer value (byte units). 64 lower kB is physical limit for LEA application complexity"]
    #[inline(always)]
    pub fn leasptr(&mut self) -> LeasptrW<Leacnf2Spec> {
        LeasptrW::new(self, 0)
    }
}
#[doc = "Configuration Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`leacnf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`leacnf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Leacnf2Spec;
impl crate::RegisterSpec for Leacnf2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`leacnf2::R`](R) reader structure"]
impl crate::Readable for Leacnf2Spec {}
#[doc = "`write(|w| ..)` method takes [`leacnf2::W`](W) writer structure"]
impl crate::Writable for Leacnf2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LEACNF2 to value 0"]
impl crate::Resettable for Leacnf2Spec {}
