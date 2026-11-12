#[doc = "Register `RXMAXP` reader"]
pub type R = crate::R<RxmaxpSpec>;
#[doc = "Register `RXMAXP` writer"]
pub type W = crate::W<RxmaxpSpec>;
#[doc = "Field `MAXPAYLOAD` reader - Maximum Payload/Transactions"]
pub type MaxpayloadR = crate::FieldReader<u16>;
#[doc = "Field `MAXPAYLOAD` writer - Maximum Payload/Transactions"]
pub type MaxpayloadW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `MULTIPLIER` reader - Multiplier Value (m-1)"]
pub type MultiplierR = crate::FieldReader;
#[doc = "Field `MULTIPLIER` writer - Multiplier Value (m-1)"]
pub type MultiplierW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:10 - Maximum Payload/Transactions"]
    #[inline(always)]
    pub fn maxpayload(&self) -> MaxpayloadR {
        MaxpayloadR::new(self.bits & 0x07ff)
    }
    #[doc = "Bits 11:15 - Multiplier Value (m-1)"]
    #[inline(always)]
    pub fn multiplier(&self) -> MultiplierR {
        MultiplierR::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Maximum Payload/Transactions"]
    #[inline(always)]
    pub fn maxpayload(&mut self) -> MaxpayloadW<'_, RxmaxpSpec> {
        MaxpayloadW::new(self, 0)
    }
    #[doc = "Bits 11:15 - Multiplier Value (m-1)"]
    #[inline(always)]
    pub fn multiplier(&mut self) -> MultiplierW<'_, RxmaxpSpec> {
        MultiplierW::new(self, 11)
    }
}
#[doc = "ENDPOINTX Maximum Amount Of Data That Can Be Transferred Through The Selected RX Endpoint In A Single Operation\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmaxp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmaxp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmaxpSpec;
impl crate::RegisterSpec for RxmaxpSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`rxmaxp::R`](R) reader structure"]
impl crate::Readable for RxmaxpSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmaxp::W`](W) writer structure"]
impl crate::Writable for RxmaxpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXMAXP to value 0"]
impl crate::Resettable for RxmaxpSpec {}
