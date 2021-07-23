use ic_cdk::export::candid::{candid_method,export_service, IDLProg};

#[ic_cdk_macros::query]
#[candid_method(query)]
fn test(test_param: String) -> String {
    format!("test_param: {0} ", test_param)
}

#[ic_cdk_macros::query(name = "supportedInterface")]
#[candid_method(query,rename="supportedInterface")]
fn supported_inteface(interface: String) -> bool {
    let verify_service_desc = format!("service:{{ {0};}}", interface);
    let verify_ast_result = verify_service_desc.parse::<IDLProg>();

    match verify_ast_result {
        Ok(verify_ast) => {
            let verify_pretty: String = candid::parser::types::to_pretty(&verify_ast, 80);
            let verify_pretty_sub: String =
                verify_pretty.replace("service : { ", "").replace(" }", "");

            let origin_did = get_did();
            let origin_ast: IDLProg = origin_did.parse().unwrap();
            let origin_pretty: String = candid::parser::types::to_pretty(&origin_ast, 80);
            origin_pretty.contains(&verify_pretty_sub)
        }
        _ => false,
    }
}

export_service!();
fn get_did() -> String {
    __export_service()
}
