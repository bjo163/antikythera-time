use mtime_integrity::{
    ingest_source_artifact, SourceArtifactSpec, SourceSignaturePolicy, TrustedKeyRegistry,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 7 || args.len() > 8 {
        eprintln!(
            "usage: ingest_source <file> <source_id> <institution_id> <canonical_url> <media_type> <retrieved_at_unix_seconds> [expected_sha256]"
        );
        std::process::exit(2);
    }

    let payload = std::fs::read(&args[1]).expect("read source file");
    let retrieved_at_unix_seconds: i64 = args[6].parse().expect("retrieval unix seconds");
    let spec = SourceArtifactSpec {
        source_id: args[2].clone(),
        institution_id: args[3].clone(),
        canonical_url: args[4].clone(),
        media_type: args[5].clone(),
        expected_sha256_hex: args.get(7).cloned(),
        signature_policy: SourceSignaturePolicy::AllowUnsigned,
    };
    let artifact = ingest_source_artifact(
        &spec,
        &payload,
        retrieved_at_unix_seconds,
        None,
        &TrustedKeyRegistry::new(),
    )
    .expect("ingest source");

    println!("source_id={}", artifact.source_id);
    println!("institution_id={}", artifact.institution_id);
    println!("canonical_url={}", artifact.canonical_url);
    println!("media_type={}", artifact.media_type);
    println!("sha256={}", artifact.sha256_hex);
    println!("retrieved_at_unix_seconds={}", artifact.retrieved_at_unix_seconds);
    println!("signature_verified={}", artifact.signature_verified());
}
