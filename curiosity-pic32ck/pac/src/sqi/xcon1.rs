#[doc = "Register `XCON1` reader"]
pub type R = crate::R<Xcon1Spec>;
#[doc = "Register `XCON1` writer"]
pub type W = crate::W<Xcon1Spec>;
#[doc = "Field `TYPECMD` reader - SPI TYPE Command - Single lane mode, - Dual lane mode, - Quad lane mode, - Reserved for Future use, Based on this field, the boot controller will send the command in 1/2/4 lane."]
pub type TypecmdR = crate::FieldReader;
#[doc = "Field `TYPECMD` writer - SPI TYPE Command - Single lane mode, - Dual lane mode, - Quad lane mode, - Reserved for Future use, Based on this field, the boot controller will send the command in 1/2/4 lane."]
pub type TypecmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPEADDR` reader - SPI TYPE Address 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will send the Address in 1/2/4 lane."]
pub type TypeaddrR = crate::FieldReader;
#[doc = "Field `TYPEADDR` writer - SPI TYPE Address 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will send the Address in 1/2/4 lane."]
pub type TypeaddrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPEMODE` reader - SPI TYPE Mode: 00 - Single lane mode, 01 - Dual lane mode, 10 - Quad lane mode, 11 - Reserved for Future use. Based on this field, the boot controller will send the Mode in 1/2/4 lane."]
pub type TypemodeR = crate::FieldReader;
#[doc = "Field `TYPEMODE` writer - SPI TYPE Mode: 00 - Single lane mode, 01 - Dual lane mode, 10 - Quad lane mode, 11 - Reserved for Future use. Based on this field, the boot controller will send the Mode in 1/2/4 lane."]
pub type TypemodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPEDUMMY` reader - SPI TYPE DUMMY: 00 - Single lane mode ,01 - Dual lane mode,10 - Quad lane mode ,11 - Reserved for Future use. Based on this field, the boot controller will send the Dummy in 1/2/4 lane."]
pub type TypedummyR = crate::FieldReader;
#[doc = "Field `TYPEDUMMY` writer - SPI TYPE DUMMY: 00 - Single lane mode ,01 - Dual lane mode,10 - Quad lane mode ,11 - Reserved for Future use. Based on this field, the boot controller will send the Dummy in 1/2/4 lane."]
pub type TypedummyW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TYPEDATA` reader - SPI TYPE DATA: 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will receive the data in 1/2/4 lane."]
pub type TypedataR = crate::FieldReader;
#[doc = "Field `TYPEDATA` writer - SPI TYPE DATA: 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will receive the data in 1/2/4 lane."]
pub type TypedataW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `READOPCODE` reader - 8bit opcode value for Read operation"]
pub type ReadopcodeR = crate::FieldReader;
#[doc = "Field `READOPCODE` writer - 8bit opcode value for Read operation"]
pub type ReadopcodeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ADDRBYTES` reader - 000 - Zero Address Bytes ,001 - 1 Address Byte ,010 - 2 Address Bytes ,011 - 3 Address Bytes ,100 - 4 Address Bytes ,101 - 111- Reserved for Future Use"]
pub type AddrbytesR = crate::FieldReader;
#[doc = "Field `ADDRBYTES` writer - 000 - Zero Address Bytes ,001 - 1 Address Byte ,010 - 2 Address Bytes ,011 - 3 Address Bytes ,100 - 4 Address Bytes ,101 - 111- Reserved for Future Use"]
pub type AddrbytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DUMMYBYTES` reader - 000 - Zero Dummy bytes ,001 - Transmit one dummy byte (8?h ff) ,010 - Transmit two dummy bytes (16?h ffff) ,011 - Transmit three dummy bytes (24?h ffffff) ,111 - Transmit Seven dummy bytes"]
pub type DummybytesR = crate::FieldReader;
#[doc = "Field `DUMMYBYTES` writer - 000 - Zero Dummy bytes ,001 - Transmit one dummy byte (8?h ff) ,010 - Transmit two dummy bytes (16?h ffff) ,011 - Transmit three dummy bytes (24?h ffffff) ,111 - Transmit Seven dummy bytes"]
pub type DummybytesW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DDRCMD` reader - XIP SPI TYPE CMD DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the command in SDR/DDR mode."]
pub type DdrcmdR = crate::BitReader;
#[doc = "Field `DDRCMD` writer - XIP SPI TYPE CMD DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the command in SDR/DDR mode."]
pub type DdrcmdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRADDR` reader - XIP SPI TYPE ADDR DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the address bytes in SDR/DDR mode."]
pub type DdraddrR = crate::BitReader;
#[doc = "Field `DDRADDR` writer - XIP SPI TYPE ADDR DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the address bytes in SDR/DDR mode."]
pub type DdraddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRMODE` reader - XIP SPI TYPE MODE DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the mode bytes in SDR/DDR mode."]
pub type DdrmodeR = crate::BitReader;
#[doc = "Field `DDRMODE` writer - XIP SPI TYPE MODE DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the mode bytes in SDR/DDR mode."]
pub type DdrmodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRDUMMY` reader - XIP SPI TYPE DUMMY DDR. 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the dummy bytes in SDR/DDR mode."]
pub type DdrdummyR = crate::BitReader;
#[doc = "Field `DDRDUMMY` writer - XIP SPI TYPE DUMMY DDR. 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the dummy bytes in SDR/DDR mode."]
pub type DdrdummyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DDRDATA` reader - XIP SPI TYPE DATA DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the data in SDR/DDR mode."]
pub type DdrdataR = crate::BitReader;
#[doc = "Field `DDRDATA` writer - XIP SPI TYPE DATA DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the data in SDR/DDR mode."]
pub type DdrdataW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDRCMD` reader - SPI TYPE CMD SDR2DDR: This bit is used by the controller only when XIP_SPI_TYPE_CMD_DDR is 1?b1 i.e., when command sent in DDR mode. 0 - The command (XIP_SPI_READ_OPCODE) will take 4 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in ddr mode. Here the command will go as 1byte only. 1- The command (XIP_SPI_READ_OPCODE) will take 8 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in sdr mode, but user wants to send in ddr mode. Here the command will go as 2bytes. The value (single byte) programmed in XIP_SPI_READ_OPCODE is replicated into 2bytes by the controller and sent on spi bus. Example for CMD SDR2DDR bit 1: Say opcode = 'h ac (1010 1100) Replicated pattern 11 00 11 00 11 11 00 00 ('h ccf0). So 1st 4 clock cycles hcc; next 4 clock cycles hf0 goes on spi bus."]
pub type SdrcmdR = crate::BitReader;
#[doc = "Field `SDRCMD` writer - SPI TYPE CMD SDR2DDR: This bit is used by the controller only when XIP_SPI_TYPE_CMD_DDR is 1?b1 i.e., when command sent in DDR mode. 0 - The command (XIP_SPI_READ_OPCODE) will take 4 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in ddr mode. Here the command will go as 1byte only. 1- The command (XIP_SPI_READ_OPCODE) will take 8 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in sdr mode, but user wants to send in ddr mode. Here the command will go as 2bytes. The value (single byte) programmed in XIP_SPI_READ_OPCODE is replicated into 2bytes by the controller and sent on spi bus. Example for CMD SDR2DDR bit 1: Say opcode = 'h ac (1010 1100) Replicated pattern 11 00 11 00 11 11 00 00 ('h ccf0). So 1st 4 clock cycles hcc; next 4 clock cycles hf0 goes on spi bus."]
pub type SdrcmdW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SPI TYPE Command - Single lane mode, - Dual lane mode, - Quad lane mode, - Reserved for Future use, Based on this field, the boot controller will send the command in 1/2/4 lane."]
    #[inline(always)]
    pub fn typecmd(&self) -> TypecmdR {
        TypecmdR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - SPI TYPE Address 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will send the Address in 1/2/4 lane."]
    #[inline(always)]
    pub fn typeaddr(&self) -> TypeaddrR {
        TypeaddrR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - SPI TYPE Mode: 00 - Single lane mode, 01 - Dual lane mode, 10 - Quad lane mode, 11 - Reserved for Future use. Based on this field, the boot controller will send the Mode in 1/2/4 lane."]
    #[inline(always)]
    pub fn typemode(&self) -> TypemodeR {
        TypemodeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - SPI TYPE DUMMY: 00 - Single lane mode ,01 - Dual lane mode,10 - Quad lane mode ,11 - Reserved for Future use. Based on this field, the boot controller will send the Dummy in 1/2/4 lane."]
    #[inline(always)]
    pub fn typedummy(&self) -> TypedummyR {
        TypedummyR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SPI TYPE DATA: 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will receive the data in 1/2/4 lane."]
    #[inline(always)]
    pub fn typedata(&self) -> TypedataR {
        TypedataR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:17 - 8bit opcode value for Read operation"]
    #[inline(always)]
    pub fn readopcode(&self) -> ReadopcodeR {
        ReadopcodeR::new(((self.bits >> 10) & 0xff) as u8)
    }
    #[doc = "Bits 18:20 - 000 - Zero Address Bytes ,001 - 1 Address Byte ,010 - 2 Address Bytes ,011 - 3 Address Bytes ,100 - 4 Address Bytes ,101 - 111- Reserved for Future Use"]
    #[inline(always)]
    pub fn addrbytes(&self) -> AddrbytesR {
        AddrbytesR::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - 000 - Zero Dummy bytes ,001 - Transmit one dummy byte (8?h ff) ,010 - Transmit two dummy bytes (16?h ffff) ,011 - Transmit three dummy bytes (24?h ffffff) ,111 - Transmit Seven dummy bytes"]
    #[inline(always)]
    pub fn dummybytes(&self) -> DummybytesR {
        DummybytesR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - XIP SPI TYPE CMD DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the command in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrcmd(&self) -> DdrcmdR {
        DdrcmdR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - XIP SPI TYPE ADDR DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the address bytes in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddraddr(&self) -> DdraddrR {
        DdraddrR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - XIP SPI TYPE MODE DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the mode bytes in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrmode(&self) -> DdrmodeR {
        DdrmodeR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - XIP SPI TYPE DUMMY DDR. 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the dummy bytes in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrdummy(&self) -> DdrdummyR {
        DdrdummyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - XIP SPI TYPE DATA DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the data in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrdata(&self) -> DdrdataR {
        DdrdataR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SPI TYPE CMD SDR2DDR: This bit is used by the controller only when XIP_SPI_TYPE_CMD_DDR is 1?b1 i.e., when command sent in DDR mode. 0 - The command (XIP_SPI_READ_OPCODE) will take 4 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in ddr mode. Here the command will go as 1byte only. 1- The command (XIP_SPI_READ_OPCODE) will take 8 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in sdr mode, but user wants to send in ddr mode. Here the command will go as 2bytes. The value (single byte) programmed in XIP_SPI_READ_OPCODE is replicated into 2bytes by the controller and sent on spi bus. Example for CMD SDR2DDR bit 1: Say opcode = 'h ac (1010 1100) Replicated pattern 11 00 11 00 11 11 00 00 ('h ccf0). So 1st 4 clock cycles hcc; next 4 clock cycles hf0 goes on spi bus."]
    #[inline(always)]
    pub fn sdrcmd(&self) -> SdrcmdR {
        SdrcmdR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SPI TYPE Command - Single lane mode, - Dual lane mode, - Quad lane mode, - Reserved for Future use, Based on this field, the boot controller will send the command in 1/2/4 lane."]
    #[inline(always)]
    pub fn typecmd(&mut self) -> TypecmdW<'_, Xcon1Spec> {
        TypecmdW::new(self, 0)
    }
    #[doc = "Bits 2:3 - SPI TYPE Address 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will send the Address in 1/2/4 lane."]
    #[inline(always)]
    pub fn typeaddr(&mut self) -> TypeaddrW<'_, Xcon1Spec> {
        TypeaddrW::new(self, 2)
    }
    #[doc = "Bits 4:5 - SPI TYPE Mode: 00 - Single lane mode, 01 - Dual lane mode, 10 - Quad lane mode, 11 - Reserved for Future use. Based on this field, the boot controller will send the Mode in 1/2/4 lane."]
    #[inline(always)]
    pub fn typemode(&mut self) -> TypemodeW<'_, Xcon1Spec> {
        TypemodeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - SPI TYPE DUMMY: 00 - Single lane mode ,01 - Dual lane mode,10 - Quad lane mode ,11 - Reserved for Future use. Based on this field, the boot controller will send the Dummy in 1/2/4 lane."]
    #[inline(always)]
    pub fn typedummy(&mut self) -> TypedummyW<'_, Xcon1Spec> {
        TypedummyW::new(self, 6)
    }
    #[doc = "Bits 8:9 - SPI TYPE DATA: 00 - Single lane mode ,01 - Dual lane mode ,10 - Quad lane mode ,11 - Reserved for Future use Based on this field, the boot controller will receive the data in 1/2/4 lane."]
    #[inline(always)]
    pub fn typedata(&mut self) -> TypedataW<'_, Xcon1Spec> {
        TypedataW::new(self, 8)
    }
    #[doc = "Bits 10:17 - 8bit opcode value for Read operation"]
    #[inline(always)]
    pub fn readopcode(&mut self) -> ReadopcodeW<'_, Xcon1Spec> {
        ReadopcodeW::new(self, 10)
    }
    #[doc = "Bits 18:20 - 000 - Zero Address Bytes ,001 - 1 Address Byte ,010 - 2 Address Bytes ,011 - 3 Address Bytes ,100 - 4 Address Bytes ,101 - 111- Reserved for Future Use"]
    #[inline(always)]
    pub fn addrbytes(&mut self) -> AddrbytesW<'_, Xcon1Spec> {
        AddrbytesW::new(self, 18)
    }
    #[doc = "Bits 21:23 - 000 - Zero Dummy bytes ,001 - Transmit one dummy byte (8?h ff) ,010 - Transmit two dummy bytes (16?h ffff) ,011 - Transmit three dummy bytes (24?h ffffff) ,111 - Transmit Seven dummy bytes"]
    #[inline(always)]
    pub fn dummybytes(&mut self) -> DummybytesW<'_, Xcon1Spec> {
        DummybytesW::new(self, 21)
    }
    #[doc = "Bit 24 - XIP SPI TYPE CMD DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the command in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrcmd(&mut self) -> DdrcmdW<'_, Xcon1Spec> {
        DdrcmdW::new(self, 24)
    }
    #[doc = "Bit 25 - XIP SPI TYPE ADDR DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the address bytes in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddraddr(&mut self) -> DdraddrW<'_, Xcon1Spec> {
        DdraddrW::new(self, 25)
    }
    #[doc = "Bit 26 - XIP SPI TYPE MODE DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the mode bytes in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrmode(&mut self) -> DdrmodeW<'_, Xcon1Spec> {
        DdrmodeW::new(self, 26)
    }
    #[doc = "Bit 27 - XIP SPI TYPE DUMMY DDR. 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the dummy bytes in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrdummy(&mut self) -> DdrdummyW<'_, Xcon1Spec> {
        DdrdummyW::new(self, 27)
    }
    #[doc = "Bit 28 - XIP SPI TYPE DATA DDR 0 - SDR mode, 1 - DDR mode. Based on this field, the boot controller will send the data in SDR/DDR mode."]
    #[inline(always)]
    pub fn ddrdata(&mut self) -> DdrdataW<'_, Xcon1Spec> {
        DdrdataW::new(self, 28)
    }
    #[doc = "Bit 29 - SPI TYPE CMD SDR2DDR: This bit is used by the controller only when XIP_SPI_TYPE_CMD_DDR is 1?b1 i.e., when command sent in DDR mode. 0 - The command (XIP_SPI_READ_OPCODE) will take 4 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in ddr mode. Here the command will go as 1byte only. 1- The command (XIP_SPI_READ_OPCODE) will take 8 clock cycles. Data will go in both the edges. Used when Flash expecting the cmd in sdr mode, but user wants to send in ddr mode. Here the command will go as 2bytes. The value (single byte) programmed in XIP_SPI_READ_OPCODE is replicated into 2bytes by the controller and sent on spi bus. Example for CMD SDR2DDR bit 1: Say opcode = 'h ac (1010 1100) Replicated pattern 11 00 11 00 11 11 00 00 ('h ccf0). So 1st 4 clock cycles hcc; next 4 clock cycles hf0 goes on spi bus."]
    #[inline(always)]
    pub fn sdrcmd(&mut self) -> SdrcmdW<'_, Xcon1Spec> {
        SdrcmdW::new(self, 29)
    }
}
#[doc = "SPI XIP Control1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`xcon1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xcon1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Xcon1Spec;
impl crate::RegisterSpec for Xcon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xcon1::R`](R) reader structure"]
impl crate::Readable for Xcon1Spec {}
#[doc = "`write(|w| ..)` method takes [`xcon1::W`](W) writer structure"]
impl crate::Writable for Xcon1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets XCON1 to value 0x202c_0c00"]
impl crate::Resettable for Xcon1Spec {
    const RESET_VALUE: u32 = 0x202c_0c00;
}
