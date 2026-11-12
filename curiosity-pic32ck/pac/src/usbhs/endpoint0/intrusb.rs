#[doc = "Register `INTRUSB` reader"]
pub type R = crate::R<IntrusbSpec>;
#[doc = "Field `SUSPEND` reader - Set When Suspend Signal Is Detected On The Bus"]
pub type SuspendR = crate::BitReader;
#[doc = "Field `RESUME` reader - Set When Resume Signal Is Detected On The Bus"]
pub type ResumeR = crate::BitReader;
#[doc = "Field `RESET` reader - Set In Peripheral Mode when Reset Is Detected On The Bus. In HostMode When Babble Is Detected"]
pub type ResetR = crate::BitReader;
#[doc = "Field `SOF` reader - Set When A New Frame Starts"]
pub type SofR = crate::BitReader;
#[doc = "Field `CONN` reader - Set When A Device Connection Is Detected"]
pub type ConnR = crate::BitReader;
#[doc = "Field `DISCON` reader - Set In Host Mode When A Device Disconnect Is Detected.Set In Peripheral Mode When A Session Ends"]
pub type DisconR = crate::BitReader;
#[doc = "Field `SESSREQ` reader - Set When Session Request Signaling Has Been Detected."]
pub type SessreqR = crate::BitReader;
#[doc = "Field `VBUSERR` reader - Set When VBus Drops Below The VBus Valid Threshold During A Session."]
pub type VbuserrR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set When Suspend Signal Is Detected On The Bus"]
    #[inline(always)]
    pub fn suspend(&self) -> SuspendR {
        SuspendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set When Resume Signal Is Detected On The Bus"]
    #[inline(always)]
    pub fn resume(&self) -> ResumeR {
        ResumeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set In Peripheral Mode when Reset Is Detected On The Bus. In HostMode When Babble Is Detected"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set When A New Frame Starts"]
    #[inline(always)]
    pub fn sof(&self) -> SofR {
        SofR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set When A Device Connection Is Detected"]
    #[inline(always)]
    pub fn conn(&self) -> ConnR {
        ConnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set In Host Mode When A Device Disconnect Is Detected.Set In Peripheral Mode When A Session Ends"]
    #[inline(always)]
    pub fn discon(&self) -> DisconR {
        DisconR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set When Session Request Signaling Has Been Detected."]
    #[inline(always)]
    pub fn sessreq(&self) -> SessreqR {
        SessreqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set When VBus Drops Below The VBus Valid Threshold During A Session."]
    #[inline(always)]
    pub fn vbuserr(&self) -> VbuserrR {
        VbuserrR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Read Only Register Which Indicates Which USB Interrupts Are Currently Active.\n\nYou can [`read`](crate::Reg::read) this register and get [`intrusb::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrusbSpec;
impl crate::RegisterSpec for IntrusbSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`intrusb::R`](R) reader structure"]
impl crate::Readable for IntrusbSpec {}
#[doc = "`reset()` method sets INTRUSB to value 0"]
impl crate::Resettable for IntrusbSpec {}
