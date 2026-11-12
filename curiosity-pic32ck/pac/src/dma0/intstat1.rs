#[doc = "Register `INTSTAT1` reader"]
pub type R = crate::R<Intstat1Spec>;
#[doc = "Field `CH0` reader - DMA Channel active interrupt at priority 1"]
pub type Ch0R = crate::BitReader;
#[doc = "Field `CH1` reader - DMA Channel active interrupt at priority 1"]
pub type Ch1R = crate::BitReader;
#[doc = "Field `CH2` reader - DMA Channel active interrupt at priority 1"]
pub type Ch2R = crate::BitReader;
#[doc = "Field `CH3` reader - DMA Channel active interrupt at priority 1"]
pub type Ch3R = crate::BitReader;
#[doc = "Field `CH4` reader - DMA Channel active interrupt at priority 1"]
pub type Ch4R = crate::BitReader;
#[doc = "Field `CH5` reader - DMA Channel active interrupt at priority 1"]
pub type Ch5R = crate::BitReader;
#[doc = "Field `CH6` reader - DMA Channel active interrupt at priority 1"]
pub type Ch6R = crate::BitReader;
#[doc = "Field `CH7` reader - DMA Channel active interrupt at priority 1"]
pub type Ch7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch0(&self) -> Ch0R {
        Ch0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch1(&self) -> Ch1R {
        Ch1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch2(&self) -> Ch2R {
        Ch2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch3(&self) -> Ch3R {
        Ch3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch4(&self) -> Ch4R {
        Ch4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch5(&self) -> Ch5R {
        Ch5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch6(&self) -> Ch6R {
        Ch6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA Channel active interrupt at priority 1"]
    #[inline(always)]
    pub fn ch7(&self) -> Ch7R {
        Ch7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA INTERRUPT PRIORITY 1 STATUS REGISTER\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Intstat1Spec;
impl crate::RegisterSpec for Intstat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat1::R`](R) reader structure"]
impl crate::Readable for Intstat1Spec {}
#[doc = "`reset()` method sets INTSTAT1 to value 0"]
impl crate::Resettable for Intstat1Spec {}
