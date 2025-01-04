#[cfg(feature = "Mono+Security+Interface+CipherSuiteCode")]
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CipherSuiteCode {
    #[default]
    TLS_DHE_DSS_EXPORT_WITH_DES40_CBC_SHA = 17u16,
    TLS_DHE_DSS_WITH_3DES_EDE_CBC_SHA = 19u16,
    TLS_DHE_DSS_WITH_AES_128_CBC_SHA = 50u16,
    TLS_DHE_DSS_WITH_AES_128_CBC_SHA256 = 64u16,
    TLS_DHE_DSS_WITH_AES_128_GCM_SHA256 = 162u16,
    TLS_DHE_DSS_WITH_AES_256_CBC_SHA = 56u16,
    TLS_DHE_DSS_WITH_AES_256_CBC_SHA256 = 106u16,
    TLS_DHE_DSS_WITH_AES_256_GCM_SHA384 = 163u16,
    TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA = 68u16,
    TLS_DHE_DSS_WITH_CAMELLIA_128_CBC_SHA256 = 189u16,
    TLS_DHE_DSS_WITH_CAMELLIA_128_GCM_SHA256 = 49280u16,
    TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA = 135u16,
    TLS_DHE_DSS_WITH_CAMELLIA_256_CBC_SHA256 = 195u16,
    TLS_DHE_DSS_WITH_CAMELLIA_256_GCM_SHA384 = 49281u16,
    TLS_DHE_DSS_WITH_DES_CBC_SHA = 18u16,
    TLS_DHE_DSS_WITH_SEED_CBC_SHA = 153u16,
    TLS_DHE_PSK_WITH_3DES_EDE_CBC_SHA = 143u16,
    TLS_DHE_PSK_WITH_AES_128_CBC_SHA = 144u16,
    TLS_DHE_PSK_WITH_AES_128_CBC_SHA256 = 178u16,
    TLS_DHE_PSK_WITH_AES_128_CCM = 49318u16,
    TLS_DHE_PSK_WITH_AES_128_GCM_SHA256 = 170u16,
    TLS_DHE_PSK_WITH_AES_256_CBC_SHA = 145u16,
    TLS_DHE_PSK_WITH_AES_256_CBC_SHA384 = 179u16,
    TLS_DHE_PSK_WITH_AES_256_CCM = 49319u16,
    TLS_DHE_PSK_WITH_AES_256_GCM_SHA384 = 171u16,
    TLS_DHE_PSK_WITH_CAMELLIA_128_CBC_SHA256 = 49302u16,
    TLS_DHE_PSK_WITH_CAMELLIA_128_GCM_SHA256 = 49296u16,
    TLS_DHE_PSK_WITH_CAMELLIA_256_CBC_SHA384 = 49303u16,
    TLS_DHE_PSK_WITH_CAMELLIA_256_GCM_SHA384 = 49297u16,
    TLS_DHE_PSK_WITH_ESTREAM_SALSA20_SHA1 = 58396u16,
    TLS_DHE_PSK_WITH_NULL_SHA = 45u16,
    TLS_DHE_PSK_WITH_NULL_SHA256 = 180u16,
    TLS_DHE_PSK_WITH_NULL_SHA384 = 181u16,
    TLS_DHE_PSK_WITH_RC4_128_SHA = 142u16,
    TLS_DHE_PSK_WITH_SALSA20_SHA1 = 58397u16,
    TLS_DHE_RSA_EXPORT_WITH_DES40_CBC_SHA = 20u16,
    TLS_DHE_RSA_WITH_3DES_EDE_CBC_SHA = 22u16,
    TLS_DHE_RSA_WITH_AES_128_CBC_SHA = 51u16,
    TLS_DHE_RSA_WITH_AES_128_CBC_SHA256 = 103u16,
    TLS_DHE_RSA_WITH_AES_128_CCM = 49310u16,
    TLS_DHE_RSA_WITH_AES_128_CCM_8 = 49314u16,
    TLS_DHE_RSA_WITH_AES_128_GCM_SHA256 = 158u16,
    TLS_DHE_RSA_WITH_AES_256_CBC_SHA = 57u16,
    TLS_DHE_RSA_WITH_AES_256_CBC_SHA256 = 107u16,
    TLS_DHE_RSA_WITH_AES_256_CCM = 49311u16,
    TLS_DHE_RSA_WITH_AES_256_CCM_8 = 49315u16,
    TLS_DHE_RSA_WITH_AES_256_GCM_SHA384 = 159u16,
    TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA = 69u16,
    TLS_DHE_RSA_WITH_CAMELLIA_128_CBC_SHA256 = 190u16,
    TLS_DHE_RSA_WITH_CAMELLIA_128_GCM_SHA256 = 49276u16,
    TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA = 136u16,
    TLS_DHE_RSA_WITH_CAMELLIA_256_CBC_SHA256 = 196u16,
    TLS_DHE_RSA_WITH_CAMELLIA_256_GCM_SHA384 = 49277u16,
    TLS_DHE_RSA_WITH_CHACHA20_POLY1305_SHA256 = 52245u16,
    TLS_DHE_RSA_WITH_DES_CBC_SHA = 21u16,
    TLS_DHE_RSA_WITH_ESTREAM_SALSA20_SHA1 = 58398u16,
    TLS_DHE_RSA_WITH_SALSA20_SHA1 = 58399u16,
    TLS_DHE_RSA_WITH_SEED_CBC_SHA = 154u16,
    TLS_DH_DSS_EXPORT_WITH_DES40_CBC_SHA = 11u16,
    TLS_DH_DSS_WITH_3DES_EDE_CBC_SHA = 13u16,
    TLS_DH_DSS_WITH_AES_128_CBC_SHA = 48u16,
    TLS_DH_DSS_WITH_AES_128_CBC_SHA256 = 62u16,
    TLS_DH_DSS_WITH_AES_128_GCM_SHA256 = 164u16,
    TLS_DH_DSS_WITH_AES_256_CBC_SHA = 54u16,
    TLS_DH_DSS_WITH_AES_256_CBC_SHA256 = 104u16,
    TLS_DH_DSS_WITH_AES_256_GCM_SHA384 = 165u16,
    TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA = 66u16,
    TLS_DH_DSS_WITH_CAMELLIA_128_CBC_SHA256 = 187u16,
    TLS_DH_DSS_WITH_CAMELLIA_128_GCM_SHA256 = 49282u16,
    TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA = 133u16,
    TLS_DH_DSS_WITH_CAMELLIA_256_CBC_SHA256 = 193u16,
    TLS_DH_DSS_WITH_CAMELLIA_256_GCM_SHA384 = 49283u16,
    TLS_DH_DSS_WITH_DES_CBC_SHA = 12u16,
    TLS_DH_DSS_WITH_SEED_CBC_SHA = 151u16,
    TLS_DH_RSA_EXPORT_WITH_DES40_CBC_SHA = 14u16,
    TLS_DH_RSA_WITH_3DES_EDE_CBC_SHA = 16u16,
    TLS_DH_RSA_WITH_AES_128_CBC_SHA = 49u16,
    TLS_DH_RSA_WITH_AES_128_CBC_SHA256 = 63u16,
    TLS_DH_RSA_WITH_AES_128_GCM_SHA256 = 160u16,
    TLS_DH_RSA_WITH_AES_256_CBC_SHA = 55u16,
    TLS_DH_RSA_WITH_AES_256_CBC_SHA256 = 105u16,
    TLS_DH_RSA_WITH_AES_256_GCM_SHA384 = 161u16,
    TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA = 67u16,
    TLS_DH_RSA_WITH_CAMELLIA_128_CBC_SHA256 = 188u16,
    TLS_DH_RSA_WITH_CAMELLIA_128_GCM_SHA256 = 49278u16,
    TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA = 134u16,
    TLS_DH_RSA_WITH_CAMELLIA_256_CBC_SHA256 = 194u16,
    TLS_DH_RSA_WITH_CAMELLIA_256_GCM_SHA384 = 49279u16,
    TLS_DH_RSA_WITH_DES_CBC_SHA = 15u16,
    TLS_DH_RSA_WITH_SEED_CBC_SHA = 152u16,
    TLS_DH_anon_EXPORT_WITH_DES40_CBC_SHA = 25u16,
    TLS_DH_anon_EXPORT_WITH_RC4_40_MD5 = 23u16,
    TLS_DH_anon_WITH_3DES_EDE_CBC_SHA = 27u16,
    TLS_DH_anon_WITH_AES_128_CBC_SHA = 52u16,
    TLS_DH_anon_WITH_AES_128_CBC_SHA256 = 108u16,
    TLS_DH_anon_WITH_AES_128_GCM_SHA256 = 166u16,
    TLS_DH_anon_WITH_AES_256_CBC_SHA = 58u16,
    TLS_DH_anon_WITH_AES_256_CBC_SHA256 = 109u16,
    TLS_DH_anon_WITH_AES_256_GCM_SHA384 = 167u16,
    TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA = 70u16,
    TLS_DH_anon_WITH_CAMELLIA_128_CBC_SHA256 = 191u16,
    TLS_DH_anon_WITH_CAMELLIA_128_GCM_SHA256 = 49284u16,
    TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA = 137u16,
    TLS_DH_anon_WITH_CAMELLIA_256_CBC_SHA256 = 197u16,
    TLS_DH_anon_WITH_CAMELLIA_256_GCM_SHA384 = 49285u16,
    TLS_DH_anon_WITH_DES_CBC_SHA = 26u16,
    TLS_DH_anon_WITH_RC4_128_MD5 = 24u16,
    TLS_DH_anon_WITH_SEED_CBC_SHA = 155u16,
    TLS_ECDHE_ECDSA_WITH_3DES_EDE_CBC_SHA = 49160u16,
    TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA = 49161u16,
    TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA256 = 49187u16,
    TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 = 49195u16,
    TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA = 49162u16,
    TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA384 = 49188u16,
    TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 = 49196u16,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_CBC_SHA256 = 49266u16,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_128_GCM_SHA256 = 49286u16,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_CBC_SHA384 = 49267u16,
    TLS_ECDHE_ECDSA_WITH_CAMELLIA_256_GCM_SHA384 = 49287u16,
    TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 = 52244u16,
    TLS_ECDHE_ECDSA_WITH_ESTREAM_SALSA20_SHA1 = 58388u16,
    TLS_ECDHE_ECDSA_WITH_NULL_SHA = 49158u16,
    TLS_ECDHE_ECDSA_WITH_RC4_128_SHA = 49159u16,
    TLS_ECDHE_ECDSA_WITH_SALSA20_SHA1 = 58389u16,
    TLS_ECDHE_PSK_WITH_3DES_EDE_CBC_SHA = 49204u16,
    TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA = 49205u16,
    TLS_ECDHE_PSK_WITH_AES_128_CBC_SHA256 = 49207u16,
    TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA = 49206u16,
    TLS_ECDHE_PSK_WITH_AES_256_CBC_SHA384 = 49208u16,
    TLS_ECDHE_PSK_WITH_CAMELLIA_128_CBC_SHA256 = 49306u16,
    TLS_ECDHE_PSK_WITH_CAMELLIA_256_CBC_SHA384 = 49307u16,
    TLS_ECDHE_PSK_WITH_ESTREAM_SALSA20_SHA1 = 58392u16,
    TLS_ECDHE_PSK_WITH_NULL_SHA = 49209u16,
    TLS_ECDHE_PSK_WITH_NULL_SHA256 = 49210u16,
    TLS_ECDHE_PSK_WITH_NULL_SHA384 = 49211u16,
    TLS_ECDHE_PSK_WITH_RC4_128_SHA = 49203u16,
    TLS_ECDHE_PSK_WITH_SALSA20_SHA1 = 58393u16,
    TLS_ECDHE_RSA_WITH_3DES_EDE_CBC_SHA = 49170u16,
    TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA = 49171u16,
    TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256 = 49191u16,
    TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256 = 49199u16,
    TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA = 49172u16,
    TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA384 = 49192u16,
    TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384 = 49200u16,
    TLS_ECDHE_RSA_WITH_CAMELLIA_128_CBC_SHA256 = 49270u16,
    TLS_ECDHE_RSA_WITH_CAMELLIA_128_GCM_SHA256 = 49290u16,
    TLS_ECDHE_RSA_WITH_CAMELLIA_256_CBC_SHA384 = 49271u16,
    TLS_ECDHE_RSA_WITH_CAMELLIA_256_GCM_SHA384 = 49291u16,
    TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 = 52243u16,
    TLS_ECDHE_RSA_WITH_ESTREAM_SALSA20_SHA1 = 58386u16,
    TLS_ECDHE_RSA_WITH_NULL_SHA = 49168u16,
    TLS_ECDHE_RSA_WITH_RC4_128_SHA = 49169u16,
    TLS_ECDHE_RSA_WITH_SALSA20_SHA1 = 58387u16,
    TLS_ECDH_ECDSA_WITH_3DES_EDE_CBC_SHA = 49155u16,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA = 49156u16,
    TLS_ECDH_ECDSA_WITH_AES_128_CBC_SHA256 = 49189u16,
    TLS_ECDH_ECDSA_WITH_AES_128_GCM_SHA256 = 49197u16,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA = 49157u16,
    TLS_ECDH_ECDSA_WITH_AES_256_CBC_SHA384 = 49190u16,
    TLS_ECDH_ECDSA_WITH_AES_256_GCM_SHA384 = 49198u16,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_128_CBC_SHA256 = 49268u16,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_128_GCM_SHA256 = 49288u16,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_256_CBC_SHA384 = 49269u16,
    TLS_ECDH_ECDSA_WITH_CAMELLIA_256_GCM_SHA384 = 49289u16,
    TLS_ECDH_ECDSA_WITH_NULL_SHA = 49153u16,
    TLS_ECDH_ECDSA_WITH_RC4_128_SHA = 49154u16,
    TLS_ECDH_RSA_WITH_3DES_EDE_CBC_SHA = 49165u16,
    TLS_ECDH_RSA_WITH_AES_128_CBC_SHA = 49166u16,
    TLS_ECDH_RSA_WITH_AES_128_CBC_SHA256 = 49193u16,
    TLS_ECDH_RSA_WITH_AES_128_GCM_SHA256 = 49201u16,
    TLS_ECDH_RSA_WITH_AES_256_CBC_SHA = 49167u16,
    TLS_ECDH_RSA_WITH_AES_256_CBC_SHA384 = 49194u16,
    TLS_ECDH_RSA_WITH_AES_256_GCM_SHA384 = 49202u16,
    TLS_ECDH_RSA_WITH_CAMELLIA_128_CBC_SHA256 = 49272u16,
    TLS_ECDH_RSA_WITH_CAMELLIA_128_GCM_SHA256 = 49292u16,
    TLS_ECDH_RSA_WITH_CAMELLIA_256_CBC_SHA384 = 49273u16,
    TLS_ECDH_RSA_WITH_CAMELLIA_256_GCM_SHA384 = 49293u16,
    TLS_ECDH_RSA_WITH_NULL_SHA = 49163u16,
    TLS_ECDH_RSA_WITH_RC4_128_SHA = 49164u16,
    TLS_ECDH_anon_WITH_3DES_EDE_CBC_SHA = 49175u16,
    TLS_ECDH_anon_WITH_AES_128_CBC_SHA = 49176u16,
    TLS_ECDH_anon_WITH_AES_256_CBC_SHA = 49177u16,
    TLS_ECDH_anon_WITH_NULL_SHA = 49173u16,
    TLS_ECDH_anon_WITH_RC4_128_SHA = 49174u16,
    TLS_EMPTY_RENEGOTIATION_INFO_SCSV = 255u16,
    TLS_FALLBACK_SCSV = 22016u16,
    TLS_NULL_WITH_NULL_NULL = 0u16,
    TLS_PSK_DHE_WITH_AES_128_CCM_8 = 49322u16,
    TLS_PSK_DHE_WITH_AES_256_CCM_8 = 49323u16,
    TLS_PSK_WITH_3DES_EDE_CBC_SHA = 139u16,
    TLS_PSK_WITH_AES_128_CBC_SHA = 140u16,
    TLS_PSK_WITH_AES_128_CBC_SHA256 = 174u16,
    TLS_PSK_WITH_AES_128_CCM = 49316u16,
    TLS_PSK_WITH_AES_128_CCM_8 = 49320u16,
    TLS_PSK_WITH_AES_128_GCM_SHA256 = 168u16,
    TLS_PSK_WITH_AES_256_CBC_SHA = 141u16,
    TLS_PSK_WITH_AES_256_CBC_SHA384 = 175u16,
    TLS_PSK_WITH_AES_256_CCM = 49317u16,
    TLS_PSK_WITH_AES_256_CCM_8 = 49321u16,
    TLS_PSK_WITH_AES_256_GCM_SHA384 = 169u16,
    TLS_PSK_WITH_CAMELLIA_128_CBC_SHA256 = 49300u16,
    TLS_PSK_WITH_CAMELLIA_128_GCM_SHA256 = 49294u16,
    TLS_PSK_WITH_CAMELLIA_256_CBC_SHA384 = 49301u16,
    TLS_PSK_WITH_CAMELLIA_256_GCM_SHA384 = 49295u16,
    TLS_PSK_WITH_ESTREAM_SALSA20_SHA1 = 58390u16,
    TLS_PSK_WITH_NULL_SHA = 44u16,
    TLS_PSK_WITH_NULL_SHA256 = 176u16,
    TLS_PSK_WITH_NULL_SHA384 = 177u16,
    TLS_PSK_WITH_RC4_128_SHA = 138u16,
    TLS_PSK_WITH_SALSA20_SHA1 = 58391u16,
    TLS_RSA_EXPORT_WITH_DES40_CBC_SHA = 8u16,
    TLS_RSA_EXPORT_WITH_RC2_CBC_40_MD5 = 6u16,
    TLS_RSA_EXPORT_WITH_RC4_40_MD5 = 3u16,
    TLS_RSA_PSK_WITH_3DES_EDE_CBC_SHA = 147u16,
    TLS_RSA_PSK_WITH_AES_128_CBC_SHA = 148u16,
    TLS_RSA_PSK_WITH_AES_128_CBC_SHA256 = 182u16,
    TLS_RSA_PSK_WITH_AES_128_GCM_SHA256 = 172u16,
    TLS_RSA_PSK_WITH_AES_256_CBC_SHA = 149u16,
    TLS_RSA_PSK_WITH_AES_256_CBC_SHA384 = 183u16,
    TLS_RSA_PSK_WITH_AES_256_GCM_SHA384 = 173u16,
    TLS_RSA_PSK_WITH_CAMELLIA_128_CBC_SHA256 = 49304u16,
    TLS_RSA_PSK_WITH_CAMELLIA_128_GCM_SHA256 = 49298u16,
    TLS_RSA_PSK_WITH_CAMELLIA_256_CBC_SHA384 = 49305u16,
    TLS_RSA_PSK_WITH_CAMELLIA_256_GCM_SHA384 = 49299u16,
    TLS_RSA_PSK_WITH_ESTREAM_SALSA20_SHA1 = 58394u16,
    TLS_RSA_PSK_WITH_NULL_SHA = 46u16,
    TLS_RSA_PSK_WITH_NULL_SHA256 = 184u16,
    TLS_RSA_PSK_WITH_NULL_SHA384 = 185u16,
    TLS_RSA_PSK_WITH_RC4_128_SHA = 146u16,
    TLS_RSA_PSK_WITH_SALSA20_SHA1 = 58395u16,
    TLS_RSA_WITH_3DES_EDE_CBC_SHA = 10u16,
    TLS_RSA_WITH_AES_128_CBC_SHA = 47u16,
    TLS_RSA_WITH_AES_128_CBC_SHA256 = 60u16,
    TLS_RSA_WITH_AES_128_CCM = 49308u16,
    TLS_RSA_WITH_AES_128_CCM_8 = 49312u16,
    TLS_RSA_WITH_AES_128_GCM_SHA256 = 156u16,
    TLS_RSA_WITH_AES_256_CBC_SHA = 53u16,
    TLS_RSA_WITH_AES_256_CBC_SHA256 = 61u16,
    TLS_RSA_WITH_AES_256_CCM = 49309u16,
    TLS_RSA_WITH_AES_256_CCM_8 = 49313u16,
    TLS_RSA_WITH_AES_256_GCM_SHA384 = 157u16,
    TLS_RSA_WITH_CAMELLIA_128_CBC_SHA = 65u16,
    TLS_RSA_WITH_CAMELLIA_128_CBC_SHA256 = 186u16,
    TLS_RSA_WITH_CAMELLIA_128_GCM_SHA256 = 49274u16,
    TLS_RSA_WITH_CAMELLIA_256_CBC_SHA = 132u16,
    TLS_RSA_WITH_CAMELLIA_256_CBC_SHA256 = 192u16,
    TLS_RSA_WITH_CAMELLIA_256_GCM_SHA384 = 49275u16,
    TLS_RSA_WITH_DES_CBC_SHA = 9u16,
    TLS_RSA_WITH_ESTREAM_SALSA20_SHA1 = 58384u16,
    TLS_RSA_WITH_IDEA_CBC_SHA = 7u16,
    TLS_RSA_WITH_NULL_MD5 = 1u16,
    TLS_RSA_WITH_NULL_SHA = 2u16,
    TLS_RSA_WITH_NULL_SHA256 = 59u16,
    TLS_RSA_WITH_RC4_128_MD5 = 4u16,
    TLS_RSA_WITH_RC4_128_SHA = 5u16,
    TLS_RSA_WITH_SALSA20_SHA1 = 58385u16,
    TLS_RSA_WITH_SEED_CBC_SHA = 150u16,
    TLS_SRP_SHA_DSS_WITH_3DES_EDE_CBC_SHA = 49180u16,
    TLS_SRP_SHA_DSS_WITH_AES_128_CBC_SHA = 49183u16,
    TLS_SRP_SHA_DSS_WITH_AES_256_CBC_SHA = 49186u16,
    TLS_SRP_SHA_RSA_WITH_3DES_EDE_CBC_SHA = 49179u16,
    TLS_SRP_SHA_RSA_WITH_AES_128_CBC_SHA = 49182u16,
    TLS_SRP_SHA_RSA_WITH_AES_256_CBC_SHA = 49185u16,
    TLS_SRP_SHA_WITH_3DES_EDE_CBC_SHA = 49178u16,
    TLS_SRP_SHA_WITH_AES_128_CBC_SHA = 49181u16,
    TLS_SRP_SHA_WITH_AES_256_CBC_SHA = 49184u16,
}
#[cfg(feature = "Mono+Security+Interface+CipherSuiteCode")]
quest_hook::libil2cpp::unsafe_impl_value_type!(
    in quest_hook::libil2cpp for crate ::Mono::Security::Interface::CipherSuiteCode =>
    "Mono.Security.Interface"."CipherSuiteCode"
);
