#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `WAKEUP` reader - USB Activity Detection Interrupt"]
pub type WakeupR = crate::BitReader;
#[doc = "Field `WAKEUP` writer - USB Activity Detection Interrupt"]
pub type WakeupW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESUME` reader - Resume Interrupt"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESUME` writer - Resume Interrupt"]
pub type ResumeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `USB` reader - USBCORE General Interrupt"]
pub type UsbR = crate::BitReader;
#[doc = "Field `USB` writer - USBCORE General Interrupt"]
pub type UsbW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DMA` reader - DMA Interrupt"]
pub type DmaR = crate::BitReader;
#[doc = "Field `DMA` writer - DMA Interrupt"]
pub type DmaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `T1MS` reader - Timer 1ms Tick Interrupt"]
pub type T1msR = crate::BitReader;
#[doc = "Field `T1MS` writer - Timer 1ms Tick Interrupt"]
pub type T1msW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PHYRDY` reader - PHY Ready Interrupt"]
pub type PhyrdyR = crate::BitReader;
#[doc = "Field `PHYRDY` writer - PHY Ready Interrupt"]
pub type PhyrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - USB Activity Detection Interrupt"]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Resume Interrupt"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - USBCORE General Interrupt"]
    #[inline(always)]
    pub fn usb(&self) -> UsbR {
        UsbR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dma(&self) -> DmaR {
        DmaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timer 1ms Tick Interrupt"]
    #[inline(always)]
    pub fn t1ms(&self) -> T1msR {
        T1msR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHY Ready Interrupt"]
    #[inline(always)]
    pub fn phyrdy(&self) -> PhyrdyR {
        PhyrdyR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Activity Detection Interrupt"]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, IntflagSpec> {
        WakeupW::new(self, 0)
    }
    #[doc = "Bit 1 - Resume Interrupt"]
    #[inline(always)]
    pub fn resume(&mut self) -> ResumeW<'_, IntflagSpec> {
        ResumeW::new(self, 1)
    }
    #[doc = "Bit 2 - USBCORE General Interrupt"]
    #[inline(always)]
    pub fn usb(&mut self) -> UsbW<'_, IntflagSpec> {
        UsbW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Interrupt"]
    #[inline(always)]
    pub fn dma(&mut self) -> DmaW<'_, IntflagSpec> {
        DmaW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer 1ms Tick Interrupt"]
    #[inline(always)]
    pub fn t1ms(&mut self) -> T1msW<'_, IntflagSpec> {
        T1msW::new(self, 4)
    }
    #[doc = "Bit 5 - PHY Ready Interrupt"]
    #[inline(always)]
    pub fn phyrdy(&mut self) -> PhyrdyW<'_, IntflagSpec> {
        PhyrdyW::new(self, 5)
    }
}
#[doc = "USBHS Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3f;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
