#[doc = "Register `MBCONFIG` reader"]
pub type R = crate::R<MbconfigSpec>;
#[doc = "Register `MBCONFIG` writer"]
pub type W = crate::W<MbconfigSpec>;
#[doc = "Field `TXINT` reader - MAILBOX TRANSMIT INTERRUPT ENABLE"]
pub type TxintR = crate::BitReader;
#[doc = "Field `TXINT` writer - MAILBOX TRANSMIT INTERRUPT ENABLE"]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXINT` reader - MAILBOX RECEIVE INTERRUPT ENABLE"]
pub type RxintR = crate::BitReader;
#[doc = "Field `RXINT` writer - MAILBOX RECEIVE INTERRUPT ENABLE"]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MAILBOX TRANSMIT INTERRUPT ENABLE"]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - MAILBOX RECEIVE INTERRUPT ENABLE"]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MAILBOX TRANSMIT INTERRUPT ENABLE"]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, MbconfigSpec> {
        TxintW::new(self, 0)
    }
    #[doc = "Bit 1 - MAILBOX RECEIVE INTERRUPT ENABLE"]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, MbconfigSpec> {
        RxintW::new(self, 1)
    }
}
#[doc = "MAILBOX CONFIGURATION\n\nYou can [`read`](crate::Reg::read) this register and get [`mbconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbconfigSpec;
impl crate::RegisterSpec for MbconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbconfig::R`](R) reader structure"]
impl crate::Readable for MbconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`mbconfig::W`](W) writer structure"]
impl crate::Writable for MbconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBCONFIG to value 0"]
impl crate::Resettable for MbconfigSpec {}
