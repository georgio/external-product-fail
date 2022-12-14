// use concrete_core::backends::fft::private::crypto::ggsw::*;

use concrete_core::{
    backends::fft::private::{
        c64,
        crypto::ggsw::{external_product, external_product_scratch, FourierGgswCiphertext},
        math::fft::Fft,
    },
    commons::{
        crypto::glwe::GlweCiphertext,
        math::{
            tensor::{AsMutTensor, Tensor},
            torus::UnsignedTorus,
        },
    },
    prelude::{
        DecompositionBaseLog, DecompositionLevelCount, GlweDimension, PolynomialSize, Variance, *,
    },
};

use aligned_vec::avec;
use dyn_stack::{DynStack, GlobalMemBuffer, ReborrowMut};

fn external_product_bench<Scalar: UnsignedTorus>(n: usize, a: f64, b: Scalar) {
    let polynomial_size = PolynomialSize(n);
    let glwe_size = GlweSize(3);
    let decomposition_level_count = DecompositionLevelCount(4);
    let decomposition_base_log = DecompositionBaseLog(2);

    let mut out = GlweCiphertext::from_container(
        avec![Scalar::ZERO; polynomial_size.0 * glwe_size.0].into_boxed_slice(),
        polynomial_size,
    );
    let mut comp = c64::default();
    comp.re = a;
    comp.im = a;

    let ggsw = FourierGgswCiphertext::new(
        avec![
            comp;
            polynomial_size.0 / 2 * glwe_size.0 * glwe_size.0 * decomposition_level_count.0
        ]
        .into_boxed_slice(),
        polynomial_size,
        glwe_size,
        decomposition_base_log,
        decomposition_level_count,
    );
    let glwe = GlweCiphertext::from_container(
        avec![b; polynomial_size.0 * glwe_size.0].into_boxed_slice(),
        polynomial_size,
    );
    let fft = Fft::new(polynomial_size);
    let fft = fft.as_view();

    let mut mem = GlobalMemBuffer::new(
        external_product_scratch::<Scalar>(glwe_size, polynomial_size, fft).unwrap(),
    );
    let mut stack = DynStack::new(&mut mem);

    external_product(
        out.as_mut_view(),
        ggsw.as_view(),
        glwe.as_view(),
        fft,
        stack.rb_mut(),
    );
    //println!("{:?}", out.as_view());
    //println!("{:?}", &ggsw);
    //println!("{:?}", &glwe);
}
// fn main() {
//     external_product_bench::<u64>(16, 5., 17);
// }

// fn main() {
//     let mut fft_engine = FftEngine::new(()).unwrap();

//     let raw_input = 3_u64 << 59;
//     let raw_input_cleatext = 4_u64;
//     let lwe_dimension = LweDimension(750);
//     let noise = Variance(2_f64.powf(-104.));
//     const UNSAFE_SECRET: u128 = 0;
//     let mut engine = DefaultEngine::new(Box::new(UnixSeeder::new(UNSAFE_SECRET))).unwrap();
//     let cleartext: Cleartext64 = engine.create_cleartext_from(&raw_input_cleatext).unwrap();
//     let pt: Plaintext64 = engine.create_plaintext_from(&1).unwrap();
//     let key: GlweSecretKey64 = engine
//         .generate_new_glwe_secret_key(GlweDimension(2), PolynomialSize(1024))
//         .unwrap();

//     let B = DecompositionBaseLog(2);
//     let ell = DecompositionLevelCount(12);
//     let c: GgswCiphertext64 = engine
//         .encrypt_scalar_ggsw_ciphertext(&key, &pt, noise, ell, B)
//         .unwrap();
//     let complex_c: FftFourierGgswCiphertext64 = fft_engine.convert_ggsw_ciphertext(&c).unwrap();
//     let ct = engine.create_glwe_ciphertext_from(Vec::new(), PolynomialSize(1024));
//     let mut ct_out = engine.create_glwe_ciphertext_from(Vec::new(), PolynomialSize(1024));
//     fft_engine.discard_compute_external_product_glwe_ciphertext_ggsw_ciphertext(
//         &ct,
//         &complex_c,
//         &mut ct_out,
//     );
//     // c.external_product();

////     println!("Hello, world!");
// }
// fn main() -> Result<(), Box<dyn Error>> {
//     // DISCLAIMER: the parameters used here are only for test purpose, and are not secure.
//     let glwe_dimension = GlweDimension(2);
//     let polynomial_size = PolynomialSize(256);
//     let level = DecompositionLevelCount(1);
//     let base_log = DecompositionBaseLog(4);
//     // Here a hard-set encoding is applied (shift by 20 bits)
//     let input_ggsw = 3_u64 << 20;
//     let input_glwe = vec![3_u64 << 20; polynomial_size.0];
//     let noise = Variance(2_f64.powf(-55.));

//     // Unix seeder must be given a secret input.
//     // Here we just give it 0, which is totally unsafe.
//     const UNSAFE_SECRET: u128 = 0;
//     let mut default_engine = DefaultEngine::new(Box::new(UnixSeeder::new(UNSAFE_SECRET)))?;

//     let encoder = default_engine.create_encoder_from(&FloatEncoderMinMaxConfig {
//         min: 0.,
//         max: 10.,
//         nb_bit_precision: 8,
//         nb_bit_padding: 1,
//     })?;
//     let encoder_vector = default_engine.create_encoder_vector_from(
//         vec![
//             FloatEncoderMinMaxConfig {
//                 min: 0.,
//                 max: 10.,
//                 nb_bit_precision: 8,
//                 nb_bit_padding: 1,
//             };
//             256
//         ]
//         .as_slice(),
//     )?;

//     let one = 1.;
//     let one_vec = vec![1.; 256];
//     let cleartext: CleartextF64 = default_engine.create_cleartext_from(&one)?;
//     let plaintext: Plaintext64 = default_engine.encode_cleartext(&encoder, &cleartext)?;

//     let cleartext_vec = default_engine.create_cleartext_vector_from(&one_vec)?;
//     let plaintext_vec = default_engine.encode_cleartext_vector(&encoder_vector, &cleartext_vec)?;

//     let mut fft_engine = FftEngine::new(())?;
//     let key: GlweSecretKey64 =
//         default_engine.generate_new_glwe_secret_key(glwe_dimension, polynomial_size)?;

//     let ggsw =
//         default_engine.encrypt_scalar_ggsw_ciphertext(&key, &plaintext, noise, level, base_log)?;

//     let complex_ggsw: FftFourierGgswCiphertext64 = fft_engine.convert_ggsw_ciphertext(&ggsw)?;

//     let glwe = default_engine.encrypt_glwe_ciphertext(&key, &plaintext_vec, noise)?;

//     // We allocate an output ciphertext simply by cloning the input.
//     // The content of this output ciphertext will by wiped by the external product.
//     let mut product = default_engine.zero_encrypt_glwe_ciphertext(&key, noise)?;

//     fft_engine.discard_compute_external_product_glwe_ciphertext_ggsw_ciphertext(
//         &glwe,
//         &complex_ggsw,
//         &mut product,
//     )?;

//     let dec = default_engine.decrypt_glwe_ciphertext(&key, &product)?;
////     println!("{:?}", dec.plaintext_count());
//     let clear = default_engine.decode_plaintext_vector(&encoder_vector, &dec);
////     println!("{:?}", &clear);

//     // let dec3 = default_engine.retrieve_cleartext_vector(&dec);
//     assert_eq!(complex_ggsw.glwe_dimension(), glwe_dimension);
//     assert_eq!(complex_ggsw.polynomial_size(), polynomial_size);
//     assert_eq!(complex_ggsw.decomposition_base_log(), base_log);
//     assert_eq!(complex_ggsw.decomposition_level_count(), level);
//     return Ok(());
// }

// fn main() {
//     let n = 256;
//     let polynomial_size = PolynomialSize(n);
//     let glwe_size = GlweSize(3);
//     let decomposition_level_count = DecompositionLevelCount(4);
//     let decomposition_base_log = DecompositionBaseLog(2);

//     let mut out = GlweCiphertext::from_container(
//         avec![17u64; polynomial_size.0 * glwe_size.0].into_boxed_slice(),
//         polynomial_size,
//     );

//     let mut comp = c64::default();

//     comp.im = 2.;
//     comp.re = 12.;
//     let ggsw = FourierGgswCiphertext::new(
//         avec![
//         comp;
//         polynomial_size.0 / 2 * glwe_size.0 * glwe_size.0 * decomposition_level_count.0
//         ]
//         .into_boxed_slice(),
//         polynomial_size,
//         glwe_size,
//         decomposition_base_log,
//         decomposition_level_count,
//     );
//     let glwe = GlweCiphertext::from_container(
//         avec![3u64; polynomial_size.0 * glwe_size.0].into_boxed_slice(),
//         polynomial_size,
//     );
//     let fft = Fft::new(polynomial_size);
//     let fft = fft.as_view();

//     let mut mem = GlobalMemBuffer::new(
//         external_product_scratch::<u64>(glwe_size, polynomial_size, fft).unwrap(),
//     );
//     let mut stack = DynStack::new(&mut mem);

//     external_product(
//         out.as_mut_view(),
//         ggsw.as_view(),
//         glwe.as_view(),
//         fft,
//         stack.rb_mut(),
//     );
////     println!("{:?}", out);
////     println!("{:?}", c64::default());
// }

fn works() {
    let mut fft_engine = FftEngine::new(()).unwrap();

    let raw_input = vec![3_u64 << 59; 64];
    let noise = Variance(2_f64.powf(-104.));
    const UNSAFE_SECRET: u128 = 0;
    let mut engine = DefaultEngine::new(Box::new(UnixSeeder::new(UNSAFE_SECRET))).unwrap();
    let plaintext_vector: PlaintextVector64 =
        engine.create_plaintext_vector_from(&raw_input).unwrap();
    let ggsw_pt: Plaintext64 = engine.create_plaintext_from(&1).unwrap();
    let key: GlweSecretKey64 = engine
        .generate_new_glwe_secret_key(GlweDimension(2), PolynomialSize(64))
        .unwrap();

    let B = DecompositionBaseLog(2);
    let ell = DecompositionLevelCount(12);
    let c: GgswCiphertext64 = engine
        .encrypt_scalar_ggsw_ciphertext(&key, &ggsw_pt, noise, ell, B)
        .unwrap();
    let complex_c: FftFourierGgswCiphertext64 = fft_engine.convert_ggsw_ciphertext(&c).unwrap();
    let ct = engine
        .encrypt_glwe_ciphertext(&key, &plaintext_vector, noise)
        .unwrap();
    let mut ct_out = engine.zero_encrypt_glwe_ciphertext(&key, noise).unwrap();
    fft_engine.discard_compute_external_product_glwe_ciphertext_ggsw_ciphertext(
        &ct,
        &complex_c,
        &mut ct_out,
    );
    let _dec = engine.decrypt_glwe_ciphertext(&key, &ct_out);

    //println!("output ct: {:?}", ct_out);
    //println!("output ct: {:?}", dec);

    let x: Vec<u64> = vec![
        1729378958375949598,
        1729380057886768378,
        1729380057887644426,
        1729378958375847111,
        1729380057888629210,
        1729378958376659076,
        1729383356422939005,
        1729378958375386024,
        1729378958376652936,
        1729381157399650297,
        1729384455935327563,
        1729382256911128108,
        1729378958376850097,
        1729380057888432965,
        1729382256910885101,
        1729381157399766807,
        1729381157398525461,
        1729383356424247485,
        1729383356423216345,
        1729382256912855713,
        1729381157399000073,
        1729381157400197897,
        1729382256910299807,
        1729381157400660629,
        1729382256910985485,
        1729380057889847560,
        1729383356422850082,
        1729380057888263549,
        1729378958375634604,
        1729376759351999320,
        1729380057887772971,
        1729380057888325504,
        1729381157399783687,
        1729381157398409517,
        1729377858864206087,
        1729377858863795599,
        1729378958374368640,
        1729382256910091995,
        1729382256910727563,
        1729381157399750981,
        1729382256910216817,
        1729378958375087114,
        1729378958375416166,
        1729377858864480058,
        1729381157399525981,
        1729381157399760382,
        1729384455933772705,
        1729384455934303384,
        1729381157398539262,
        1729381157400672047,
        1729382256911257189,
        1729381157399508306,
        1729382256911276553,
        1729382256911790042,
        1729384455934253704,
        1729382256910511850,
        1729386654957500872,
        1729384455935110492,
        1729387754468723540,
        1729384455934066602,
        1729381157399902079,
        1729384455932510163,
        1729385555445098613,
        1729385555444978095,
    ];
    for i in x {
        print!("{:?}, ", (i as f64) / (1u128 << 59) as f64);
    }
}

fn main() {
    let B = DecompositionBaseLog(2);
    let ell = DecompositionLevelCount(12);
    let poly_size = PolynomialSize(64);
    let glwe_dimension = GlweDimension(2);

    let noise = Variance(2_f64.powf(-104.));
    const UNSAFE_SECRET: u128 = 0;
    let mut engine = DefaultEngine::new(Box::new(UnixSeeder::new(UNSAFE_SECRET))).unwrap();
    let mut fft_engine = FftEngine::new(()).unwrap();

    let key: GlweSecretKey64 = engine
        .generate_new_glwe_secret_key(glwe_dimension, poly_size)
        .unwrap();

    let ggsw_pt1: Plaintext64 = engine.create_plaintext_from(&1).unwrap();
    let c1: GgswCiphertext64 = engine
        .encrypt_scalar_ggsw_ciphertext(&key, &ggsw_pt1, noise, ell, B)
        .unwrap();

    let ggsw_pt2: Plaintext64 = engine.create_plaintext_from(&1).unwrap();
    let mut c2: GgswCiphertext64 = engine
        .encrypt_scalar_ggsw_ciphertext(&key, &ggsw_pt2, noise, ell, B)
        .unwrap();

    let complex_c: FftFourierGgswCiphertext64 = fft_engine.convert_ggsw_ciphertext(&c1).unwrap();

    // let mut list = c2.0.as_mut_glwe_list().ciphertext_iter_mut().enumerate();
    let list2 =
        c2.0.as_mut_tensor()
            .as_mut_container()
            .into_iter()
            .enumerate();

    let mut aux = vec![engine.zero_encrypt_glwe_ciphertext(&key, noise).unwrap(); list2.len()];

    for (i, val) in list2 {
        let owned_container = vec![val.clone(); (glwe_dimension.0 + 1) * poly_size.0];

        let ciphertext: GlweCiphertext64 = engine
            .create_glwe_ciphertext_from(owned_container, poly_size)
            .unwrap();

        fft_engine
            .discard_compute_external_product_glwe_ciphertext_ggsw_ciphertext(
                &ciphertext,
                &complex_c,
                &mut aux[i],
            )
            .unwrap();
    }

    let ggsw_pt3: Plaintext64 = engine.create_plaintext_from(&1).unwrap();

    let mut c3: GgswCiphertext64 = engine
        .encrypt_scalar_ggsw_ciphertext(&key, &ggsw_pt3, noise, ell, B)
        .unwrap();

    for (i, mut it) in c3.0.as_mut_glwe_list().ciphertext_iter_mut().enumerate() {
        let aux_container_i = engine
            .consume_retrieve_glwe_ciphertext(aux[i].clone())
            .unwrap();

        let mut tensor = Tensor::allocate(0u64, aux_container_i.len());
        tensor.as_mut_container().clone_from(&aux_container_i);

        it.as_mut_polynomial_list()
            .as_mut_tensor()
            .fill_with_copy(&tensor);
    }

    let last_row_x = c3.0.as_mut_glwe_list();
    let last_row = last_row_x.ciphertext_iter().last().unwrap();

    let raw_buffer = last_row.as_polynomial_list().into_container();

    let view: GlweCiphertextView64 = engine
        .create_glwe_ciphertext_from(raw_buffer, poly_size)
        .unwrap();

    let mut serialization_engine = DefaultSerializationEngine::new(()).unwrap();
    let serialized = serialization_engine.serialize(&view).unwrap();

    let recovered: GlweCiphertext64 = serialization_engine
        .deserialize(serialized.as_slice())
        .unwrap();

    let decryption = engine.decrypt_glwe_ciphertext(&key, &recovered).unwrap();

    println!("{:?}", decryption);

    let decr: Vec<u64> = vec![
        2699142716511241179,
        6211033433161973302,
        6211033433162992712,
        9722924149814508854,
        13234814866464732886,
        16746705583116515478,
        16746705583116799320,
        16746705583117453807,
        5323742942709315663,
        8835633659360882925,
        8835633659361043085,
        12347524376013010962,
        15859415092663636480,
        924561735605533248,
        4436452452257990462,
        11460233885560666859,
        14972124602211266426,
        3549161961805004012,
        3549161961805760812,
        7061052678457438183,
        10572943395107838715,
        14084834111760359345,
        2661871471353888435,
        9685652904656042691,
        13197543621307007728,
        1774580980900647642,
        5286471697552884833,
        8798362414204275828,
        12310253130855377965,
        15822143847508267529,
        887290490450448905,
        4399181207100519716,
        7911071923752402644,
        11422962640404771378,
        18446744073707766252,
        3511890716649540299,
        7023781433300359539,
        10535672149952810841,
        14047562866604963539,
        17559453583255131209,
        6136490942849593811,
        9648381659501220577,
        13160272376152488092,
        16672163092804092536,
        5249200452396693598,
        12272981885700905938,
        15784872602353314266,
        4361909961945842699,
        7873800678597908364,
        14897582111901691204,
        14897582111900735083,
        3474619471494745714,
        6986510188146014668,
        10498400904798066805,
        17522182338101193875,
        2587328981042569365,
        2587328981042706431,
        6099219697695455464,
        6099219697695114978,
        9611110414345579444,
        9611110414346778695,
        16634891847650074306,
        5211929207243544111,
        12235710640546351073,
    ];
    for x in decr.clone() {
        print!("{:?}, ", (x as f64) / (1u128 << 52) as f64);
    }

    println!("\n\n\n");
    for x in decr {
        print!("{:?}, ", (x as f64) / (1u128 << 64) as f64);
    }
}
