#[doc = "Register `MBTXSTATUS` reader"]
pub type R = crate::R<MbtxstatusSpec>;
#[doc = "Register `MBTXSTATUS` writer"]
pub type W = crate::W<MbtxstatusSpec>;
#[doc = "Field `TXREMAINING` reader - TRANSMIT BYTES REMAINING"]
pub type TxremainingR = crate::FieldReader<u16>;
#[doc = "Field `TXREMAINING` writer - TRANSMIT BYTES REMAINING"]
pub type TxremainingW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TXTYPE` reader - TRANSMIT MESSAGE HEADER TYPE"]
pub type TxtypeR = crate::FieldReader;
#[doc = "Field `TXTYPE` writer - TRANSMIT MESSAGE HEADER TYPE"]
pub type TxtypeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TXINT` reader - TRANSMIT FIFO INTERRUPT FLAG"]
pub type TxintR = crate::BitReader;
#[doc = "Field `TXINT` writer - TRANSMIT FIFO INTERRUPT FLAG"]
pub type TxintW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFULL` reader - TRANSMIT FIFO FULL"]
pub type TxfullR = crate::BitReader;
#[doc = "Field `TXFULL` writer - TRANSMIT FIFO FULL"]
pub type TxfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXERR` reader - TRANSMIT ERROR"]
pub type TxerrR = crate::BitReader;
#[doc = "Field `TXERR` writer - TRANSMIT ERROR"]
pub type TxerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - TRANSMIT BYTES REMAINING"]
    #[inline(always)]
    pub fn txremaining(&self) -> TxremainingR {
        TxremainingR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - TRANSMIT MESSAGE HEADER TYPE"]
    #[inline(always)]
    pub fn txtype(&self) -> TxtypeR {
        TxtypeR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - TRANSMIT FIFO INTERRUPT FLAG"]
    #[inline(always)]
    pub fn txint(&self) -> TxintR {
        TxintR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - TRANSMIT FIFO FULL"]
    #[inline(always)]
    pub fn txfull(&self) -> TxfullR {
        TxfullR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 23 - TRANSMIT ERROR"]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - TRANSMIT BYTES REMAINING"]
    #[inline(always)]
    pub fn txremaining(&mut self) -> TxremainingW<'_, MbtxstatusSpec> {
        TxremainingW::new(self, 0)
    }
    #[doc = "Bits 16:19 - TRANSMIT MESSAGE HEADER TYPE"]
    #[inline(always)]
    pub fn txtype(&mut self) -> TxtypeW<'_, MbtxstatusSpec> {
        TxtypeW::new(self, 16)
    }
    #[doc = "Bit 20 - TRANSMIT FIFO INTERRUPT FLAG"]
    #[inline(always)]
    pub fn txint(&mut self) -> TxintW<'_, MbtxstatusSpec> {
        TxintW::new(self, 20)
    }
    #[doc = "Bit 21 - TRANSMIT FIFO FULL"]
    #[inline(always)]
    pub fn txfull(&mut self) -> TxfullW<'_, MbtxstatusSpec> {
        TxfullW::new(self, 21)
    }
    #[doc = "Bit 23 - TRANSMIT ERROR"]
    #[inline(always)]
    pub fn txerr(&mut self) -> TxerrW<'_, MbtxstatusSpec> {
        TxerrW::new(self, 23)
    }
}
#[doc = "MAILBOX TRANSMIT STATUS\n\nYou can [`read`](crate::Reg::read) this register and get [`mbtxstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mbtxstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MbtxstatusSpec;
impl crate::RegisterSpec for MbtxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mbtxstatus::R`](R) reader structure"]
impl crate::Readable for MbtxstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`mbtxstatus::W`](W) writer structure"]
impl crate::Writable for MbtxstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MBTXSTATUS to value 0x0010_0000"]
impl crate::Resettable for MbtxstatusSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
