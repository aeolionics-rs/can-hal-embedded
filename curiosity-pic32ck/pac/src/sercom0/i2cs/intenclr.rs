#[doc = "Register `INTENCLR` reader"]
pub type R = crate::R<IntenclrSpec>;
#[doc = "Register `INTENCLR` writer"]
pub type W = crate::W<IntenclrSpec>;
#[doc = "Field `PREC` reader - Stop Received Interrupt Disable"]
pub type PrecR = crate::BitReader;
#[doc = "Field `PREC` writer - Stop Received Interrupt Disable"]
pub type PrecW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `AMATCH` reader - Address Match Interrupt Disable"]
pub type AmatchR = crate::BitReader;
#[doc = "Field `AMATCH` writer - Address Match Interrupt Disable"]
pub type AmatchW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DRDY` reader - Data Interrupt Disable"]
pub type DrdyR = crate::BitReader;
#[doc = "Field `DRDY` writer - Data Interrupt Disable"]
pub type DrdyW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TXFE` reader - Tx FIFO Empty Interrupt Disable"]
pub type TxfeR = crate::BitReader;
#[doc = "Field `TXFE` writer - Tx FIFO Empty Interrupt Disable"]
pub type TxfeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RXFF` reader - Rx FIFO Full Interrupt Disable"]
pub type RxffR = crate::BitReader;
#[doc = "Field `RXFF` writer - Rx FIFO Full Interrupt Disable"]
pub type RxffW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ERROR` reader - Combined Error Interrupt Disable"]
pub type ErrorR = crate::BitReader;
#[doc = "Field `ERROR` writer - Combined Error Interrupt Disable"]
pub type ErrorW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Stop Received Interrupt Disable"]
    #[inline(always)]
    pub fn prec(&self) -> PrecR {
        PrecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Disable"]
    #[inline(always)]
    pub fn amatch(&self) -> AmatchR {
        AmatchR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data Interrupt Disable"]
    #[inline(always)]
    pub fn drdy(&self) -> DrdyR {
        DrdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Tx FIFO Empty Interrupt Disable"]
    #[inline(always)]
    pub fn txfe(&self) -> TxfeR {
        TxfeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxff(&self) -> RxffR {
        RxffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Received Interrupt Disable"]
    #[inline(always)]
    pub fn prec(&mut self) -> PrecW<'_, IntenclrSpec> {
        PrecW::new(self, 0)
    }
    #[doc = "Bit 1 - Address Match Interrupt Disable"]
    #[inline(always)]
    pub fn amatch(&mut self) -> AmatchW<'_, IntenclrSpec> {
        AmatchW::new(self, 1)
    }
    #[doc = "Bit 2 - Data Interrupt Disable"]
    #[inline(always)]
    pub fn drdy(&mut self) -> DrdyW<'_, IntenclrSpec> {
        DrdyW::new(self, 2)
    }
    #[doc = "Bit 3 - Tx FIFO Empty Interrupt Disable"]
    #[inline(always)]
    pub fn txfe(&mut self) -> TxfeW<'_, IntenclrSpec> {
        TxfeW::new(self, 3)
    }
    #[doc = "Bit 4 - Rx FIFO Full Interrupt Disable"]
    #[inline(always)]
    pub fn rxff(&mut self) -> RxffW<'_, IntenclrSpec> {
        RxffW::new(self, 4)
    }
    #[doc = "Bit 7 - Combined Error Interrupt Disable"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<'_, IntenclrSpec> {
        ErrorW::new(self, 7)
    }
}
#[doc = "I2CS Interrupt Enable Clear\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenclrSpec;
impl crate::RegisterSpec for IntenclrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intenclr::R`](R) reader structure"]
impl crate::Readable for IntenclrSpec {}
#[doc = "`write(|w| ..)` method takes [`intenclr::W`](W) writer structure"]
impl crate::Writable for IntenclrSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0x9f;
}
#[doc = "`reset()` method sets INTENCLR to value 0"]
impl crate::Resettable for IntenclrSpec {}
