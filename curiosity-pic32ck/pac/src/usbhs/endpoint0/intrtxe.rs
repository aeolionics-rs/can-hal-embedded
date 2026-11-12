#[doc = "Register `INTRTXE` reader"]
pub type R = crate::R<IntrtxeSpec>;
#[doc = "Register `INTRTXE` writer"]
pub type W = crate::W<IntrtxeSpec>;
#[doc = "Field `EP0TXEN` reader - Interrupt Enabled for TX EP0"]
pub type Ep0txenR = crate::BitReader;
#[doc = "Field `EP0TXEN` writer - Interrupt Enabled for TX EP0"]
pub type Ep0txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP1TXEN` reader - Interrupt Enabled for TX EP1"]
pub type Ep1txenR = crate::BitReader;
#[doc = "Field `EP1TXEN` writer - Interrupt Enabled for TX EP1"]
pub type Ep1txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2TXEN` reader - Interrupt Enabled for TX EP2"]
pub type Ep2txenR = crate::BitReader;
#[doc = "Field `EP2TXEN` writer - Interrupt Enabled for TX EP2"]
pub type Ep2txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3TXEN` reader - Interrupt Enabled for TX EP3"]
pub type Ep3txenR = crate::BitReader;
#[doc = "Field `EP3TXEN` writer - Interrupt Enabled for TX EP3"]
pub type Ep3txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4TXEN` reader - Interrupt Enabled for TX EP4"]
pub type Ep4txenR = crate::BitReader;
#[doc = "Field `EP4TXEN` writer - Interrupt Enabled for TX EP4"]
pub type Ep4txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5TXEN` reader - Interrupt Enabled for TX EP5"]
pub type Ep5txenR = crate::BitReader;
#[doc = "Field `EP5TXEN` writer - Interrupt Enabled for TX EP5"]
pub type Ep5txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6TXEN` reader - Interrupt Enabled for TX EP6"]
pub type Ep6txenR = crate::BitReader;
#[doc = "Field `EP6TXEN` writer - Interrupt Enabled for TX EP6"]
pub type Ep6txenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7TXEN` reader - Interrupt Enabled for TX EP7"]
pub type Ep7txenR = crate::BitReader;
#[doc = "Field `EP7TXEN` writer - Interrupt Enabled for TX EP7"]
pub type Ep7txenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Interrupt Enabled for TX EP0"]
    #[inline(always)]
    pub fn ep0txen(&self) -> Ep0txenR {
        Ep0txenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enabled for TX EP1"]
    #[inline(always)]
    pub fn ep1txen(&self) -> Ep1txenR {
        Ep1txenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enabled for TX EP2"]
    #[inline(always)]
    pub fn ep2txen(&self) -> Ep2txenR {
        Ep2txenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enabled for TX EP3"]
    #[inline(always)]
    pub fn ep3txen(&self) -> Ep3txenR {
        Ep3txenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enabled for TX EP4"]
    #[inline(always)]
    pub fn ep4txen(&self) -> Ep4txenR {
        Ep4txenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enabled for TX EP5"]
    #[inline(always)]
    pub fn ep5txen(&self) -> Ep5txenR {
        Ep5txenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enabled for TX EP6"]
    #[inline(always)]
    pub fn ep6txen(&self) -> Ep6txenR {
        Ep6txenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enabled for TX EP7"]
    #[inline(always)]
    pub fn ep7txen(&self) -> Ep7txenR {
        Ep7txenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt Enabled for TX EP0"]
    #[inline(always)]
    pub fn ep0txen(&mut self) -> Ep0txenW<'_, IntrtxeSpec> {
        Ep0txenW::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt Enabled for TX EP1"]
    #[inline(always)]
    pub fn ep1txen(&mut self) -> Ep1txenW<'_, IntrtxeSpec> {
        Ep1txenW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Enabled for TX EP2"]
    #[inline(always)]
    pub fn ep2txen(&mut self) -> Ep2txenW<'_, IntrtxeSpec> {
        Ep2txenW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enabled for TX EP3"]
    #[inline(always)]
    pub fn ep3txen(&mut self) -> Ep3txenW<'_, IntrtxeSpec> {
        Ep3txenW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Enabled for TX EP4"]
    #[inline(always)]
    pub fn ep4txen(&mut self) -> Ep4txenW<'_, IntrtxeSpec> {
        Ep4txenW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Enabled for TX EP5"]
    #[inline(always)]
    pub fn ep5txen(&mut self) -> Ep5txenW<'_, IntrtxeSpec> {
        Ep5txenW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Enabled for TX EP6"]
    #[inline(always)]
    pub fn ep6txen(&mut self) -> Ep6txenW<'_, IntrtxeSpec> {
        Ep6txenW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Enabled for TX EP7"]
    #[inline(always)]
    pub fn ep7txen(&mut self) -> Ep7txenW<'_, IntrtxeSpec> {
        Ep7txenW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Bits for INTRTX\n\nYou can [`read`](crate::Reg::read) this register and get [`intrtxe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrtxe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrtxeSpec;
impl crate::RegisterSpec for IntrtxeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intrtxe::R`](R) reader structure"]
impl crate::Readable for IntrtxeSpec {}
#[doc = "`write(|w| ..)` method takes [`intrtxe::W`](W) writer structure"]
impl crate::Writable for IntrtxeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRTXE to value 0xffff"]
impl crate::Resettable for IntrtxeSpec {
    const RESET_VALUE: u16 = 0xffff;
}
