#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `WAKEUP` reader - Clear USCORE Detaction Interrupt Enable"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - Clear USCORE Detaction Interrupt Enable"]
pub type WakeupW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESUME` reader - Clear Resume Interrupt Enable"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Clear Resume Interrupt Enable"]
pub type ResumeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `USB` reader - Clear USBCORE General Interrupt Enable"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - Clear USBCORE General Interrupt Enable"]
pub type UsbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA` reader - Clear DMA Interrupt Enable"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - Clear DMA Interrupt Enable"]
pub type DmaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `T1MS` reader - Clear T1MS Interrupt Enable"]
pub type T1msR = crate::BitReader;
#[doc = "Field `T1MS` writer - Clear T1MS Interrupt Enable"]
pub type T1msW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PHYRDY` reader - Clear PHYRDY Interrupt Enable"]
pub type PhyrdyR = crate::BitReader;
#[doc = "Field `PHYRDY` writer - Clear PHYRDY Interrupt Enable"]
pub type PhyrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear USCORE Detaction Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear USBCORE General Interrupt Enable"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear T1MS Interrupt Enable"]
    #[inline(always)]
    pub fn t1ms(&self) -> T1msR {
        T1msR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Clear PHYRDY Interrupt Enable"]
    #[inline(always)]
    pub fn phyrdy(&self) -> PhyrdyR {
        PhyrdyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear USCORE Detaction Interrupt Enable"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IntenclrSpec> {
        WakeupW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear Resume Interrupt Enable"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, IntenclrSpec> {
        ResumeW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear USBCORE General Interrupt Enable"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, IntenclrSpec> {
        UsbW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear DMA Interrupt Enable"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, IntenclrSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear T1MS Interrupt Enable"]
    #[inline(always)]
    pub fn t1ms(&mut self) -> T1msW<'_, IntenclrSpec> {
        T1msW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear PHYRDY Interrupt Enable"]
    #[inline(always)]
    pub fn phyrdy(&mut self) -> PhyrdyW<'_, IntenclrSpec> {
        PhyrdyW::new(self, 5)
    }
}
#[doc = "USBHS Interrupt Enable Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
