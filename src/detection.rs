//! Interfaces to the detector itself.

use std::c_str::CString;
use std::default::Default;
use std::ptr::{null, null_mut};
use sync::mutex::{StaticMutex, MUTEX_INIT};

use libc::{c_int, c_double};
use ffi::{CLDHints, Encoding, CLD2_ExtDetectLanguageSummary4,
          CLD2_DetectLanguageVersion};
use ffi::Language as LanguageId;

use language::LanguageIdExt;
use types::*;

// A lock which controls our access to DetectLanguageVersion, which uses
// an internal buffer to format the string.
//
// TODO: Should we move this to the cld2-sys package, in case other
// packages want to synchronize with us?
static CLD2_VERSION_LOCK: StaticMutex = MUTEX_INIT;

/// Get the version of cld2 and its embedded data files as a string.
///
/// ```
/// use cld2::detector_version;
/// format!("cld2 version: {}", detector_version());
/// ```
pub fn detector_version() -> String {
    let _ = CLD2_VERSION_LOCK.lock();
    unsafe {
        let version_string = CLD2_DetectLanguageVersion();
        assert!(version_string.is_not_null());
        CString::new(version_string, false).as_str()
            .expect("CLD2::DectectLanguageVersion returned invalid string")
            .to_string()
    }
}

/// Detect the language of the input text.
///
/// ```
/// use cld2::{detect_language, Format, Reliable, Unreliable, Lang};
///
/// let text = "It is an ancient Mariner,
/// And he stoppeth one of three.
/// 'By thy long grey beard and glittering eye,
/// Now wherefore stopp'st thou me?";
///
/// assert_eq!((Some(Lang("en")), Reliable),
///            detect_language(text, Format::Text));
///
/// assert_eq!((None, Unreliable),
///            detect_language("blah", Format::Html));
/// ```
pub fn detect_language(text: &str, format: Format) ->
    (Option<Lang>, Reliability)
{
    let result = detect_language_ext(text, format, &Default::default());
    (result.language, result.reliability)
}

/// Detect the language of the input text, using optional hints, and return
/// detailed statistics.
///
/// ```
/// use std::default::Default;
/// use cld2::{detect_language_ext, Format, Lang};
///
/// let text = "Sur le pont d'Avignon,
/// L'on y danse, l'on y danse,
/// Sur le pont d'Avignon
/// L'on y danse tous en rond.
///
/// Les belles dames font comme ça
/// Et puis encore comme ça.
/// Les messieurs font comme ça
/// Et puis encore comme ça.";
///
/// let detected =
///   detect_language_ext(text, Format::Text, &Default::default());
/// 
/// assert_eq!(Some(Lang("fr")), detected.language);
/// ```
pub fn detect_language_ext(text: &str, format: Format, hints: &Hints)
    -> DetectionResult
{
    let mut language3 = [LanguageId::UNKNOWN_LANGUAGE,
                         LanguageId::UNKNOWN_LANGUAGE,
                         LanguageId::UNKNOWN_LANGUAGE];
    let mut percent3: [c_int, ..3] = [0, 0, 0];
    let mut normalized_score3: [c_double, ..3] = [0.0, 0.0, 0.0];
    let mut text_bytes: c_int = 0;
    let mut is_reliable: bool = false;

    unsafe {
        hints.with_c_rep(|hints_ptr| {
            let lang = CLD2_ExtDetectLanguageSummary4(
                text.as_ptr() as *const i8, text.len() as c_int,
                format == Format::Text, hints_ptr, 0,
                language3.as_mut_slice().as_mut_ptr(),
                percent3.as_mut_slice().as_mut_ptr(),
                normalized_score3.as_mut_slice().as_mut_ptr(),
                null_mut(), &mut text_bytes, &mut is_reliable);
            from_ffi(lang, &language3, &percent3, &normalized_score3,
                     text_bytes, is_reliable)
        })
    }
}

/// A value which can be converted to type `R` for use with the FFI.
trait WithCRep<R> {
    /// Call the function `body` with a C-compatible represention of type
    /// `R`.
    fn with_c_rep<T>(&self, body: |R| -> T) -> T;
}

impl<'a> WithCRep<*const CLDHints> for Hints<'a> {
    fn with_c_rep<T>(&self, body: |*const CLDHints| -> T) -> T {
        let clang = self.content_language.map(|v| v.to_c_str());
        let clang_ptr = clang.map(|v| v.as_ptr()).unwrap_or(null());
        let tld = self.tld.map(|v| v.to_c_str());
        let tld_ptr = tld.map(|v| v.as_ptr()).unwrap_or(null());
        let lang = self.language
            .map(|Lang(c)| LanguageIdExt::from_name(c))
            .unwrap_or(LanguageId::UNKNOWN_LANGUAGE);
        let encoding = self.encoding
            .unwrap_or(Encoding::UNKNOWN_ENCODING) as c_int;
        let hints =
            CLDHints{content_language_hint: clang_ptr, tld_hint: tld_ptr,
                     encoding_hint: encoding, language_hint: lang};
        body(&hints)
    }
}

fn from_ffi(lang: LanguageId, language3: &[LanguageId, ..3],
            percent3: &[c_int, ..3], normalized_score3: &[c_double, ..3],
            text_bytes: c_int, reliable: bool) -> DetectionResult
{
    let score_n = |n| {
        LanguageScore{language: language3[n].to_lang(),
                      percent: percent3[n] as u8,
                      normalized_score: normalized_score3[n]}
    };

    DetectionResult::new(lang.to_lang(),
                         [score_n(0), score_n(1), score_n(2)],
                         text_bytes,
                         Reliability::from_bool(reliable))
}
