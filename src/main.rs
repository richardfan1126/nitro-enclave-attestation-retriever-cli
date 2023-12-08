use clap::{Parser, Subcommand};
use serde_bytes::ByteBuf;
use aws_nitro_enclaves_nsm_api::api::{Request, Response};
use aws_nitro_enclaves_nsm_api::driver::{nsm_init as native_nsm_init, nsm_exit, nsm_process_request};

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate attestation document
    Attest {
        /// (Optional) DER format public key the attestation consumer can use to encrypt data with
        #[clap(short, long, required=false)]
        public_key: Option<String>,

        /// (Optional) Base64-encoded DER format public key the attestation consumer can use to encrypt data with
        #[clap(long, required=false, conflicts_with="public-key")]
        public_key_b64: Option<String>,
    
        /// (Optional) Additional signed user data
        #[clap(short, long, required=false)]
        user_data: Option<String>,
    
        /// (Optional) Base64-encoded additional signed user data
        #[clap(long, required=false, conflicts_with="user-data")]
        user_data_b64: Option<String>,
        
        /// (Optional) Cryptographic nonce provided by the attestation consumer as a proof of authenticity
        #[clap(short, long, required=false)]
        nonce: Option<String>,
        
        /// (Optional) Base64-encoded cryptographic nonce provided by the attestation consumer as a proof of authenticity
        #[clap(long, required=false, conflicts_with="nonce")]
        nonce_b64: Option<String>,
    },

    /// Generate random bytes from NSM
    GetRandom {
        /// Byte length of the random data (Maximum 256 bytes)
        #[clap(short, long, required=true)]
        length: u16,
    },

    /// Get PCR value
    DescribePCR {
        /// The index of PCR
        #[clap(short, long, required=true)]
        index: u16,
    }
}

fn nsm_init() -> i32 {
    let nsm_fd = native_nsm_init();

    if nsm_fd == -1 {
        eprintln!("nsm-cli must be run inside Nitro Enclave");
        std::process::exit(1)
    }

    return nsm_fd;
}

fn error_exit(msg: &str, code: i32, nsm_fd: i32) {
    eprintln!("{}", msg);
    nsm_exit(nsm_fd);

    std::process::exit(code);
}

fn attest(public_key: Option<ByteBuf>, user_data: Option<ByteBuf>, nonce: Option<ByteBuf>) {
    let nsm_fd = nsm_init();

    let request = Request::Attestation {
        public_key,
        user_data,
        nonce,
    };

    let response = nsm_process_request(nsm_fd, request);
    
    match response {
        Response::Attestation{document} => {
            print!("{}", base64::encode(document));
        },
        Response::Error(err) => {
            error_exit(format!("{:?}", err).as_str(), 1, nsm_fd);
        },
        _ => {
            error_exit("Something went wrong", 1, nsm_fd);
        }
    }

    nsm_exit(nsm_fd);
}

unsafe fn get_random(byte_length:&u16) {
    if byte_length < &0 {
        return;
    }

    let buf_len: &mut usize = &mut 0;

    let mut buf = vec![0; *byte_length as usize];
    let buf_ptr = buf.as_mut_ptr();
    *buf_len = buf.len();
    
    let nsm_fd = nsm_init();
    let request = Request::GetRandom {};
    let response = nsm_process_request(nsm_fd, request);

    match response {
        Response::GetRandom { random } => {
            *buf_len = std::cmp::min(*buf_len, random.len());
            std::ptr::copy_nonoverlapping(random.as_ptr(), buf_ptr, *buf_len);
            print!("{}", base64::encode(buf));
        },
        Response::Error(err) => {
            error_exit(format!("{:?}", err).as_str(), 1, nsm_fd);
        },
        _ => {
            error_exit("Something went wrong", 1, nsm_fd);
        }
    }

    nsm_exit(nsm_fd);
}

fn get_byte_buf_from_input(plain_text:&Option<String>, base64:&Option<String>) -> Option<ByteBuf> {
    let mut result:Option<ByteBuf> = None;
    if !base64.is_none() {
        let base64_string = base64.as_ref().unwrap();
        let result_bytes = base64::decode(base64_string).unwrap();
        result = Some(ByteBuf::from(result_bytes));
    } else if !plain_text.is_none() {
        result = Some(ByteBuf::from(plain_text.as_ref().unwrap().as_bytes()));
    }

    return result;
}

fn describe_pcr(index:&u16) {
    let nsm_fd = nsm_init();
    let index = index.to_owned();
    let request = Request::DescribePCR { index };
    let response = nsm_process_request(nsm_fd, request);

    match response {
        Response::DescribePCR { lock:_, data } => {
            print!("{}", hex::encode(data));
        },
        Response::Error(err) => {
            error_exit(format!("{:?}", err).as_str(), 1, nsm_fd);
        },
        _ => {
            error_exit("Something went wrong", 1, nsm_fd);
        }
    }

    nsm_exit(nsm_fd);
}

fn main() {
    let args = Cli::parse();

    match &args.subcmd {
        Commands::Attest {public_key, public_key_b64, user_data, user_data_b64, nonce, nonce_b64} => {
            let public_key_byte_buf = get_byte_buf_from_input(public_key, public_key_b64);
            let user_data_byte_buf = get_byte_buf_from_input(user_data, user_data_b64);
            let nonce_byte_buf = get_byte_buf_from_input(nonce, nonce_b64);

            attest(public_key_byte_buf, user_data_byte_buf, nonce_byte_buf);
        },

        Commands::GetRandom {length} => {
            if length > &256 {
                eprintln!("Length should be within 256 bytes");
                std::process::exit(1)
            }

            unsafe {
                get_random(length);
            }
        },

        Commands::DescribePCR {index} => {
            if index > &31 {
                eprintln!("Index should not be greater than 31");
                std::process::exit(1)
            }

            describe_pcr(index);
        }
    }
}
