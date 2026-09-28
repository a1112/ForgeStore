use forge_store_core::service_api::{parse_request, StoreRequest};

#[test]
fn store_ipc_rejects_unknown_fields_and_oversize() {
    let valid = b"{\"schemaVersion\":1,\"requestId\":\"ui-1\",\"operation\":\"enqueue\",\"payload\":{\"appId\":\"7zip\",\"action\":\"install\"}}\n";
    let parsed = parse_request(valid).unwrap();
    assert!(matches!(parsed, StoreRequest::Enqueue { .. }));
    assert!(parse_request(b"{\"schemaVersion\":1,\"requestId\":\"ui-1\",\"operation\":\"snapshot\",\"payload\":{},\"extra\":true}\n").is_err());
    assert!(parse_request(&vec![b'x'; 4097]).is_err());
    assert!(parse_request(b"{\"schemaVersion\":1,\"requestId\":\"ui-1\",\"operation\":\"enqueue\",\"payload\":{\"appId\":\"evil;rm\",\"action\":\"install\"}}\n").is_err());
    assert!(parse_request(
        b"{\"schemaVersion\":1,\"requestId\":\"ui-1\",\"operation\":\"snapshot\",\"payload\":{}}\n"
    )
    .is_ok());
}
