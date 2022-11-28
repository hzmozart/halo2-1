fn main() {
    #[cfg(feature = "cuda")]
    {
        use ec_gpu_gen::SourceBuilder;
        use pairing::bn256::{Fq, Fr, G1Affine};
        let source_builder = SourceBuilder::new()
            .add_fft::<Fr>()
            .add_multiexp::<G1Affine, Fq>();
        ec_gpu_gen::generate(&source_builder);
    }

    let mut nvcc = cc::Build::new();

    nvcc.cuda(true)
        .flag("-cudart=shared")
        .flag("-gencode")
        .flag("arch=compute_61,code=sm_61");
    nvcc.file("cuda/hello.cu").compile("libhello.a");

    println!("cargo:rustc-link-search=native=/usr/local/cuda/lib64");
    println!("cargo:rustc-link-lib=cudart");
}
