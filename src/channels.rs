extern crate libsoundio_sys as raw;

use super::util::*;

use std::fmt;

/// ChannelId indicates the location or intent of a channel (left, right, LFE, etc.).
///
/// It supports the `Display` trait so that you can convert `ChannelId::FrontLeft` to `"Front Left"` for example.
///
/// # Examples
///
/// ```
/// # use soundio::*;
/// println!("Layout: {}", ChannelId::FrontLeftCenter);
///
/// assert_eq!(format!("{}", ChannelId::MsMid), "Mid/Side Mid");
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ChannelId {
	Invalid,

	/// The more commonly supported ids.
	FrontLeft, 
	FrontRight,
	FrontCenter,
	Lfe,
	BackLeft,
	BackRight,
	FrontLeftCenter,
	FrontRightCenter,
	BackCenter,
	SideLeft,
	SideRight,
	TopCenter,
	TopFrontLeft,
	TopFrontCenter,
	TopFrontRight,
	TopBackLeft,
	TopBackCenter,
	TopBackRight,

	/// The less commonly supported ids.
	BackLeftCenter, 
	BackRightCenter,
	FrontLeftWide,
	FrontRightWide,
	FrontLeftHigh,
	FrontCenterHigh,
	FrontRightHigh,
	TopFrontLeftCenter,
	TopFrontRightCenter,
	TopSideLeft,
	TopSideRight,
	LeftLfe,
	RightLfe,
	Lfe2,
	BottomCenter,
	BottomLeftCenter,
	BottomRightCenter,

	/// Mid/side recording
	MsMid,
	MsSide,

	/// First order ambisonic channels
	AmbisonicW,
	AmbisonicX,
	AmbisonicY,
	AmbisonicZ,

	/// X-Y Recording
	XyX,
	XyY,

	/// The "other" channel ids
	HeadphonesLeft, 
	HeadphonesRight,
	ClickTrack,
	ForeignLanguage,
	HearingImpaired,
	Narration,
	Haptic,
	DialogCentricMix,

	Aux,
	Aux0,
	Aux1,
	Aux2,
	Aux3,
	Aux4,
	Aux5,
	Aux6,
	Aux7,
	Aux8,
	Aux9,
	Aux10,
	Aux11,
	Aux12,
	Aux13,
	Aux14,
	Aux15,
}

impl From<raw::SoundIoChannelId> for ChannelId {
	fn from(channel_id: raw::SoundIoChannelId) -> ChannelId {
		match channel_id {
			raw::SoundIoChannelId::SoundIoChannelIdFrontLeft => ChannelId::FrontLeft,
			raw::SoundIoChannelId::SoundIoChannelIdFrontRight => ChannelId::FrontRight,
			raw::SoundIoChannelId::SoundIoChannelIdFrontCenter => ChannelId::FrontCenter,
			raw::SoundIoChannelId::SoundIoChannelIdLfe => ChannelId::Lfe,
			raw::SoundIoChannelId::SoundIoChannelIdBackLeft => ChannelId::BackLeft,
			raw::SoundIoChannelId::SoundIoChannelIdBackRight => ChannelId::BackRight,
			raw::SoundIoChannelId::SoundIoChannelIdFrontLeftCenter => ChannelId::FrontLeftCenter,
			raw::SoundIoChannelId::SoundIoChannelIdFrontRightCenter => ChannelId::FrontRightCenter,
			raw::SoundIoChannelId::SoundIoChannelIdBackCenter => ChannelId::BackCenter,
			raw::SoundIoChannelId::SoundIoChannelIdSideLeft => ChannelId::SideLeft,
			raw::SoundIoChannelId::SoundIoChannelIdSideRight => ChannelId::SideRight,
			raw::SoundIoChannelId::SoundIoChannelIdTopCenter => ChannelId::TopCenter,
			raw::SoundIoChannelId::SoundIoChannelIdTopFrontLeft => ChannelId::TopFrontLeft,
			raw::SoundIoChannelId::SoundIoChannelIdTopFrontCenter => ChannelId::TopFrontCenter,
			raw::SoundIoChannelId::SoundIoChannelIdTopFrontRight => ChannelId::TopFrontRight,
			raw::SoundIoChannelId::SoundIoChannelIdTopBackLeft => ChannelId::TopBackLeft,
			raw::SoundIoChannelId::SoundIoChannelIdTopBackCenter => ChannelId::TopBackCenter,
			raw::SoundIoChannelId::SoundIoChannelIdTopBackRight  => ChannelId::TopBackRight ,
			raw::SoundIoChannelId::SoundIoChannelIdBackLeftCenter => ChannelId::BackLeftCenter,
			raw::SoundIoChannelId::SoundIoChannelIdBackRightCenter => ChannelId::BackRightCenter,
			raw::SoundIoChannelId::SoundIoChannelIdFrontLeftWide => ChannelId::FrontLeftWide,
			raw::SoundIoChannelId::SoundIoChannelIdFrontRightWide => ChannelId::FrontRightWide,
			raw::SoundIoChannelId::SoundIoChannelIdFrontLeftHigh => ChannelId::FrontLeftHigh,
			raw::SoundIoChannelId::SoundIoChannelIdFrontCenterHigh => ChannelId::FrontCenterHigh,
			raw::SoundIoChannelId::SoundIoChannelIdFrontRightHigh => ChannelId::FrontRightHigh,
			raw::SoundIoChannelId::SoundIoChannelIdTopFrontLeftCenter => ChannelId::TopFrontLeftCenter,
			raw::SoundIoChannelId::SoundIoChannelIdTopFrontRightCenter => ChannelId::TopFrontRightCenter,
			raw::SoundIoChannelId::SoundIoChannelIdTopSideLeft => ChannelId::TopSideLeft,
			raw::SoundIoChannelId::SoundIoChannelIdTopSideRight => ChannelId::TopSideRight,
			raw::SoundIoChannelId::SoundIoChannelIdLeftLfe => ChannelId::LeftLfe,
			raw::SoundIoChannelId::SoundIoChannelIdRightLfe => ChannelId::RightLfe,
			raw::SoundIoChannelId::SoundIoChannelIdLfe2 => ChannelId::Lfe2,
			raw::SoundIoChannelId::SoundIoChannelIdBottomCenter => ChannelId::BottomCenter,
			raw::SoundIoChannelId::SoundIoChannelIdBottomLeftCenter => ChannelId::BottomLeftCenter,
			raw::SoundIoChannelId::SoundIoChannelIdBottomRightCenter => ChannelId::BottomRightCenter,
			raw::SoundIoChannelId::SoundIoChannelIdMsMid => ChannelId::MsMid,
			raw::SoundIoChannelId::SoundIoChannelIdMsSide => ChannelId::MsSide,
			raw::SoundIoChannelId::SoundIoChannelIdAmbisonicW => ChannelId::AmbisonicW,
			raw::SoundIoChannelId::SoundIoChannelIdAmbisonicX => ChannelId::AmbisonicX,
			raw::SoundIoChannelId::SoundIoChannelIdAmbisonicY => ChannelId::AmbisonicY,
			raw::SoundIoChannelId::SoundIoChannelIdAmbisonicZ => ChannelId::AmbisonicZ,
			raw::SoundIoChannelId::SoundIoChannelIdXyX => ChannelId::XyX,
			raw::SoundIoChannelId::SoundIoChannelIdXyY => ChannelId::XyY,
			raw::SoundIoChannelId::SoundIoChannelIdHeadphonesLeft => ChannelId::HeadphonesLeft,
			raw::SoundIoChannelId::SoundIoChannelIdHeadphonesRight => ChannelId::HeadphonesRight,
			raw::SoundIoChannelId::SoundIoChannelIdClickTrack => ChannelId::ClickTrack,
			raw::SoundIoChannelId::SoundIoChannelIdForeignLanguage => ChannelId::ForeignLanguage,
			raw::SoundIoChannelId::SoundIoChannelIdHearingImpaired => ChannelId::HearingImpaired,
			raw::SoundIoChannelId::SoundIoChannelIdNarration => ChannelId::Narration,
			raw::SoundIoChannelId::SoundIoChannelIdHaptic => ChannelId::Haptic,
			raw::SoundIoChannelId::SoundIoChannelIdDialogCentricMix  => ChannelId::DialogCentricMix ,
			raw::SoundIoChannelId::SoundIoChannelIdAux => ChannelId::Aux,
			raw::SoundIoChannelId::SoundIoChannelIdAux0 => ChannelId::Aux0,
			raw::SoundIoChannelId::SoundIoChannelIdAux1 => ChannelId::Aux1,
			raw::SoundIoChannelId::SoundIoChannelIdAux2 => ChannelId::Aux2,
			raw::SoundIoChannelId::SoundIoChannelIdAux3 => ChannelId::Aux3,
			raw::SoundIoChannelId::SoundIoChannelIdAux4 => ChannelId::Aux4,
			raw::SoundIoChannelId::SoundIoChannelIdAux5 => ChannelId::Aux5,
			raw::SoundIoChannelId::SoundIoChannelIdAux6 => ChannelId::Aux6,
			raw::SoundIoChannelId::SoundIoChannelIdAux7 => ChannelId::Aux7,
			raw::SoundIoChannelId::SoundIoChannelIdAux8 => ChannelId::Aux8,
			raw::SoundIoChannelId::SoundIoChannelIdAux9 => ChannelId::Aux9,
			raw::SoundIoChannelId::SoundIoChannelIdAux10 => ChannelId::Aux10,
			raw::SoundIoChannelId::SoundIoChannelIdAux11 => ChannelId::Aux11,
			raw::SoundIoChannelId::SoundIoChannelIdAux12 => ChannelId::Aux12,
			raw::SoundIoChannelId::SoundIoChannelIdAux13 => ChannelId::Aux13,
			raw::SoundIoChannelId::SoundIoChannelIdAux14 => ChannelId::Aux14,
			raw::SoundIoChannelId::SoundIoChannelIdAux15 => ChannelId::Aux15,
			_ => ChannelId::Invalid,
		}
	}
}

impl From<ChannelId> for raw::SoundIoChannelId {
	fn from(channel_id: ChannelId) -> raw::SoundIoChannelId {
		match channel_id {
			ChannelId::FrontLeft => raw::SoundIoChannelId::SoundIoChannelIdFrontLeft,
			ChannelId::FrontRight => raw::SoundIoChannelId::SoundIoChannelIdFrontRight,
			ChannelId::FrontCenter => raw::SoundIoChannelId::SoundIoChannelIdFrontCenter,
			ChannelId::Lfe => raw::SoundIoChannelId::SoundIoChannelIdLfe,
			ChannelId::BackLeft => raw::SoundIoChannelId::SoundIoChannelIdBackLeft,
			ChannelId::BackRight => raw::SoundIoChannelId::SoundIoChannelIdBackRight,
			ChannelId::FrontLeftCenter => raw::SoundIoChannelId::SoundIoChannelIdFrontLeftCenter,
			ChannelId::FrontRightCenter => raw::SoundIoChannelId::SoundIoChannelIdFrontRightCenter,
			ChannelId::BackCenter => raw::SoundIoChannelId::SoundIoChannelIdBackCenter,
			ChannelId::SideLeft => raw::SoundIoChannelId::SoundIoChannelIdSideLeft,
			ChannelId::SideRight => raw::SoundIoChannelId::SoundIoChannelIdSideRight,
			ChannelId::TopCenter => raw::SoundIoChannelId::SoundIoChannelIdTopCenter,
			ChannelId::TopFrontLeft => raw::SoundIoChannelId::SoundIoChannelIdTopFrontLeft,
			ChannelId::TopFrontCenter => raw::SoundIoChannelId::SoundIoChannelIdTopFrontCenter,
			ChannelId::TopFrontRight => raw::SoundIoChannelId::SoundIoChannelIdTopFrontRight,
			ChannelId::TopBackLeft => raw::SoundIoChannelId::SoundIoChannelIdTopBackLeft,
			ChannelId::TopBackCenter => raw::SoundIoChannelId::SoundIoChannelIdTopBackCenter,
			ChannelId::TopBackRight  => raw::SoundIoChannelId::SoundIoChannelIdTopBackRight ,
			ChannelId::BackLeftCenter => raw::SoundIoChannelId::SoundIoChannelIdBackLeftCenter,
			ChannelId::BackRightCenter => raw::SoundIoChannelId::SoundIoChannelIdBackRightCenter,
			ChannelId::FrontLeftWide => raw::SoundIoChannelId::SoundIoChannelIdFrontLeftWide,
			ChannelId::FrontRightWide => raw::SoundIoChannelId::SoundIoChannelIdFrontRightWide,
			ChannelId::FrontLeftHigh => raw::SoundIoChannelId::SoundIoChannelIdFrontLeftHigh,
			ChannelId::FrontCenterHigh => raw::SoundIoChannelId::SoundIoChannelIdFrontCenterHigh,
			ChannelId::FrontRightHigh => raw::SoundIoChannelId::SoundIoChannelIdFrontRightHigh,
			ChannelId::TopFrontLeftCenter => raw::SoundIoChannelId::SoundIoChannelIdTopFrontLeftCenter,
			ChannelId::TopFrontRightCenter => raw::SoundIoChannelId::SoundIoChannelIdTopFrontRightCenter,
			ChannelId::TopSideLeft => raw::SoundIoChannelId::SoundIoChannelIdTopSideLeft,
			ChannelId::TopSideRight => raw::SoundIoChannelId::SoundIoChannelIdTopSideRight,
			ChannelId::LeftLfe => raw::SoundIoChannelId::SoundIoChannelIdLeftLfe,
			ChannelId::RightLfe => raw::SoundIoChannelId::SoundIoChannelIdRightLfe,
			ChannelId::Lfe2 => raw::SoundIoChannelId::SoundIoChannelIdLfe2,
			ChannelId::BottomCenter => raw::SoundIoChannelId::SoundIoChannelIdBottomCenter,
			ChannelId::BottomLeftCenter => raw::SoundIoChannelId::SoundIoChannelIdBottomLeftCenter,
			ChannelId::BottomRightCenter => raw::SoundIoChannelId::SoundIoChannelIdBottomRightCenter,
			ChannelId::MsMid => raw::SoundIoChannelId::SoundIoChannelIdMsMid,
			ChannelId::MsSide => raw::SoundIoChannelId::SoundIoChannelIdMsSide,
			ChannelId::AmbisonicW => raw::SoundIoChannelId::SoundIoChannelIdAmbisonicW,
			ChannelId::AmbisonicX => raw::SoundIoChannelId::SoundIoChannelIdAmbisonicX,
			ChannelId::AmbisonicY => raw::SoundIoChannelId::SoundIoChannelIdAmbisonicY,
			ChannelId::AmbisonicZ => raw::SoundIoChannelId::SoundIoChannelIdAmbisonicZ,
			ChannelId::XyX => raw::SoundIoChannelId::SoundIoChannelIdXyX,
			ChannelId::XyY => raw::SoundIoChannelId::SoundIoChannelIdXyY,
			ChannelId::HeadphonesLeft => raw::SoundIoChannelId::SoundIoChannelIdHeadphonesLeft,
			ChannelId::HeadphonesRight => raw::SoundIoChannelId::SoundIoChannelIdHeadphonesRight,
			ChannelId::ClickTrack => raw::SoundIoChannelId::SoundIoChannelIdClickTrack,
			ChannelId::ForeignLanguage => raw::SoundIoChannelId::SoundIoChannelIdForeignLanguage,
			ChannelId::HearingImpaired => raw::SoundIoChannelId::SoundIoChannelIdHearingImpaired,
			ChannelId::Narration => raw::SoundIoChannelId::SoundIoChannelIdNarration,
			ChannelId::Haptic => raw::SoundIoChannelId::SoundIoChannelIdHaptic,
			ChannelId::DialogCentricMix  => raw::SoundIoChannelId::SoundIoChannelIdDialogCentricMix ,
			ChannelId::Aux => raw::SoundIoChannelId::SoundIoChannelIdAux,
			ChannelId::Aux0 => raw::SoundIoChannelId::SoundIoChannelIdAux0,
			ChannelId::Aux1 => raw::SoundIoChannelId::SoundIoChannelIdAux1,
			ChannelId::Aux2 => raw::SoundIoChannelId::SoundIoChannelIdAux2,
			ChannelId::Aux3 => raw::SoundIoChannelId::SoundIoChannelIdAux3,
			ChannelId::Aux4 => raw::SoundIoChannelId::SoundIoChannelIdAux4,
			ChannelId::Aux5 => raw::SoundIoChannelId::SoundIoChannelIdAux5,
			ChannelId::Aux6 => raw::SoundIoChannelId::SoundIoChannelIdAux6,
			ChannelId::Aux7 => raw::SoundIoChannelId::SoundIoChannelIdAux7,
			ChannelId::Aux8 => raw::SoundIoChannelId::SoundIoChannelIdAux8,
			ChannelId::Aux9 => raw::SoundIoChannelId::SoundIoChannelIdAux9,
			ChannelId::Aux10 => raw::SoundIoChannelId::SoundIoChannelIdAux10,
			ChannelId::Aux11 => raw::SoundIoChannelId::SoundIoChannelIdAux11,
			ChannelId::Aux12 => raw::SoundIoChannelId::SoundIoChannelIdAux12,
			ChannelId::Aux13 => raw::SoundIoChannelId::SoundIoChannelIdAux13,
			ChannelId::Aux14 => raw::SoundIoChannelId::SoundIoChannelIdAux14,
			ChannelId::Aux15 => raw::SoundIoChannelId::SoundIoChannelIdAux15,
			_ => raw::SoundIoChannelId::SoundIoChannelIdInvalid,
		}
	}
}

impl fmt::Display for ChannelId {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// In the C source these only use ASCII characters so it is technically ambiguous
		// whether this is UTF-8 or Latin1.
		let s = latin1_to_string( unsafe { raw::soundio_get_channel_name((*self).into()) } );
		f.write_str(&s)
	}
}

impl ChannelId {
	/// Given UTF-8 encoded text which is the name of a channel such as
	/// "Front Left", "FL", or "front-left", return the corresponding
	/// `ChannelId`. Returns `None` for no match.
	///
	/// # Examples
	///
	/// ```
	/// # use soundio::*;
	/// assert_eq!(ChannelId::parse("Front Left Center"), Some(ChannelId::FrontLeftCenter));
	/// assert_eq!(ChannelId::parse("FLC"), Some(ChannelId::FrontLeftCenter));
	/// assert_eq!(ChannelId::parse("front-left-of-center"), Some(ChannelId::FrontLeftCenter));
	/// assert_eq!(ChannelId::parse("Shot is the best!"), None);
	/// ```
	pub fn parse(id: &str) -> Option<ChannelId> {
		match unsafe { raw::soundio_parse_channel_id(id.as_ptr() as _, id.len() as _) } {
			raw::SoundIoChannelId::SoundIoChannelIdInvalid => None,
			x => Some(x.into()),
		}
	}
}
