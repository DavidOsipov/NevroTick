//! Protected review ledger. Catalogs, signatures and fingerprints cannot write it.
//! Deliberately empty: this source staging does not manufacture human approval.
use super::{Catalog, FINGERPRINT};
/// Independent protected semantic grant, not deserializable catalog metadata.
struct Review {
    /// Protected compatibility revision to which this grant applies.
    contract: &'static str,
    /// Untrusted revision checked during construction and bound by approval.
    revision: u32,
    /// Exact independently approved canonical content bytes.
    canonical_english: &'static [u8],
}
/// Protected whole-English review grants; empty until independently approved.
const REVIEWS: &[Review] = &[];
/// Authority is fixed by the receiving code path, never catalog metadata.
#[derive(Clone, Copy)]
pub(super) struct Authority {
    /// True only through the test-only constructor constant.
    synthetic_fixture: bool,
}
impl Authority {
    /// Production paths always use the protected release ledger.
    pub(super) const RELEASE: Self = Self {
        synthetic_fixture: false,
    };
    /// Mechanism-test authority; the constant is absent from production.
    #[cfg(test)]
    pub(super) const SYNTHETIC_FIXTURE: Self = Self {
        synthetic_fixture: true,
    };
}
/// Independent fixed bytes for mechanism tests, not a production semantic grant.
#[cfg(test)]
const SYNTHETIC: &[u8] = include_bytes!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/fixtures/test-only-english.cbor"
));
/// Match protected exact canonical English bytes and contract/revision; no catalog flag applies.
pub(super) fn approved(catalog: &Catalog, canonical: &[u8], authority: Authority) -> bool {
    if authority.synthetic_fixture {
        #[cfg(test)]
        {
            return catalog.revision == 2 && canonical == SYNTHETIC;
        }
        #[cfg(not(test))]
        {
            return false;
        }
    }
    REVIEWS.iter().any(|review| {
        review.contract == FINGERPRINT
            && review.revision == catalog.revision
            && review.canonical_english == canonical
    })
}
// Integration owner adds exact canonical bytes only after independent whole-English
// meaning/action/a11y review. Do not point this ledger at the catalog's own path,
// populate it from a data-only PR, or add a runtime "verified" override.
