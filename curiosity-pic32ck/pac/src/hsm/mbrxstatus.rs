#[doc = "Register `MBRXSTATUS` reader"]
pub type R = crate::R<MbrxstatusSpec>;
#[doc = "Register `MBRXSTATUS` writer"]
pub type W = crate::W<MbrxstatusSpec>;
#[doc = "Field `RXREMAINING` reader - RECEIVE BYTES REMAINING"]
pub type RxremainingR = crate::FieldReader<u16>;
#[doc = "Field `RXREMAINING` writer - RECEIVE BYTES REMAINING"]
pub type RxremainingW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `RXTYPE` reader - RECEIVE MESSAGE HEADER TYPE"]
pub type RxtypeR = crate::FieldReader;
#[doc = "Field `RXTYPE` writer - RECEIVE MESSAGE HEADER TYPE"]
pub type RxtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RXINT` reader - RECEIVE FIFO INTERRUPT FLAG"]
pub type RxintR = crate::BitReader;
#[doc = "Field `RXINT` writer - RECEIVE FIFO INTERRUPT FLAG"]
pub type RxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXEMPTY` reader - RECEIVE FIFO FULL"]
pub type RxemptyR = crate::BitReader;
#[doc = "Field `RXEMPTY` writer - RECEIVE FIFO FULL"]
pub type RxemptyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXHEAD` reader - Receive Header Next Word bit"]
pub type RxheadR = crate::BitReader;
#[doc = "Field `RXHEAD` writer - Receive Header Next Word bit"]
pub type RxheadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXERR` reader - RECEIVE ERROR"]
pub type RxerrR = crate::BitReader;
#[doc = "Field `RXERR` writer - RECEIVE ERROR"]
pub type RxerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - RECEIVE BYTES REMAINING"]
    #[inline(always)]
    pub fn rxremaining(&self) -> RxremainingR {
        RxremainingR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - RECEIVE MESSAGE HEADER TYPE"]
    #[inline(always)]
    pub fn rxtype(&self) -> RxtypeR {
        RxtypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - RECEIVE FIFO INTERRUPT FLAG"]
    #[inline(always)]
    pub fn rxint(&self) -> RxintR {
        RxintR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - RECEIVE FIFO FULL"]
    #[inline(always)]
    pub fn rxempty(&self) -> RxemptyR {
        RxemptyR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Receive Header Next Word bit"]
    #[inline(always)]
    pub fn rxhead(&self) -> RxheadR {
        RxheadR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - RECEIVE ERROR"]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - RECEIVE BYTES REMAINING"]
    #[inline(always)]
    pub fn rxremaining(&mut self) -> RxremainingW<'_, MbrxstatusSpec> {
        RxremainingW::new(self, 0)
    }
    #[doc = "Bits 16:19 - RECEIVE MESSAGE HEADER TYPE"]
    #[inline(always)]
    pub fn rxtype(&mut self) -> RxtypeW<'_, MbrxstatusSpec> {
        RxtypeW::new(self, 16)
    }
    #[doc = "Bit 20 - RECEIVE FIFO INTERRUPT FLAG"]
    #[inline(always)]
    pub fn rxint(&mut self) -> RxintW<'_, MbrxstatusSpec> {
        RxintW::new(self, 20)
    }
    #[doc = "Bit 21 - RECEIVE FIFO FULL"]
    #[inline(always)]
    pub fn rxempty(&mut self) -> RxemptyW<'_, MbrxstatusSpec> {
        RxemptyW::new(self, 21)
    }
    #[doc = "Bit 22 - Receive Header Next Word bit"]
    #[inline(always)]
    pub fn rxhead(&mut self) -> RxheadW<'_, MbrxstatusSpec> {
        RxheadW::new(self, 22)
    }
    #[doc = "Bit 23 - RECEIVE ERROR"]
    #[inline(always)]
    pub fn rxerr(&mut self) -> RxerrW<'_, MbrxstatusSpec> {
        RxerrW::new(self, 23)
    }
}
#[doc = "MAILBOX RECEIVE STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mbrxstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbrxstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbrxstatusSpec;
impl crate::RegisterSpec for MbrxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbrxstatus::R`](R) reader structure"]
impl crate::Readable for MbrxstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mbrxstatus::W`](W) writer structure"]
impl crate::Writable for MbrxstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBRXSTATUS to value 0x0020_0000"]
impl crate::Resettable for MbrxstatusSpec {
    const RESET_VALUE: u32 = 0x0020_0000;
}
