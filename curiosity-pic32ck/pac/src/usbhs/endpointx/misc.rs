#[doc = "Register `MISC` reader"]
pub type R = crate::R<MiscSpec>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MiscSpec>;
#[doc = "DMA_REQ signal for all OUT endpoints will be deasserted based on this bit selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxedmaselect {
    #[doc = "0: Dma_req signal for all out endpoints will be de-asserted when maxp bytes have been written to an endpoint. this is late mode."]
    Maxp = 0,
    #[doc = "1: Dma_req signal for all out endpoints will be de-asserted when maxp-8 bytes have been written to an endpoint. this is early mode."]
    Maxp8 = 1,
}
impl From<Rxedmaselect> for bool {
    #[inline(always)]
    fn from(variant: Rxedmaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXEDMA` reader - DMA_REQ signal for all OUT endpoints will be deasserted based on this bit selection"]
pub type RxedmaR = crate::BitReader<Rxedmaselect>;
impl RxedmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxedmaselect {
        match self.bits {
            false => Rxedmaselect::Maxp,
            true => Rxedmaselect::Maxp8,
        }
    }
    #[doc = "Dma_req signal for all out endpoints will be de-asserted when maxp bytes have been written to an endpoint. this is late mode."]
    #[inline(always)]
    pub fn is_maxp(&self) -> bool {
        *self == Rxedmaselect::Maxp
    }
    #[doc = "Dma_req signal for all out endpoints will be de-asserted when maxp-8 bytes have been written to an endpoint. this is early mode."]
    #[inline(always)]
    pub fn is_maxp_8(&self) -> bool {
        *self == Rxedmaselect::Maxp8
    }
}
#[doc = "Field `RXEDMA` writer - DMA_REQ signal for all OUT endpoints will be deasserted based on this bit selection"]
pub type RxedmaW<'a, REG> = crate::BitWriter<'a, REG, Rxedmaselect>;
impl<'a, REG> RxedmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dma_req signal for all out endpoints will be de-asserted when maxp bytes have been written to an endpoint. this is late mode."]
    #[inline(always)]
    pub fn maxp(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedmaselect::Maxp)
    }
    #[doc = "Dma_req signal for all out endpoints will be de-asserted when maxp-8 bytes have been written to an endpoint. this is early mode."]
    #[inline(always)]
    pub fn maxp_8(self) -> &'a mut crate::W<REG> {
        self.variant(Rxedmaselect::Maxp8)
    }
}
#[doc = "DMA_REQ signal for all IN endpoints will be deasserted based on this bit selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txedmaselect {
    #[doc = "0: Dma_req signal for all in endpoints will be de-asserted when maxp bytes have been written to an endpoint. this is late mode."]
    Maxp = 0,
    #[doc = "1: Dma_req signal for all in endpoints will be de-asserted when maxp-8 bytes have been written to an endpoint. this is early mode."]
    Maxp8 = 1,
}
impl From<Txedmaselect> for bool {
    #[inline(always)]
    fn from(variant: Txedmaselect) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXEDMA` reader - DMA_REQ signal for all IN endpoints will be deasserted based on this bit selection"]
pub type TxedmaR = crate::BitReader<Txedmaselect>;
impl TxedmaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txedmaselect {
        match self.bits {
            false => Txedmaselect::Maxp,
            true => Txedmaselect::Maxp8,
        }
    }
    #[doc = "Dma_req signal for all in endpoints will be de-asserted when maxp bytes have been written to an endpoint. this is late mode."]
    #[inline(always)]
    pub fn is_maxp(&self) -> bool {
        *self == Txedmaselect::Maxp
    }
    #[doc = "Dma_req signal for all in endpoints will be de-asserted when maxp-8 bytes have been written to an endpoint. this is early mode."]
    #[inline(always)]
    pub fn is_maxp_8(&self) -> bool {
        *self == Txedmaselect::Maxp8
    }
}
#[doc = "Field `TXEDMA` writer - DMA_REQ signal for all IN endpoints will be deasserted based on this bit selection"]
pub type TxedmaW<'a, REG> = crate::BitWriter<'a, REG, Txedmaselect>;
impl<'a, REG> TxedmaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Dma_req signal for all in endpoints will be de-asserted when maxp bytes have been written to an endpoint. this is late mode."]
    #[inline(always)]
    pub fn maxp(self) -> &'a mut crate::W<REG> {
        self.variant(Txedmaselect::Maxp)
    }
    #[doc = "Dma_req signal for all in endpoints will be de-asserted when maxp-8 bytes have been written to an endpoint. this is early mode."]
    #[inline(always)]
    pub fn maxp_8(self) -> &'a mut crate::W<REG> {
        self.variant(Txedmaselect::Maxp8)
    }
}
impl R {
    #[doc = "Bit 0 - DMA_REQ signal for all OUT endpoints will be deasserted based on this bit selection"]
    #[inline(always)]
    pub fn rxedma(&self) -> RxedmaR {
        RxedmaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA_REQ signal for all IN endpoints will be deasserted based on this bit selection"]
    #[inline(always)]
    pub fn txedma(&self) -> TxedmaR {
        TxedmaR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA_REQ signal for all OUT endpoints will be deasserted based on this bit selection"]
    #[inline(always)]
    pub fn rxedma(&mut self) -> RxedmaW<'_, MiscSpec> {
        RxedmaW::new(self, 0)
    }
    #[doc = "Bit 1 - DMA_REQ signal for all IN endpoints will be deasserted based on this bit selection"]
    #[inline(always)]
    pub fn txedma(&mut self) -> TxedmaW<'_, MiscSpec> {
        TxedmaW::new(self, 1)
    }
}
#[doc = "RX/TX Early DMA Enable Bits\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiscSpec;
impl crate::RegisterSpec for MiscSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MiscSpec {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MiscSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MiscSpec {}
