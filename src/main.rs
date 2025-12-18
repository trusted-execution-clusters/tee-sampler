use attester::{detect_attestable_devices, detect_tee_type, BoxedAttester};
use verifier::az_snp_vtpm::{parse_tee_evidence_az, extend_claim};
use az_snp_vtpm::vtpm;
use kbs_types::Tee;
use az_snp_vtpm::hcl;
use serde_json::Value;
use verifier::snp::{SnpEvidence, parse_tee_evidence};
use log::warn;
#[cfg(feature = "az-tdx-vtpm")]
use verifier::intel_dcap::{ecdsa_quote_verification, extend_using_custom_claims};
#[cfg(feature = "az-tdx-vtpm")]
use verifier::tdx::claims::generate_parsed_claim;
#[cfg(feature = "az-tdx-vtpm")]
use verifier::tdx::quote::parse_tdx_quote;
#[cfg(feature = "az-tdx-vtpm")]
use az_tdx_vtpm::vtpm::Quote as TpmQuote;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct EvidenceAzSnpVtpm {
    quote: vtpm::Quote,
    report: Vec<u8>,
    vcek: String,
}
#[cfg(feature = "az-tdx-vtpm")]
#[derive(Serialize, Deserialize)]
struct EvidenceAzTdxVtpm {
    tpm_quote: TpmQuote,
    hcl_report: Vec<u8>,
    td_quote: Vec<u8>,
}

async fn print_claim(evidence_value: Value, tee_type: Tee) {
    let mut claim = evidence_value.clone();
    match tee_type {
        Tee::AzSnpVtpm => {
            let evidence: EvidenceAzSnpVtpm = serde_json::from_value(evidence_value).unwrap();
            let hcl_report = hcl::HclReport::new(evidence.report).unwrap();
            let snp_report = hcl_report.try_into();
            claim = parse_tee_evidence_az(&snp_report.unwrap());
            let _ = extend_claim(&mut claim, &evidence.quote);
        }

        #[cfg(feature = "az-tdx-vtpm")]
        Tee::AzTdxVtpm => { 
            let evidence = serde_json::from_value::<EvidenceAzTdxVtpm>(evidence_value).unwrap();
            // let hcl_report = hcl::HclReport::new(evidence.hcl_report).unwrap();
            let td_quote = parse_tdx_quote(&evidence.td_quote).unwrap();
            claim = generate_parsed_claim(td_quote, None).unwrap();
            let custom_claims = ecdsa_quote_verification(&evidence.td_quote).await.unwrap();
            let _ = extend_claim(&mut claim, &evidence.tpm_quote);
            let _ = extend_using_custom_claims(&mut claim, custom_claims);
        }
        Tee::Snp => {
            let evidence = serde_json::from_value::<SnpEvidence>(evidence_value).unwrap();
            claim = parse_tee_evidence(&evidence.attestation_report);
        }
        _ => warn!("Unsupported tee type: {:?}", tee_type)
    }
    println!("{:?}:\n{}", tee_type, serde_json::to_string_pretty(&claim).expect("Failed to serialize claim"));
}

#[tokio::main]
async fn main() {
    env_logger::init();
    // report_data on all platforms is 32 bytes length.
    let report_data = vec![0u8; 32];
    let tee_type = detect_tee_type();
    let evidence_value = TryInto::<BoxedAttester>::try_into(tee_type.clone())
        .expect("Failed to initialize attester.")
        .get_evidence(report_data.clone())
        .await
        .expect("get evidence failed");
    
    print_claim(evidence_value, tee_type).await;
    for tee in detect_attestable_devices() {
        let attester =
            TryInto::<BoxedAttester>::try_into(tee).expect("Failed to initialize device attester");

        let evidence_value = attester
            .get_evidence(report_data.clone())
            .await
            .expect("get additional evidence failed");

        print_claim(evidence_value, tee).await;
    }
}