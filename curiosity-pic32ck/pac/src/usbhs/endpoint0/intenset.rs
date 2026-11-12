#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Field `WAKEUP` reader - USCORE Detaction Interrupt Enable"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - USCORE Detaction Interrupt Enable"]
pub type WakeupW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RESUME` reader - Resume Interrupt Enable"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Resume Interrupt Enable"]
pub type ResumeW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `USB` reader - USBCORE General Interrupt Enable"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - USBCORE General Interrupt Enable"]
pub type UsbW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `DMA` reader - DMA Interrupt Enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Interrupt Enable"]
pub type DmaW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `T1MS` reader - T1MS Interrupt Enable"]
pub type T1msR = crate::BitReader;
#[doc = "Field `T1MS` writer - T1MS Interrupt Enable"]
pub type T1msW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `PHYRDY` reader - PHYRDY Interrupt Enable"]
pub type PhyrdyR = crate::BitReader;
#[doc = "Field `PHYRDY` writer - PHYRDY Interrupt Enable"]
pub type PhyrdyW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - USCORE Detaction Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USBCORE General Interrupt Enable"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - T1MS Interrupt Enable"]
    #[inline(always)]
    pub fn t1ms(&self) -> T1msR {
        T1msR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHYRDY Interrupt Enable"]
    #[inline(always)]
    pub fn phyrdy(&self) -> PhyrdyR {
        PhyrdyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USCORE Detaction Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IntensetSpec> {
        WakeupW::new(self, 0)
    }
    #[doc = "Bit 1 - Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, IntensetSpec> {
        ResumeW::new(self, 1)
    }
    #[doc = "Bit 2 - USBCORE General Interrupt Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, IntensetSpec> {
        UsbW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, IntensetSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bit 4 - T1MS Interrupt Enable"]
    #[inline(always)]
    pub fn t1ms(&mut self) -> T1msW<'_, IntensetSpec> {
        T1msW::new(self, 4)
    }
    #[doc = "Bit 5 - PHYRDY Interrupt Enable"]
    #[inline(always)]
    pub fn phyrdy(&mut self) -> PhyrdyW<'_, IntensetSpec> {
        PhyrdyW::new(self, 5)
    }
}
#[doc = "USBHS Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
