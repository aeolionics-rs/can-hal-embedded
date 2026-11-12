#[doc = "Register `CON` reader"]
pub type R = crate::R<ConSpec>;
#[doc = "Register `CON` writer"]
pub type W = crate::W<ConSpec>;
#[doc = "Field `TXRXCOUNT` reader - This bit specifies the total number of bytes to transmit or receive (based on CMD_INIT field). ,16?d 0 - Reserved,16?d 1 - 1byte to transmit/receive ,16?d 2 - 2bytes to transmit/receive NOTE: Count must be programmed to non-zero value prior to initiating transaction"]
pub type TxrxcountR = crate::FieldReader<u16>;
#[doc = "Field `TXRXCOUNT` writer - This bit specifies the total number of bytes to transmit or receive (based on CMD_INIT field). ,16?d 0 - Reserved,16?d 1 - 1byte to transmit/receive ,16?d 2 - 2bytes to transmit/receive NOTE: Count must be programmed to non-zero value prior to initiating transaction"]
pub type TxrxcountW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CMDINIT` reader - This signal indicates the command initiation mode. If it is Transmit, Commands are initiated based on writes to transmit register or the contents of TX FIFO. If the CMD_INIT is Receive, commands are initiated based on reads to Read register or RX FIFO availability. 00 = Reserved. 01 = Initiate Transmit. 10 = Initiate Receive. 11 = Reserved. Note: If Boot mode is enabled, the default value is 2?b10 This is NOT active status field, see SPI Status2 Reg (0x30)"]
pub type CmdinitR = crate::FieldReader;
#[doc = "Field `CMDINIT` writer - This signal indicates the command initiation mode. If it is Transmit, Commands are initiated based on writes to transmit register or the contents of TX FIFO. If the CMD_INIT is Receive, commands are initiated based on reads to Read register or RX FIFO availability. 00 = Reserved. 01 = Initiate Transmit. 10 = Initiate Receive. 11 = Reserved. Note: If Boot mode is enabled, the default value is 2?b10 This is NOT active status field, see SPI Status2 Reg (0x30)"]
pub type CmdinitW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LANEMODE` reader - SPI Lane mode 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use"]
pub type LanemodeR = crate::FieldReader;
#[doc = "Field `LANEMODE` writer - SPI Lane mode 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use"]
pub type LanemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DEVSEL` reader - SPI Device Select: This field along with bit25 is used to select a particular SPI device. Note: For Michigan Ax versions, extension by bit25 is NOT used (only 4 devices are selectable, by bits 21:20 here)."]
pub type DevselR = crate::FieldReader;
#[doc = "Field `DEVSEL` writer - SPI Device Select: This field along with bit25 is used to select a particular SPI device. Note: For Michigan Ax versions, extension by bit25 is NOT used (only 4 devices are selectable, by bits 21:20 here)."]
pub type DevselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DASSERT` reader - Chip Select Assert 0 - CS is not de-asserted after transmission/reception of specified number of bytes in the control register 1 - CS is de-asserted after transmission/reception of specified number of bytes in the control register"]
pub type DassertR = crate::BitReader;
#[doc = "Field `DASSERT` writer - Chip Select Assert 0 - CS is not de-asserted after transmission/reception of specified number of bytes in the control register 1 - CS is de-asserted after transmission/reception of specified number of bytes in the control register"]
pub type DassertW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRMODE` reader - SDR_DDR: 0 - SDR mode, 1 - DDR mode. Note: For Michigan Ax versions this bit is Reserved and should be kept 0."]
pub type DdrmodeR = crate::BitReader;
#[doc = "Field `DDRMODE` writer - SDR_DDR: 0 - SDR mode, 1 - DDR mode. Note: For Michigan Ax versions this bit is Reserved and should be kept 0."]
pub type DdrmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCHECK` reader - Status Check This bit is mainly used for Programming or Erase operations. 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: For Michigan Ax versions, this bit is Reserved and should be kept 0."]
pub type ScheckR = crate::BitReader;
#[doc = "Field `SCHECK` writer - Status Check This bit is mainly used for Programming or Erase operations. 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: For Michigan Ax versions, this bit is Reserved and should be kept 0."]
pub type ScheckW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - This bit specifies the total number of bytes to transmit or receive (based on CMD_INIT field). ,16?d 0 - Reserved,16?d 1 - 1byte to transmit/receive ,16?d 2 - 2bytes to transmit/receive NOTE: Count must be programmed to non-zero value prior to initiating transaction"]
    #[inline(always)]
    pub fn txrxcount(&self) -> TxrxcountR {
        TxrxcountR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - This signal indicates the command initiation mode. If it is Transmit, Commands are initiated based on writes to transmit register or the contents of TX FIFO. If the CMD_INIT is Receive, commands are initiated based on reads to Read register or RX FIFO availability. 00 = Reserved. 01 = Initiate Transmit. 10 = Initiate Receive. 11 = Reserved. Note: If Boot mode is enabled, the default value is 2?b10 This is NOT active status field, see SPI Status2 Reg (0x30)"]
    #[inline(always)]
    pub fn cmdinit(&self) -> CmdinitR {
        CmdinitR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - SPI Lane mode 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use"]
    #[inline(always)]
    pub fn lanemode(&self) -> LanemodeR {
        LanemodeR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21 - SPI Device Select: This field along with bit25 is used to select a particular SPI device. Note: For Michigan Ax versions, extension by bit25 is NOT used (only 4 devices are selectable, by bits 21:20 here)."]
    #[inline(always)]
    pub fn devsel(&self) -> DevselR {
        DevselR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Chip Select Assert 0 - CS is not de-asserted after transmission/reception of specified number of bytes in the control register 1 - CS is de-asserted after transmission/reception of specified number of bytes in the control register"]
    #[inline(always)]
    pub fn dassert(&self) -> DassertR {
        DassertR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SDR_DDR: 0 - SDR mode, 1 - DDR mode. Note: For Michigan Ax versions this bit is Reserved and should be kept 0."]
    #[inline(always)]
    pub fn ddrmode(&self) -> DdrmodeR {
        DdrmodeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Status Check This bit is mainly used for Programming or Erase operations. 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: For Michigan Ax versions, this bit is Reserved and should be kept 0."]
    #[inline(always)]
    pub fn scheck(&self) -> ScheckR {
        ScheckR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - This bit specifies the total number of bytes to transmit or receive (based on CMD_INIT field). ,16?d 0 - Reserved,16?d 1 - 1byte to transmit/receive ,16?d 2 - 2bytes to transmit/receive NOTE: Count must be programmed to non-zero value prior to initiating transaction"]
    #[inline(always)]
    pub fn txrxcount(&mut self) -> TxrxcountW<'_, ConSpec> {
        TxrxcountW::new(self, 0)
    }
    #[doc = "Bits 16:17 - This signal indicates the command initiation mode. If it is Transmit, Commands are initiated based on writes to transmit register or the contents of TX FIFO. If the CMD_INIT is Receive, commands are initiated based on reads to Read register or RX FIFO availability. 00 = Reserved. 01 = Initiate Transmit. 10 = Initiate Receive. 11 = Reserved. Note: If Boot mode is enabled, the default value is 2?b10 This is NOT active status field, see SPI Status2 Reg (0x30)"]
    #[inline(always)]
    pub fn cmdinit(&mut self) -> CmdinitW<'_, ConSpec> {
        CmdinitW::new(self, 16)
    }
    #[doc = "Bits 18:19 - SPI Lane mode 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use"]
    #[inline(always)]
    pub fn lanemode(&mut self) -> LanemodeW<'_, ConSpec> {
        LanemodeW::new(self, 18)
    }
    #[doc = "Bits 20:21 - SPI Device Select: This field along with bit25 is used to select a particular SPI device. Note: For Michigan Ax versions, extension by bit25 is NOT used (only 4 devices are selectable, by bits 21:20 here)."]
    #[inline(always)]
    pub fn devsel(&mut self) -> DevselW<'_, ConSpec> {
        DevselW::new(self, 20)
    }
    #[doc = "Bit 22 - Chip Select Assert 0 - CS is not de-asserted after transmission/reception of specified number of bytes in the control register 1 - CS is de-asserted after transmission/reception of specified number of bytes in the control register"]
    #[inline(always)]
    pub fn dassert(&mut self) -> DassertW<'_, ConSpec> {
        DassertW::new(self, 22)
    }
    #[doc = "Bit 23 - SDR_DDR: 0 - SDR mode, 1 - DDR mode. Note: For Michigan Ax versions this bit is Reserved and should be kept 0."]
    #[inline(always)]
    pub fn ddrmode(&mut self) -> DdrmodeW<'_, ConSpec> {
        DdrmodeW::new(self, 23)
    }
    #[doc = "Bit 24 - Status Check This bit is mainly used for Programming or Erase operations. 0 - do not check status, 1 - check status. If this bit is set to 1, the hardware will issue the status command automatically after the current operation and wait for busy bit to clear. The hardware will issue the next command only after this Status check phase. Note: For Michigan Ax versions, this bit is Reserved and should be kept 0."]
    #[inline(always)]
    pub fn scheck(&mut self) -> ScheckW<'_, ConSpec> {
        ScheckW::new(self, 24)
    }
}
#[doc = "SPI Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for ConSpec {}
#[doc = "`write(|w| ..)` method takes [`con::W`](W) writer structure"]
impl crate::Writable for ConSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for ConSpec {}
