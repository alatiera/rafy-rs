use hyper;
use json;
use native_tls;

use std::io;
use std::num;

error_chain!{
    errors{
        // This is probably not needed with error-chain.
        // Should probably raise hyperErrors but I am not familiar
        // with the codebase.
        VideoNotFound(s: String){
            description("A network request has failed."),
            display("A network request has failed.: '{}'", s),
        }
    }

    foreign_links  {
        HyperError(hyper::Error);
        IOError(io::Error);
        JsonError(json::Error);
        StdNumParseError(num::ParseIntError);
        NativeTlsError(native_tls::Error);
    }
}
