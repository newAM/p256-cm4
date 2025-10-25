#![no_std]
#![no_main]
#![cfg(test)]

use defmt_semihosting as _; // global logger

#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    use cortex_m_semihosting::debug;

    defmt::error!("{}", defmt::Display2Format(info));
    debug::exit(debug::EXIT_FAILURE);
    loop {}
}

// FIPS 186-4 P-256,SHA256 vectors, from `186-4ecdsatestvectors/SigVer.rsp`, sha256: e9841c3d12e323042751460d0b8ef4bc59c2640105ec7da4852775f80ab10191
pub const VECTORS_SHA256: [Vector; 15] = [
    vector! {
        Msg = "e4796db5f785f207aa30d311693b3702821dff1168fd2e04c0836825aefd850d9aa60326d88cde1a23c7745351392ca2288d632c264f197d05cd424a30336c19fd09bb229654f0222fcb881a4b35c290a093ac159ce13409111ff0358411133c24f5b8e2090d6db6558afc36f06ca1f6ef779785adba68db27a409859fc4c4a0"
        Qx = "87f8f2b218f49845f6f10eec3877136269f5c1a54736dbdf69f89940cad41555"
        Qy = "e15f369036f49842fac7a86c8a2b0557609776814448b8f5e84aa9f4395205e9"
        R = "d19ff48b324915576416097d2544f7cbdf8768b1454ad20e0baac50e211f23b0"
        S = "a3e81e59311cdfff2d4784949f7a2cb50ba6c3a91fa54710568e61aca3e847c6"
        Result = "F (3 - S changed)"
    },
    vector! {
        Msg = "069a6e6b93dfee6df6ef6997cd80dd2182c36653cef10c655d524585655462d683877f95ecc6d6c81623d8fac4e900ed0019964094e7de91f1481989ae1873004565789cbf5dc56c62aedc63f62f3b894c9c6f7788c8ecaadc9bd0e81ad91b2b3569ea12260e93924fdddd3972af5273198f5efda0746219475017557616170e"
        Qx = "5cf02a00d205bdfee2016f7421807fc38ae69e6b7ccd064ee689fc1a94a9f7d2"
        Qy = "ec530ce3cc5c9d1af463f264d685afe2b4db4b5828d7e61b748930f3ce622a85"
        R = "dc23d130c6117fb5751201455e99f36f59aba1a6a21cf2d0e7481a97451d6693"
        S = "d6ce7708c18dbf35d4f8aa7240922dc6823f2e7058cbc1484fcad1599db5018c"
        Result = "F (2 - R changed)"
    },
    vector! {
        Msg = "df04a346cf4d0e331a6db78cca2d456d31b0a000aa51441defdb97bbeb20b94d8d746429a393ba88840d661615e07def615a342abedfa4ce912e562af714959896858af817317a840dcff85a057bb91a3c2bf90105500362754a6dd321cdd86128cfc5f04667b57aa78c112411e42da304f1012d48cd6a7052d7de44ebcc01de"
        Qx = "2ddfd145767883ffbb0ac003ab4a44346d08fa2570b3120dcce94562422244cb"
        Qy = "5f70c7d11ac2b7a435ccfbbae02c3df1ea6b532cc0e9db74f93fffca7c6f9a64"
        R = "9913111cff6f20c5bf453a99cd2c2019a4e749a49724a08774d14e4c113edda8"
        S = "9467cd4cd21ecb56b0cab0a9a453b43386845459127a952421f5c6382866c5cc"
        Result = "F (4 - Q changed)"
    },
    vector! {
        Msg = "e1130af6a38ccb412a9c8d13e15dbfc9e69a16385af3c3f1e5da954fd5e7c45fd75e2b8c36699228e92840c0562fbf3772f07e17f1add56588dd45f7450e1217ad239922dd9c32695dc71ff2424ca0dec1321aa47064a044b7fe3c2b97d03ce470a592304c5ef21eed9f93da56bb232d1eeb0035f9bf0dfafdcc4606272b20a3"
        Qx = "e424dc61d4bb3cb7ef4344a7f8957a0c5134e16f7a67c074f82e6e12f49abf3c"
        Qy = "970eed7aa2bc48651545949de1dddaf0127e5965ac85d1243d6f60e7dfaee927"
        R = "bf96b99aa49c705c910be33142017c642ff540c76349b9dab72f981fd9347f4f"
        S = "17c55095819089c2e03b9cd415abdf12444e323075d98f31920b9e0f57ec871c"
        Result = "P (0 )"
    },
    vector! {
        Msg = "73c5f6a67456ae48209b5f85d1e7de7758bf235300c6ae2bdceb1dcb27a7730fb68c950b7fcada0ecc4661d3578230f225a875e69aaa17f1e71c6be5c831f22663bac63d0c7a9635edb0043ff8c6f26470f02a7bc56556f1437f06dfa27b487a6c4290d8bad38d4879b334e341ba092dde4e4ae694a9c09302e2dbf443581c08"
        Qx = "e0fc6a6f50e1c57475673ee54e3a57f9a49f3328e743bf52f335e3eeaa3d2864"
        Qy = "7f59d689c91e463607d9194d99faf316e25432870816dde63f5d4b373f12f22a"
        R = "1d75830cd36f4c9aa181b2c4221e87f176b7f05b7c87824e82e396c88315c407"
        S = "cb2acb01dac96efc53a32d4a0d85d0c2e48955214783ecf50a4f0414a319c05a"
        Result = "P (0 )"
    },
    vector! {
        Msg = "666036d9b4a2426ed6585a4e0fd931a8761451d29ab04bd7dc6d0c5b9e38e6c2b263ff6cb837bd04399de3d757c6c7005f6d7a987063cf6d7e8cb38a4bf0d74a282572bd01d0f41e3fd066e3021575f0fa04f27b700d5b7ddddf50965993c3f9c7118ed78888da7cb221849b3260592b8e632d7c51e935a0ceae15207bedd548"
        Qx = "a849bef575cac3c6920fbce675c3b787136209f855de19ffe2e8d29b31a5ad86"
        Qy = "bf5fe4f7858f9b805bd8dcc05ad5e7fb889de2f822f3d8b41694e6c55c16b471"
        R = "25acc3aa9d9e84c7abf08f73fa4195acc506491d6fc37cb9074528a7db87b9d6"
        S = "9b21d5b5259ed3f2ef07dfec6cc90d3a37855d1ce122a85ba6a333f307d31537"
        Result = "F (2 - R changed)"
    },
    vector! {
        Msg = "7e80436bce57339ce8da1b5660149a20240b146d108deef3ec5da4ae256f8f894edcbbc57b34ce37089c0daa17f0c46cd82b5a1599314fd79d2fd2f446bd5a25b8e32fcf05b76d644573a6df4ad1dfea707b479d97237a346f1ec632ea5660efb57e8717a8628d7f82af50a4e84b11f21bdff6839196a880ae20b2a0918d58cd"
        Qx = "3dfb6f40f2471b29b77fdccba72d37c21bba019efa40c1c8f91ec405d7dcc5df"
        Qy = "f22f953f1e395a52ead7f3ae3fc47451b438117b1e04d613bc8555b7d6e6d1bb"
        R = "548886278e5ec26bed811dbb72db1e154b6f17be70deb1b210107decb1ec2a5a"
        S = "e93bfebd2f14f3d827ca32b464be6e69187f5edbd52def4f96599c37d58eee75"
        Result = "F (4 - Q changed)"
    },
    vector! {
        Msg = "1669bfb657fdc62c3ddd63269787fc1c969f1850fb04c933dda063ef74a56ce13e3a649700820f0061efabf849a85d474326c8a541d99830eea8131eaea584f22d88c353965dabcdc4bf6b55949fd529507dfb803ab6b480cd73ca0ba00ca19c438849e2cea262a1c57d8f81cd257fb58e19dec7904da97d8386e87b84948169"
        Qx = "69b7667056e1e11d6caf6e45643f8b21e7a4bebda463c7fdbc13bc98efbd0214"
        Qy = "d3f9b12eb46c7c6fda0da3fc85bc1fd831557f9abc902a3be3cb3e8be7d1aa2f"
        R = "288f7a1cd391842cce21f00e6f15471c04dc182fe4b14d92dc18910879799790"
        S = "247b3c4e89a3bcadfea73c7bfd361def43715fa382b8c3edf4ae15d6e55e9979"
        Result = "F (1 - Message changed)"
    },
    vector! {
        Msg = "3fe60dd9ad6caccf5a6f583b3ae65953563446c4510b70da115ffaa0ba04c076115c7043ab8733403cd69c7d14c212c655c07b43a7c71b9a4cffe22c2684788ec6870dc2013f269172c822256f9e7cc674791bf2d8486c0f5684283e1649576efc982ede17c7b74b214754d70402fb4bb45ad086cf2cf76b3d63f7fce39ac970"
        Qx = "bf02cbcf6d8cc26e91766d8af0b164fc5968535e84c158eb3bc4e2d79c3cc682"
        Qy = "069ba6cb06b49d60812066afa16ecf7b51352f2c03bd93ec220822b1f3dfba03"
        R = "f5acb06c59c2b4927fb852faa07faf4b1852bbb5d06840935e849c4d293d1bad"
        S = "049dab79c89cc02f1484c437f523e080a75f134917fda752f2d5ca397addfe5d"
        Result = "F (3 - S changed)"
    },
    vector! {
        Msg = "983a71b9994d95e876d84d28946a041f8f0a3f544cfcc055496580f1dfd4e312a2ad418fe69dbc61db230cc0c0ed97e360abab7d6ff4b81ee970a7e97466acfd9644f828ffec538abc383d0e92326d1c88c55e1f46a668a039beaa1be631a89129938c00a81a3ae46d4aecbf9707f764dbaccea3ef7665e4c4307fa0b0a3075c"
        Qx = "224a4d65b958f6d6afb2904863efd2a734b31798884801fcab5a590f4d6da9de"
        Qy = "178d51fddada62806f097aa615d33b8f2404e6b1479f5fd4859d595734d6d2b9"
        R = "87b93ee2fecfda54deb8dff8e426f3c72c8864991f8ec2b3205bb3b416de93d2"
        S = "4044a24df85be0cc76f21a4430b75b8e77b932a87f51e4eccbc45c263ebf8f66"
        Result = "F (2 - R changed)"
    },
    vector! {
        Msg = "4a8c071ac4fd0d52faa407b0fe5dab759f7394a5832127f2a3498f34aac287339e043b4ffa79528faf199dc917f7b066ad65505dab0e11e6948515052ce20cfdb892ffb8aa9bf3f1aa5be30a5bbe85823bddf70b39fd7ebd4a93a2f75472c1d4f606247a9821f1a8c45a6cb80545de2e0c6c0174e2392088c754e9c8443eb5af"
        Qx = "43691c7795a57ead8c5c68536fe934538d46f12889680a9cb6d055a066228369"
        Qy = "f8790110b3c3b281aa1eae037d4f1234aff587d903d93ba3af225c27ddc9ccac"
        R = "8acd62e8c262fa50dd9840480969f4ef70f218ebf8ef9584f199031132c6b1ce"
        S = "cfca7ed3d4347fb2a29e526b43c348ae1ce6c60d44f3191b6d8ea3a2d9c92154"
        Result = "F (3 - S changed)"
    },
    vector! {
        Msg = "0a3a12c3084c865daf1d302c78215d39bfe0b8bf28272b3c0b74beb4b7409db0718239de700785581514321c6440a4bbaea4c76fa47401e151e68cb6c29017f0bce4631290af5ea5e2bf3ed742ae110b04ade83a5dbd7358f29a85938e23d87ac8233072b79c94670ff0959f9c7f4517862ff829452096c78f5f2e9a7e4e9216"
        Qx = "9157dbfcf8cf385f5bb1568ad5c6e2a8652ba6dfc63bc1753edf5268cb7eb596"
        Qy = "972570f4313d47fc96f7c02d5594d77d46f91e949808825b3d31f029e8296405"
        R = "dfaea6f297fa320b707866125c2a7d5d515b51a503bee817de9faa343cc48eeb"
        S = "8f780ad713f9c3e5a4f7fa4c519833dfefc6a7432389b1e4af463961f09764f2"
        Result = "F (1 - Message changed)"
    },
    vector! {
        Msg = "785d07a3c54f63dca11f5d1a5f496ee2c2f9288e55007e666c78b007d95cc28581dce51f490b30fa73dc9e2d45d075d7e3a95fb8a9e1465ad191904124160b7c60fa720ef4ef1c5d2998f40570ae2a870ef3e894c2bc617d8a1dc85c3c55774928c38789b4e661349d3f84d2441a3b856a76949b9f1f80bc161648a1cad5588e"
        Qx = "072b10c081a4c1713a294f248aef850e297991aca47fa96a7470abe3b8acfdda"
        Qy = "9581145cca04a0fb94cedce752c8f0370861916d2a94e7c647c5373ce6a4c8f5"
        R = "09f5483eccec80f9d104815a1be9cc1a8e5b12b6eb482a65c6907b7480cf4f19"
        S = "a4f90e560c5e4eb8696cb276e5165b6a9d486345dedfb094a76e8442d026378d"
        Result = "F (4 - Q changed)"
    },
    vector! {
        Msg = "76f987ec5448dd72219bd30bf6b66b0775c80b394851a43ff1f537f140a6e7229ef8cd72ad58b1d2d20298539d6347dd5598812bc65323aceaf05228f738b5ad3e8d9fe4100fd767c2f098c77cb99c2992843ba3eed91d32444f3b6db6cd212dd4e5609548f4bb62812a920f6e2bf1581be1ebeebdd06ec4e971862cc42055ca"
        Qx = "09308ea5bfad6e5adf408634b3d5ce9240d35442f7fe116452aaec0d25be8c24"
        Qy = "f40c93e023ef494b1c3079b2d10ef67f3170740495ce2cc57f8ee4b0618b8ee5"
        R = "5cc8aa7c35743ec0c23dde88dabd5e4fcd0192d2116f6926fef788cddb754e73"
        S = "9c9c045ebaa1b828c32f82ace0d18daebf5e156eb7cbfdc1eff4399a8a900ae7"
        Result = "F (1 - Message changed)"
    },
    vector! {
        Msg = "60cd64b2cd2be6c33859b94875120361a24085f3765cb8b2bf11e026fa9d8855dbe435acf7882e84f3c7857f96e2baab4d9afe4588e4a82e17a78827bfdb5ddbd1c211fbc2e6d884cddd7cb9d90d5bf4a7311b83f352508033812c776a0e00c003c7e0d628e50736c7512df0acfa9f2320bd102229f46495ae6d0857cc452a84"
        Qx = "2d98ea01f754d34bbc3003df5050200abf445ec728556d7ed7d5c54c55552b6d"
        Qy = "9b52672742d637a32add056dfd6d8792f2a33c2e69dafabea09b960bc61e230a"
        R = "06108e525f845d0155bf60193222b3219c98e3d49424c2fb2a0987f825c17959"
        S = "62b5cdd591e5b507e560167ba8f6f7cda74673eb315680cb89ccbc4eec477dce"
        Result = "P (0 )"
    },
];

// FIPS 186-4 P-256,SHA512 vectors, from `186-4ecdsatestvectors/SigVer.rsp`, sha256: e9841c3d12e323042751460d0b8ef4bc59c2640105ec7da4852775f80ab10191
pub const VECTORS_SHA512: [Vector; 15] = [
    vector! {
        Msg = "273b063224ab48a1bf6c7efc93429d1f89de48fc4a4fa3ffe7a49ebba1a58ff5d208a9e4bff27b418252526243ba042d1605b6df3c2ec916ceef027853a41137f7bfb6fc63844de95f58e82b9ad2565f1367d2c69bd29100f6db21a8ab7ab58affd1661add0322bd915721378df9fa233ef0b7e0a0a85be31689e21891ec8977"
        Qx = "484e31e69ef70bb8527853c22c6b6b4cd2a51311dde66c7b63f097dbb6ab27bf"
        Qy = "e1ff8177f4061d4fbbacbbc70519f0fc8c8b6053d72af0fe4f048d615004f74e"
        R = "91a303d8fe3ab4176070f6406267f6b79bfe5eb5f62ae6aeb374d90667858518"
        S = "e152119cefa26826ea07ec40a428869132d70812c5578c5a260e48d6800e046a"
        Result = "F (1 - Message changed)"
    },
    vector! {
        Msg = "d64ea1a768b0de29ab018ae93baa645d078c70a2f7aa4acd4ae7526538ebd5f697a11927cfd0ddc9187c095f14ad30544cb63ede9353af8b23c18ce22843881fe2d7bde748fc69085921677858d87d2dc3e244f6c7e2c2b2bd791f450dfdd4ff0ddd35ab2ada4f1b90ab16ef2bf63b3fbe88ce8a5d5bb85430740d3744849c13"
        Qx = "8b75fc0129c9a78f8395c63ae9694b05cd6950665cf5da7d66118de451422624"
        Qy = "b394171981d4896d6e1b4ef2336d9befe7d27e1eb87f1c14b8ddda622af379dc"
        R = "17e298e67ad2af76f6892fdcead00a88256573868f79dc74431b55103058f0b0"
        S = "881328cd91e43d30133f6e471e0b9b04353b17893fb7614fd7333d812a3df6b4"
        Result = "F (4 - Q changed)"
    },
    vector! {
        Msg = "1db85445c9d8d1478a97dd9d6ffbf11ebcd2114d2ed4e8b6811171d947e7d4daedea35af6177debe2ef6d93f94ff9d770b45d458e91deb4eef59856425d7b00291aff9b6c9fa02375ec1a06f71f7548721790023301cf6ac7fee1d451228106ef4472681e652c8cd59b15d6d16f1e13440d888e265817cb4a654f7246e0980df"
        Qx = "76e51086e078b2b116fd1e9c6fa3d53f675ae40252fb9f0cc62817bd9ce8831d"
        Qy = "ca7e609a0b1d14b7c9249b53da0b2050450e2a25cb6c8f81c5311974a7efb576"
        R = "23b653faaa7d4552388771931803ce939dd5ee62d3fa72b019be1b2272c85592"
        S = "a03c6f5c54a10861d6b8922821708e9306fd6d5d10d566845a106539cbf4fadd"
        Result = "F (4 - Q changed)"
    },
    vector! {
        Msg = "918d9f420e927b3e0a55d276b8b40d8a2c5df748727ff72a438c7e6593f542274050dce727980d3ef90c8aa5c13d53f1e8d631ebb650dee11b94902bbd7c92b8186af9039c56c43f3110697792c8cd1614166f06d09cdb58dab168cc3680a8473b1a623bf85dba855eace579d9410d2c4ca5ede6dc1e3db81e233c34ae922f49"
        Qx = "bc7c8e09bd093468f706740a4130c544374fdc924a535ef02e9d3be6c6d3bbfa"
        Qy = "af3f813ae6646f5b6dbfb0f261fd42537705c800bb1647386343428a9f2e10fc"
        R = "6bd7ce95af25abfbf14aef4b17392f1da877ab562eca38d785fe39682e9c9324"
        S = "6688bea20c87bab34d420642da9bdd4c69456bdec50835887367bb4fb7cd8650"
        Result = "F (2 - R changed)"
    },
    vector! {
        Msg = "6e2932153301a4eef680e6428929adae988c108d668a31ff55d0489947d75ff81a46bf89e84d6401f023be6e87688fbcd784d785ca846735524acb52d00452c84040a479e7cc330936441d93bbe722a9432a6e1db112b5c9403b10272cb1347fd619d463f7a9d223ad76fde06d8a6883500fb843235abff98e241bdfb5538c3e"
        Qx = "9cb0cf69303dafc761d4e4687b4ecf039e6d34ab964af80810d8d558a4a8d6f7"
        Qy = "2d51233a1788920a86ee08a1962c79efa317fb7879e297dad2146db995fa1c78"
        R = "4b9f91e4285287261a1d1c923cf619cd52c175cfe7f1be60a5258c610348ba3d"
        S = "28c45f901d71c41b298638ec0d6a85d7fcb0c33bbfec5a9c810846b639289a84"
        Result = "P (0 )"
    },
    vector! {
        Msg = "2f48ec387f181035b350772e27f478ae6ec7487923692fae217e0f8636acd062a6ac39f7435f27a0ebcfd8187a91ef00fb68d106b8da4a1dedc5a40a4fae709e92b00fcc218de76417d75185e59dff76ec1543fb429d87c2ca8134ff5ae9b45456cad93fc67223c68293231395287dc0b756355660721a1f5df83bf5bcb8456e"
        Qx = "e31096c2d512fbf84f81e9bdb16f33121702897605b43a3db546f8fb695b5f6f"
        Qy = "6fbec6a04a8c59d61c900a851d8bf8522187d3ec2637b10fa8f377689e086bba"
        R = "1b244c21c08c0c0a10477fb7a21382d405b95c755088292859ca0e71bab68361"
        S = "852f4cbfd346e90f404e1dd5c4b2c1debca3ea1abefe8400685d703aea6c5c7f"
        Result = "F (4 - Q changed)"
    },
    vector! {
        Msg = "fd2e5de421ee46c9fe6290a33f95b394bd5b7762f23178f7f6834f1f056fa9a8831446403c098ff4dd764173f974be4c89d376119613a4a1890f6fc2ddff862bda292dd49f5410d9b1cfe1d97ef4582b6152494372fc083885f540c01f86d780e6f3e75a954af2190fdae9604e3f8ab32ab0292dc0d790bd2627e37b4b4885df"
        Qx = "633c2ee5630b62c9ce839efd4d485a6d35e8b9430d264ffe501d28dbace79123"
        Qy = "4b668a1a6d1a25b089f75c2bd8d8c6a9a14fe7b729f45a82565da2e866e2c490"
        R = "bf2111c93ec055a7eda90c106fce494fd866045634fd2aa28d6e018f9106994e"
        S = "86b0341208a0aa55edecfd272f49cb34408ce54b7febc1d0a1c2ce77ab6988f8"
        Result = "F (3 - S changed)"
    },
    vector! {
        Msg = "4bc2d9a898395b12701635f1048fbfd263ec115e4150532b034d59e625238f4ed32619744c612e35ac5a23bee8d5f5651641a492217d305e5051321c273647f14bc7c4afab518554e01c82d6fc1694c8bdbeb326bb607bcaf5436303bc09f64c02c6ec50de409a484f5237f7d34e2651ada7ec429ca3b99dd87c6015d2f4b342"
        Qx = "f78dce40d1cb8c4af2749bf22c6f8a9a470b1e41112796215dd017e57df1b38a"
        Qy = "61b29b0bc03dff7fa00613b4de1e2317cfbf2badd50dee3376c032a887c5b865"
        R = "4a96169a5dea36a2594011537ee0dc19e8f9f74e82c07434079447155a830152"
        S = "a204eaa4e97d7553a1521d9f6baadc0b6d6183ba0f385d8593d6ca83607c4d82"
        Result = "F (2 - R changed)"
    },
    vector! {
        Msg = "d3356a683417508a9b913643e6ceac1281ef583f428968f9d2b6540a189d7041c477da8d207d0529720f70dab6b0da8c2168837476c1c6b63b517ed3cad48ae331cf716ecf47a0f7d00b57073ac6a4749716d49d80c4d46261d38e2e34b4f43e0f20b280842f6e3ea34fefdddfb9fa2a040ffe915e8784cfdb29b3364a34ca62"
        Qx = "3fcc3b3e1b103fe435ac214c756bdaad309389e1c803e6d84bbbc27039fcf900"
        Qy = "7f09edd1ec87a6d36dc81c1528d52a62776e666c274415a9f441d6a8df6b9237"
        R = "1cac13f277354456ae67ab09b09e07eb1af2a2bf45108da70f5c8c6a4cbcd538"
        S = "5d83752e540525602ba7e6fee4d4263f3eda59e67df20aac79ca67e8899fed0d"
        Result = "F (3 - S changed)"
    },
    vector! {
        Msg = "d7f5da9f4cf9299b7f86c52b88364ce28fe9ada55dd551a1018790f9e1205e2405ac62429d65093f74ec35a16d9f195c993cd4eb8dc0aa0dabb70a503321d8a9649160d6b3d0a0854bb68c4c39693f592ef5dd478aa2432d0865d87d48b3aea9c7d7d114165c9200e4e8d7bd02a7895ec4418e6f2fed6b244bf66209039e98a9"
        Qx = "5ec702d43a67ada86efbfc136cf16d96078906954a3f1f9e440674cd907e4676"
        Qy = "05a62044fed8470dd4fca38d89d583ce36d50d28b66ab0b51922b21da92c56d9"
        R = "75f3037298f1457dba55743999976a1c2636b2b8ab2ed3df4736a6d2934acc83"
        S = "19d43ad168dda1bb8ac423f8f08876515234b3d841e57faef1b5ab27359b27ef"
        Result = "F (1 - Message changed)"
    },
    vector! {
        Msg = "68f4b444e1cc2025e8ff55e8046ead735e6e317082edf7ce65e83573501cb92c408c1c1c6c4fcca6b96ad34224f17b20be471cc9f4f97f0a5b7bfae9558bdb2ecb6e452bb743603724273d9e8d2ca22afdda35c8a371b28153d772303e4a25dc4f28e9a6dc9635331450f5af290dfa3431c3c08b91d5c97284361c03ec78f1bc"
        Qx = "f63afe99e1b5fc652782f86b59926af22e6072be93390fe41f541204f9c935d1"
        Qy = "f6e19ce5935e336183c21becf66596b8f559d2d02ee282aa87a7d6f936f7260c"
        R = "cef4831e4515c77ca062282614b54a11b7dc4057e6997685c2fbfa95b392bf72"
        S = "f20dc01bf38e1344ba675a22239d9893b3a3e33d9a403329a3d21650e9125b75"
        Result = "P (0 )"
    },
    vector! {
        Msg = "e75be05be0aaf70719b488b89aaae9008707ca528994461db7130c4368575a024bf0981c305d61265e8b97599ec35c03badd1256b80d6bf70547ad6089b983e3bcc3481828f3259e43e655e177fc423fd7e066bd3ed68d81df84f773c0f9e5f8bf4469960b8b4d7b2a372fd0edd3521f6be670908f2d90a343f416358ea70e7e"
        Qx = "6d11b09d2767cf8d275faee746c203486259f66dd2bfa3a65c39371a66b23385"
        Qy = "4eb05c73e05261e979182833f20311e5366f72f4b949665ff294f959375534c6"
        R = "15a697cdb614e11c0810e1e764cd501fcabc70874c957587bc4883d9438e177f"
        S = "7bf6244f92bc768063cecb5336c8eaacd23db930b28703560f241c7d93950dfd"
        Result = "F (2 - R changed)"
    },
    vector! {
        Msg = "0dc4a3eab66bd2e703a8fff566c34d466f9823ae42bd2104f61a6b051c0b017833fcef4d609d137ad97c209c80eebe252857aa7fafc35f16000a2bd4b4be0fa83b6e229eddfd180101f1f40d0453148053d8306833df64d59599b90194b55541d7f22dd589da9f7be519cbbb9db416c71bfe40ec090b5b7a600eec29bfd47306"
        Qx = "f3899caba038efb534c4cea0bd276814ffd80194473c903b81af11c8c05cb6e6"
        Qy = "6ea6b17402fcf2e8e737d11ffc7c2ed3b2d0bc3b8f271a381f4294cff62682c3"
        R = "57b99380452e1d37b133c49b9ba493dee8630940477ca3351a43d90b99871e6a"
        S = "df599c3a37105af3ecc159b3b685ccb3e151b7d5cf2d97147974ae71f466b615"
        Result = "F (3 - S changed)"
    },
    vector! {
        Msg = "d55e5e124a7217879ca986f285e22ac51940b35959bbf5543104b5547356fd1a0ec37c0a23209004a2ec5bcaf3335bc45e4dc990eacd29b2d9b5cf349c7ba67711356299bceab6f048df761c65f2988803133d6723a2820fefb2654cc7c5f032f833ba78a34d2878c6b0ba654ebe26b110c935abb56024bd5d0f09b367724c07"
        Qx = "1fd6f4b98d0755291e7a230e9f81ecf909e6350aadb08e42a3262ff19200fbd2"
        Qy = "5578fef79bc477acfb8ed0dc10c4f5809c14dc5492405b3792a7940650b305d7"
        R = "97a99e96e407b3ada2c2dcf9ceeeb984d9a4d0aa66ddf0a74ca23cabfb1566cc"
        S = "0ecac315dc199cfea3c15348c130924a1f787019fe4cd3ae47ca8b111268754a"
        Result = "F (1 - Message changed)"
    },
    vector! {
        Msg = "7753c03b4202cb38bc0190a9f931eb31858d705d92d650320ff449fc99167fb3770b764c8988f6b34ac5a3d507a10e0aff7f88293f6a22c7ed8a24248a52dc125e416e158833fc38af29199f8ca4931068d4ccaa87e299e95642068f68c208cb782df13908f950564743ed1692502bafafaff169dc8fe674fb5e4f3ffd578c35"
        Qx = "2dcbd8790cee552e9f18f2b3149a2252dcd58b99ca7dc9680b92c8c43aa33874"
        Qy = "5dbc8bb8813c8e019d80e19acdb0792f537980fecde93db621aaf1f6d0e6ee34"
        R = "2bdbd8b0d759595662cc10b10236136ef6ce429641f68cf6480f472fcc77bc9f"
        S = "7e7df0c8b86f7db06caf1610166f7b9c4c75447f991d5aaf4dea720c25985c8c"
        Result = "P (0 )"
    },
];

#[macro_export]
macro_rules! vector {
    (Msg = $msg:literal Qx = $x:literal Qy = $y:literal R = $r:literal S = $s:literal Result = $result:literal ) => {
        Vector::new($msg, $x, $y, $r, $s, $result)
    };
}

#[derive(Debug, Clone, defmt::Format)]
pub struct Vector {
    pub msg: [u8; 128],
    pub r: [u8; 32],
    pub s: [u8; 32],
    pub y: [u8; 32],
    pub x: [u8; 32],
    pub result: bool,
}

impl Vector {
    pub const fn new(msg: &str, x: &str, y: &str, r: &str, s: &str, result: &str) -> Self {
        Self {
            msg: decode_hex(msg),
            x: decode_hex(x),
            y: decode_hex(y),
            r: decode_hex(r),
            s: decode_hex(s),
            result: result.as_bytes()[0] == b'P',
        }
    }
}

const fn decode_hex<const N: usize>(hex: &str) -> [u8; N] {
    const fn digit(digit: u8) -> u8 {
        match digit {
            b'a'..=b'f' => digit - b'a' + 10,
            b'A'..=b'F' => digit - b'A' + 10,
            b'0'..=b'9' => digit - b'0',
            _ => panic!("Non-hex digit"),
        }
    }

    let hex = hex.as_bytes();
    assert!(hex.len() == N * 2, "Input and output length mismatch");

    let mut output = [0u8; N];
    let mut idx = 0;
    while idx < hex.len() {
        let upper = digit(hex[idx]);
        let lower = digit(hex[idx + 1]);

        output[idx / 2] = (upper << 4) | lower;

        idx += 2;
    }

    output
}

#[defmt_test::tests]
mod tests {
    use crate::{VECTORS_SHA256, VECTORS_SHA512};

    #[init]
    fn init() {
        let mut cp = defmt::unwrap!(cortex_m::peripheral::Peripherals::take());
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();
        cp.DWT.set_cycle_count(0);
    }

    #[test]
    fn sigver_sha256() {
        use sha2::Digest;

        for (idx, v) in VECTORS_SHA256.iter().enumerate() {
            let key = p256_cm4::VerifyingKey::from_parts(&v.x, &v.y).unwrap();
            let digest: [u8; 32] = sha2::Sha256::digest(&v.msg).into();
            let signature = p256_cm4::Signature::from_parts(&v.r, &v.s).unwrap();
            let start = cortex_m::peripheral::DWT::cycle_count();
            let output = key.verify_prehash(&digest, &signature);
            let end = cortex_m::peripheral::DWT::cycle_count();

            defmt::debug!(
                "Took approximately {} cycles to verify for vector {}",
                end - start,
                idx
            );

            defmt::assert_eq!(
                output,
                v.result,
                "Expected {}, got {} for vector #{} ({:X})",
                v.result,
                output,
                idx + 1,
                v
            );
        }
    }

    #[test]
    fn sigver_sha512() {
        use sha2::Digest;

        for (idx, v) in VECTORS_SHA512.iter().enumerate() {
            let key = p256_cm4::VerifyingKey::from_parts(&v.x, &v.y).unwrap();
            let digest: [u8; 64] = sha2::Sha512::digest(&v.msg).into();
            let signature = p256_cm4::Signature::from_parts(&v.r, &v.s).unwrap();
            let start = cortex_m::peripheral::DWT::cycle_count();
            let output = key.verify_prehash(digest[..32].try_into().unwrap(), &signature);
            let end = cortex_m::peripheral::DWT::cycle_count();

            defmt::debug!(
                "Took approximately {} cycles to verify for vector {}",
                end - start,
                idx
            );

            defmt::assert_eq!(
                output,
                v.result,
                "Expected {}, got {} for vector #{} ({:X})",
                v.result,
                output,
                idx + 1,
                v
            );
        }
    }
}
