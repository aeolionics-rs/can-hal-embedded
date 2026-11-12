#[doc = "Register `INTRUSBE` reader"]
pub type R = crate::R<IntrusbeSpec>;
#[doc = "Register `INTRUSBE` writer"]
pub type W = crate::W<IntrusbeSpec>;
#[doc = "Field `SUSPENDEN` reader - SUSEPND Interrupt Enable"]
pub type SuspendenR = crate::BitReader;
#[doc = "Field `SUSPENDEN` writer - SUSEPND Interrupt Enable"]
pub type SuspendenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESUMEEN` reader - RESUME Interrupt Enable"]
pub type ResumeenR = crate::BitReader;
#[doc = "Field `RESUMEEN` writer - RESUME Interrupt Enable"]
pub type ResumeenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETEN` reader - RESET/BABBLE Interrupt Enable"]
pub type ResetenR = crate::BitReader;
#[doc = "Field `RESETEN` writer - RESET/BABBLE Interrupt Enable"]
pub type ResetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFEN` reader - SOF Interrupt Enable"]
pub type SofenR = crate::BitReader;
#[doc = "Field `SOFEN` writer - SOF Interrupt Enable"]
pub type SofenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONNEN` reader - CONN Interrupt Enable"]
pub type ConnenR = crate::BitReader;
#[doc = "Field `CONNEN` writer - CONN Interrupt Enable"]
pub type ConnenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCONEN` reader - DISCONN Interrupt Enable"]
pub type DisconenR = crate::BitReader;
#[doc = "Field `DISCONEN` writer - DISCONN Interrupt Enable"]
pub type DisconenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SESSREQEN` reader - SESSREQ Interrupt Enable"]
pub type SessreqenR = crate::BitReader;
#[doc = "Field `SESSREQEN` writer - SESSREQ Interrupt Enable"]
pub type SessreqenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VBUSERREN` reader - VBus Error Interrupt Enable"]
pub type VbuserrenR = crate::BitReader;
#[doc = "Field `VBUSERREN` writer - VBus Error Interrupt Enable"]
pub type VbuserrenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - SUSEPND Interrupt Enable"]
    #[inline(always)]
    pub fn suspenden(&self) -> SuspendenR {
        SuspendenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&self) -> ResumeenR {
        ResumeenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RESET/BABBLE Interrupt Enable"]
    #[inline(always)]
    pub fn reseten(&self) -> ResetenR {
        ResetenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn sofen(&self) -> SofenR {
        SofenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CONN Interrupt Enable"]
    #[inline(always)]
    pub fn connen(&self) -> ConnenR {
        ConnenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DISCONN Interrupt Enable"]
    #[inline(always)]
    pub fn disconen(&self) -> DisconenR {
        DisconenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SESSREQ Interrupt Enable"]
    #[inline(always)]
    pub fn sessreqen(&self) -> SessreqenR {
        SessreqenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vbuserren(&self) -> VbuserrenR {
        VbuserrenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - SUSEPND Interrupt Enable"]
    #[inline(always)]
    pub fn suspenden(&mut self) -> SuspendenW<'_, IntrusbeSpec> {
        SuspendenW::new(self, 0)
    }
    #[doc = "Bit 1 - RESUME Interrupt Enable"]
    #[inline(always)]
    pub fn resumeen(&mut self) -> ResumeenW<'_, IntrusbeSpec> {
        ResumeenW::new(self, 1)
    }
    #[doc = "Bit 2 - RESET/BABBLE Interrupt Enable"]
    #[inline(always)]
    pub fn reseten(&mut self) -> ResetenW<'_, IntrusbeSpec> {
        ResetenW::new(self, 2)
    }
    #[doc = "Bit 3 - SOF Interrupt Enable"]
    #[inline(always)]
    pub fn sofen(&mut self) -> SofenW<'_, IntrusbeSpec> {
        SofenW::new(self, 3)
    }
    #[doc = "Bit 4 - CONN Interrupt Enable"]
    #[inline(always)]
    pub fn connen(&mut self) -> ConnenW<'_, IntrusbeSpec> {
        ConnenW::new(self, 4)
    }
    #[doc = "Bit 5 - DISCONN Interrupt Enable"]
    #[inline(always)]
    pub fn disconen(&mut self) -> DisconenW<'_, IntrusbeSpec> {
        DisconenW::new(self, 5)
    }
    #[doc = "Bit 6 - SESSREQ Interrupt Enable"]
    #[inline(always)]
    pub fn sessreqen(&mut self) -> SessreqenW<'_, IntrusbeSpec> {
        SessreqenW::new(self, 6)
    }
    #[doc = "Bit 7 - VBus Error Interrupt Enable"]
    #[inline(always)]
    pub fn vbuserren(&mut self) -> VbuserrenW<'_, IntrusbeSpec> {
        VbuserrenW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Bits for USB Interrupts\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusbe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrusbe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrusbeSpec;
impl crate::RegisterSpec for IntrusbeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intrusbe::R`](R) reader structure"]
impl crate::Readable for IntrusbeSpec {}
#[doc = "`write(|w| ..)` method takes [`intrusbe::W`](W) writer structure"]
impl crate::Writable for IntrusbeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRUSBE to value 0x06"]
impl crate::Resettable for IntrusbeSpec {
    const RESET_VALUE: u8 = 0x06;
}
