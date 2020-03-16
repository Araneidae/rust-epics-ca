// Build script for linking to ca

const LIB_PATH: &str = "/dls_sw/epics/R3.14.12.7/base/lib/linux-x86_64/";

fn main()
{
    println!("cargo:rustc-link-search={:}", LIB_PATH);
}
