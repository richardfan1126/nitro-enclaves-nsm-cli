use clap::Parser;
use serde_bytes::ByteBuf;
use nsm_io::{Request, Response};

#[derive(Parser)]
struct Cli {
    #[clap(short='p', long="public-key-b64", help="(Optional) Base64-encoded DER format public key the attestation consumer can use to encrypt data with", required=false, takes_value=true)]
    public_key_b64: Option<String>,

    #[clap(short='u', long="user-data-b64", help="(Optional) Base64-encoded additional signed user data", required=false, takes_value=true)]
    user_data_b64: Option<String>,

    #[clap(short='n', long="nonce-b64", help="(Optional) Base64-encoded cryptographic nonce provided by the attestation consumer as a proof of authenticity", required=false, takes_value=true)]
    nonce_b64: Option<String>,
}

fn main() {
    let args = Cli::parse();

    let nsm_fd = nsm_driver::nsm_init();

    let mut public_key:Option<ByteBuf> = None;
    if !args.public_key_b64.is_none() {
        let public_key_b64_string = args.public_key_b64.unwrap();
        let public_key_bytes = base64::decode(public_key_b64_string).unwrap();
        public_key = Some(ByteBuf::from(public_key_bytes));
    }

    let mut user_data:Option<ByteBuf> = None;
    if !args.user_data_b64.is_none() {
        let user_data_b64_string = args.user_data_b64.unwrap();
        let user_data_bytes = base64::decode(user_data_b64_string).unwrap();
        user_data = Some(ByteBuf::from(user_data_bytes));
    }

    let mut nonce:Option<ByteBuf> = None;
    if !args.nonce_b64.is_none() {
        let nonce_b64_string = args.nonce_b64.unwrap();
        let nonce_bytes = base64::decode(nonce_b64_string).unwrap();
        nonce = Some(ByteBuf::from(nonce_bytes));
    }

    let request = Request::Attestation {
        public_key: public_key,
        user_data: user_data,
        nonce: nonce,
    };

    let response = nsm_driver::nsm_process_request(nsm_fd, request);
    
    let attestation_document = match response {
        Response::Attestation{document} => document,
        _ => unreachable!()
    };
    
    print!("{}", base64::encode(attestation_document));
    
    nsm_driver::nsm_exit(nsm_fd);
}