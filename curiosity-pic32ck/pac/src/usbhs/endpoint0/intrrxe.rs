#[doc = "Register `INTRRXE` reader"]
pub type R = crate::R<IntrrxeSpec>;
#[doc = "Register `INTRRXE` writer"]
pub type W = crate::W<IntrrxeSpec>;
#[doc = "Field `EP1RXE` reader - Interrupt Enabled for RX EP1"]
pub type Ep1rxeR = crate::BitReader;
#[doc = "Field `EP1RXE` writer - Interrupt Enabled for RX EP1"]
pub type Ep1rxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP2RXE` reader - Interrupt Enabled for RX EP2"]
pub type Ep2rxeR = crate::BitReader;
#[doc = "Field `EP2RXE` writer - Interrupt Enabled for RX EP2"]
pub type Ep2rxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP3RXE` reader - Interrupt Enabled for RX EP3"]
pub type Ep3rxeR = crate::BitReader;
#[doc = "Field `EP3RXE` writer - Interrupt Enabled for RX EP3"]
pub type Ep3rxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP4RXE` reader - Interrupt Enabled for RX EP4"]
pub type Ep4rxeR = crate::BitReader;
#[doc = "Field `EP4RXE` writer - Interrupt Enabled for RX EP4"]
pub type Ep4rxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP5RXE` reader - Interrupt Enabled for RX EP5"]
pub type Ep5rxeR = crate::BitReader;
#[doc = "Field `EP5RXE` writer - Interrupt Enabled for RX EP5"]
pub type Ep5rxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP6RXE` reader - Interrupt Enabled for RX EP6"]
pub type Ep6rxeR = crate::BitReader;
#[doc = "Field `EP6RXE` writer - Interrupt Enabled for RX EP6"]
pub type Ep6rxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP7RXE` reader - Interrupt Enabled for RX EP7"]
pub type Ep7rxeR = crate::BitReader;
#[doc = "Field `EP7RXE` writer - Interrupt Enabled for RX EP7"]
pub type Ep7rxeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Interrupt Enabled for RX EP1"]
    #[inline(always)]
    pub fn ep1rxe(&self) -> Ep1rxeR {
        Ep1rxeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt Enabled for RX EP2"]
    #[inline(always)]
    pub fn ep2rxe(&self) -> Ep2rxeR {
        Ep2rxeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt Enabled for RX EP3"]
    #[inline(always)]
    pub fn ep3rxe(&self) -> Ep3rxeR {
        Ep3rxeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt Enabled for RX EP4"]
    #[inline(always)]
    pub fn ep4rxe(&self) -> Ep4rxeR {
        Ep4rxeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt Enabled for RX EP5"]
    #[inline(always)]
    pub fn ep5rxe(&self) -> Ep5rxeR {
        Ep5rxeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt Enabled for RX EP6"]
    #[inline(always)]
    pub fn ep6rxe(&self) -> Ep6rxeR {
        Ep6rxeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt Enabled for RX EP7"]
    #[inline(always)]
    pub fn ep7rxe(&self) -> Ep7rxeR {
        Ep7rxeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Interrupt Enabled for RX EP1"]
    #[inline(always)]
    pub fn ep1rxe(&mut self) -> Ep1rxeW<'_, IntrrxeSpec> {
        Ep1rxeW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt Enabled for RX EP2"]
    #[inline(always)]
    pub fn ep2rxe(&mut self) -> Ep2rxeW<'_, IntrrxeSpec> {
        Ep2rxeW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt Enabled for RX EP3"]
    #[inline(always)]
    pub fn ep3rxe(&mut self) -> Ep3rxeW<'_, IntrrxeSpec> {
        Ep3rxeW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt Enabled for RX EP4"]
    #[inline(always)]
    pub fn ep4rxe(&mut self) -> Ep4rxeW<'_, IntrrxeSpec> {
        Ep4rxeW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt Enabled for RX EP5"]
    #[inline(always)]
    pub fn ep5rxe(&mut self) -> Ep5rxeW<'_, IntrrxeSpec> {
        Ep5rxeW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt Enabled for RX EP6"]
    #[inline(always)]
    pub fn ep6rxe(&mut self) -> Ep6rxeW<'_, IntrrxeSpec> {
        Ep6rxeW::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt Enabled for RX EP7"]
    #[inline(always)]
    pub fn ep7rxe(&mut self) -> Ep7rxeW<'_, IntrrxeSpec> {
        Ep7rxeW::new(self, 7)
    }
}
#[doc = "Interrupt Enable Bits for INTRRX\n\nYou can [`read`](crate::Reg::read) this register and get [`intrrxe::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intrrxe::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrrxeSpec;
impl crate::RegisterSpec for IntrrxeSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`intrrxe::R`](R) reader structure"]
impl crate::Readable for IntrrxeSpec {}
#[doc = "`write(|w| ..)` method takes [`intrrxe::W`](W) writer structure"]
impl crate::Writable for IntrrxeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTRRXE to value 0xfffe"]
impl crate::Resettable for IntrrxeSpec {
    const RESET_VALUE: u16 = 0xfffe;
}
