#[doc = "Register `CSR0H_PERIPHERAL_EP0_MODE` reader"]
pub type R = crate::R<Csr0hPeripheralEp0ModeSpec>;
#[doc = "Register `CSR0H_PERIPHERAL_EP0_MODE` writer"]
pub type W = crate::W<Csr0hPeripheralEp0ModeSpec>;
#[doc = "Field `FLUSHFIFO` reader - Flush The Next Packet To Be Transmitted/Read"]
pub type FlushfifoR = crate::BitReader;
#[doc = "Field `FLUSHFIFO` writer - Flush The Next Packet To Be Transmitted/Read"]
pub type FlushfifoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Flush The Next Packet To Be Transmitted/Read"]
    #[inline(always)]
    pub fn flushfifo(&self) -> FlushfifoR {
        FlushfifoR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Flush The Next Packet To Be Transmitted/Read"]
    #[inline(always)]
    pub fn flushfifo(&mut self) -> FlushfifoW<'_, Csr0hPeripheralEp0ModeSpec> {
        FlushfifoW::new(self, 0)
    }
}
#[doc = "ENDPOINT0 Additional Control And Status Bits For TX Endpoint 0\n\nYou can [`read`](crate::Reg::read) this register and get [`csr0h_peripheral_ep0_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr0h_peripheral_ep0_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr0hPeripheralEp0ModeSpec;
impl crate::RegisterSpec for Csr0hPeripheralEp0ModeSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`csr0h_peripheral_ep0_mode::R`](R) reader structure"]
impl crate::Readable for Csr0hPeripheralEp0ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`csr0h_peripheral_ep0_mode::W`](W) writer structure"]
impl crate::Writable for Csr0hPeripheralEp0ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CSR0H_PERIPHERAL_EP0_MODE to value 0"]
impl crate::Resettable for Csr0hPeripheralEp0ModeSpec {}
