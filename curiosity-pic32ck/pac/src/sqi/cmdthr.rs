#[doc = "Register `CMDTHR` reader"]
pub type R = crate::R<CmdthrSpec>;
#[doc = "Register `CMDTHR` writer"]
pub type W = crate::W<CmdthrSpec>;
#[doc = "Field `RXCMDTHR` reader - In RX initiation mode, SPI attempts to perform receive fetch operations until RX_CMD_THRES bytes of space remain in the receive buffer. If space for RX_CMD_THRES bytes is not present in the FIFO, then SPI would not initiate any transfer. RX_CMD_THRES should only be programmed when RX is not active, during IDLE or TX. Please note that if S/W performs any reads, thereby reducing the FIFO count, then HW would initiate RX transfer to get back FIFO count equal to RX_CMD_THRES. If S/W would not like any more words latched into FIFO, then command initiation mode needs to be changed to IDLE before any FIFO reads by S/W. In case of Boot/XIP mode, the SPI Controller uses the AHB Burst size, instead of RX CMD Threshold value."]
pub type RxcmdthrR = crate::FieldReader;
#[doc = "Field `RXCMDTHR` writer - In RX initiation mode, SPI attempts to perform receive fetch operations until RX_CMD_THRES bytes of space remain in the receive buffer. If space for RX_CMD_THRES bytes is not present in the FIFO, then SPI would not initiate any transfer. RX_CMD_THRES should only be programmed when RX is not active, during IDLE or TX. Please note that if S/W performs any reads, thereby reducing the FIFO count, then HW would initiate RX transfer to get back FIFO count equal to RX_CMD_THRES. If S/W would not like any more words latched into FIFO, then command initiation mode needs to be changed to IDLE before any FIFO reads by S/W. In case of Boot/XIP mode, the SPI Controller uses the AHB Burst size, instead of RX CMD Threshold value."]
pub type RxcmdthrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXCMDTHR` reader - In TX Initiation Mode, SPI Performs a transmit Operation when TX_CMD_THRES bytes are present in the TX FIFO. This should usually be set to 1 for normal flash commands, and is desired to be set to a higher value for page programming. NOTE: Value of 8?d0 should be treated as Reserved (illegal) for this field. May produce unexpected activity on SPI bus. Furthermore, values should generally be limited to EVEN values (excepting a value of 1), and MUST be so limited in DMA mode."]
pub type TxcmdthrR = crate::FieldReader;
#[doc = "Field `TXCMDTHR` writer - In TX Initiation Mode, SPI Performs a transmit Operation when TX_CMD_THRES bytes are present in the TX FIFO. This should usually be set to 1 for normal flash commands, and is desired to be set to a higher value for page programming. NOTE: Value of 8?d0 should be treated as Reserved (illegal) for this field. May produce unexpected activity on SPI bus. Furthermore, values should generally be limited to EVEN values (excepting a value of 1), and MUST be so limited in DMA mode."]
pub type TxcmdthrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In RX initiation mode, SPI attempts to perform receive fetch operations until RX_CMD_THRES bytes of space remain in the receive buffer. If space for RX_CMD_THRES bytes is not present in the FIFO, then SPI would not initiate any transfer. RX_CMD_THRES should only be programmed when RX is not active, during IDLE or TX. Please note that if S/W performs any reads, thereby reducing the FIFO count, then HW would initiate RX transfer to get back FIFO count equal to RX_CMD_THRES. If S/W would not like any more words latched into FIFO, then command initiation mode needs to be changed to IDLE before any FIFO reads by S/W. In case of Boot/XIP mode, the SPI Controller uses the AHB Burst size, instead of RX CMD Threshold value."]
    #[inline(always)]
    pub fn rxcmdthr(&self) -> RxcmdthrR {
        RxcmdthrR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - In TX Initiation Mode, SPI Performs a transmit Operation when TX_CMD_THRES bytes are present in the TX FIFO. This should usually be set to 1 for normal flash commands, and is desired to be set to a higher value for page programming. NOTE: Value of 8?d0 should be treated as Reserved (illegal) for this field. May produce unexpected activity on SPI bus. Furthermore, values should generally be limited to EVEN values (excepting a value of 1), and MUST be so limited in DMA mode."]
    #[inline(always)]
    pub fn txcmdthr(&self) -> TxcmdthrR {
        TxcmdthrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - In RX initiation mode, SPI attempts to perform receive fetch operations until RX_CMD_THRES bytes of space remain in the receive buffer. If space for RX_CMD_THRES bytes is not present in the FIFO, then SPI would not initiate any transfer. RX_CMD_THRES should only be programmed when RX is not active, during IDLE or TX. Please note that if S/W performs any reads, thereby reducing the FIFO count, then HW would initiate RX transfer to get back FIFO count equal to RX_CMD_THRES. If S/W would not like any more words latched into FIFO, then command initiation mode needs to be changed to IDLE before any FIFO reads by S/W. In case of Boot/XIP mode, the SPI Controller uses the AHB Burst size, instead of RX CMD Threshold value."]
    #[inline(always)]
    pub fn rxcmdthr(&mut self) -> RxcmdthrW<'_, CmdthrSpec> {
        RxcmdthrW::new(self, 0)
    }
    #[doc = "Bits 8:15 - In TX Initiation Mode, SPI Performs a transmit Operation when TX_CMD_THRES bytes are present in the TX FIFO. This should usually be set to 1 for normal flash commands, and is desired to be set to a higher value for page programming. NOTE: Value of 8?d0 should be treated as Reserved (illegal) for this field. May produce unexpected activity on SPI bus. Furthermore, values should generally be limited to EVEN values (excepting a value of 1), and MUST be so limited in DMA mode."]
    #[inline(always)]
    pub fn txcmdthr(&mut self) -> TxcmdthrW<'_, CmdthrSpec> {
        TxcmdthrW::new(self, 8)
    }
}
#[doc = "SPI command threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`cmdthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmdthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmdthrSpec;
impl crate::RegisterSpec for CmdthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmdthr::R`](R) reader structure"]
impl crate::Readable for CmdthrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmdthr::W`](W) writer structure"]
impl crate::Writable for CmdthrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMDTHR to value 0x0101"]
impl crate::Resettable for CmdthrSpec {
    const RESET_VALUE: u32 = 0x0101;
}
