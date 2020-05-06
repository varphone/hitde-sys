#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::unreadable_literal)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// Make HI_BOOL can convert to bool.
impl std::convert::Into<bool> for HI_BOOL {
    fn into(self) -> bool {
        self == HI_BOOL::HI_TRUE
    }
}

/// Make bool can convert to HI_BOOL.
impl std::convert::Into<HI_BOOL> for bool {
    fn into(self) -> HI_BOOL {
        if self {
            HI_BOOL::HI_TRUE
        } else {
            HI_BOOL::HI_FALSE
        }
    }
}

// Fix incomplete PartialEq trait for TDE_COMPOSOR_S
impl PartialEq for TDE_COMPOSOR_S {
    fn eq(&self, other: &Self) -> bool {
        self.stSrcSurface == other.stSrcSurface
            && self.stInRect == other.stInRect
            && self.stOutRect == other.stOutRect
            && self.stOpt == other.stOpt
    }
}

// Fix incomplete Eq trait for TDE_COMPOSOR_S
impl Eq for TDE_COMPOSOR_S {}

// Fix incomplete PartialEq trait for TDE2_COLORKEY_U
impl PartialEq for TDE2_COLORKEY_U {
    fn eq(&self, other: &Self) -> bool {
        unsafe { self._bindgen_union_align == other._bindgen_union_align }
    }
}

// Fix incomplete Eq trait for TDE2_COLORKEY_U
impl Eq for TDE2_COLORKEY_U {}

// Fix incomplete PartialEq trait for TDE2_OPT_S
impl PartialEq for TDE2_OPT_S {
    fn eq(&self, other: &Self) -> bool {
        self.enAluCmd == other.enAluCmd
            && self.enRopCode_Color == other.enRopCode_Color
            && self.enRopCode_Alpha == other.enRopCode_Alpha
            && self.enColorKeyMode == other.enColorKeyMode
            && self.unColorKeyValue == other.unColorKeyValue
            && self.enClipMode == other.enClipMode
            && self.stClipRect == other.stClipRect
            && self.enDeflickerMode == other.enDeflickerMode
            && self.enFilterMode == other.enFilterMode
            && self.enMirror == other.enMirror
            && self.bClutReload == other.bClutReload
            && self.u8GlobalAlpha == other.u8GlobalAlpha
            && self.enOutAlphaFrom == other.enOutAlphaFrom
            && self.u32Colorize == other.u32Colorize
            && self.stBlendOpt == other.stBlendOpt
            && self.stCscOpt == other.stCscOpt
            && self.bCompress == other.bCompress
            && self.bDecompress == other.bDecompress
    }
}

// Fix incomplete Eq trait for TDE2_OPT_S
impl Eq for TDE2_OPT_S {}

// Fix incomplete PartialEq trait for TDE2_PATTERN_FILL_OPT_S
impl PartialEq for TDE2_PATTERN_FILL_OPT_S {
    fn eq(&self, other: &Self) -> bool {
        self.enAluCmd == other.enAluCmd
            && self.enRopCode_Color == other.enRopCode_Color
            && self.enRopCode_Alpha == other.enRopCode_Alpha
            && self.enColorKeyMode == other.enColorKeyMode
            && self.unColorKeyValue == other.unColorKeyValue
            && self.enClipMode == other.enClipMode
            && self.stClipRect == other.stClipRect
            && self.bClutReload == other.bClutReload
            && self.u8GlobalAlpha == other.u8GlobalAlpha
            && self.enOutAlphaFrom == other.enOutAlphaFrom
            && self.u32Colorize == other.u32Colorize
            && self.stBlendOpt == other.stBlendOpt
            && self.stCscOpt == other.stCscOpt
    }
}

// Fix incomplete Eq trait for TDE2_PATTERN_FILL_OPT_S
impl Eq for TDE2_PATTERN_FILL_OPT_S {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_open_close() {
        unsafe {
            assert_eq!(HI_TDE2_Open(), HI_SUCCESS as HI_S32);
            HI_TDE2_Close();
        }
    }
}
