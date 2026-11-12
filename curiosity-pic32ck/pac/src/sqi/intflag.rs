#[doc = "Register `INTFLAG` reader"]
pub type R = crate::R<IntflagSpec>;
#[doc = "Register `INTFLAG` writer"]
pub type W = crate::W<IntflagSpec>;
#[doc = "Field `SQI` reader - Read value reflects the state of the interrupt flag. Do not use the interrupt flag and associated mask registers if the EIP already provides similar controls for its interrupts. This feature is design for interrupts created in the SIB or EIP which do not have mask or flag bits. Write to 1 to clear the flag."]
pub type SqiR = crate::BitReader;
#[doc = "Field `SQI` writer - Read value reflects the state of the interrupt flag. Do not use the interrupt flag and associated mask registers if the EIP already provides similar controls for its interrupts. This feature is design for interrupts created in the SIB or EIP which do not have mask or flag bits. Write to 1 to clear the flag."]
pub type SqiW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - Read value reflects the state of the interrupt flag. Do not use the interrupt flag and associated mask registers if the EIP already provides similar controls for its interrupts. This feature is design for interrupts created in the SIB or EIP which do not have mask or flag bits. Write to 1 to clear the flag."]
    #[inline(always)]
    pub fn sqi(&self) -> SqiR {
        SqiR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Read value reflects the state of the interrupt flag. Do not use the interrupt flag and associated mask registers if the EIP already provides similar controls for its interrupts. This feature is design for interrupts created in the SIB or EIP which do not have mask or flag bits. Write to 1 to clear the flag."]
    #[inline(always)]
    pub fn sqi(&mut self) -> SqiW<'_, IntflagSpec> {
        SqiW::new(self, 0)
    }
}
#[doc = "Interrupt Status and Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intflag::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intflag::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntflagSpec;
impl crate::RegisterSpec for IntflagSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intflag::R`](R) reader structure"]
impl crate::Readable for IntflagSpec {}
#[doc = "`write(|w| ..)` method takes [`intflag::W`](W) writer structure"]
impl crate::Writable for IntflagSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets INTFLAG to value 0"]
impl crate::Resettable for IntflagSpec {}
