#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `ERR` reader - TrustRAM Readout Error Interrupt Enable"]
pub type ErrR = crate::BitReader;
#[doc = "Field `ERR` writer - TrustRAM Readout Error Interrupt Enable"]
pub type ErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DRP` reader - Data Remanence Prevention Ended Interrupt Enable"]
pub type DrpR = crate::BitReader;
#[doc = "Field `DRP` writer - Data Remanence Prevention Ended Interrupt Enable"]
pub type DrpW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - TrustRAM Readout Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ended Interrupt Enable"]
    #[inline(always)]
    pub fn drp(&self) -> DrpR {
        DrpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TrustRAM Readout Error Interrupt Enable"]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, IntenclrSpec> {
        ErrW::new(self, 0)
    }
    #[doc = "Bit 1 - Data Remanence Prevention Ended Interrupt Enable"]
    #[inline(always)]
    pub fn drp(&mut self) -> DrpW<'_, IntenclrSpec> {
        DrpW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x03;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
