pub struct TestCase<'a> {
    pub replacement_type: &'a str,
    pub file_path: &'a str,
    pub rx_patchs: &'a [RxPatch<'a>],
}

pub struct RxPatch<'a> {
    pub regex: &'a str,
    pub new_content: &'a str,
}

#[rustfmt::skip]
pub const DATAS:&[TestCase] = &[
    
    // inline > simple
    
    // TestCase { replacement_type: "inline", file_path: "00_inline/00_simple/00-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A(.*)", new_content: "DATA1" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/00_simple/01-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A(.*)", new_content: "DATA2" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/00_simple/02-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "(.*)A", new_content: "DATA2" },
    // ]},

    // // inline > medium

    // TestCase { replacement_type: "inline", file_path: "00_inline/01_medium/00-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A(.*)D", new_content: "DATA1" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/01_medium/01-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A(.*)D", new_content: "DATA2" },
    //     RxPatch{ regex: "A(.*)D", new_content: "" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/01_medium/02-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A\\s*(DATA1)\\s*D", new_content: "DATA2" },
    // ]},

    // // inline > advanced

    // TestCase { replacement_type: "inline", file_path: "00_inline/02_advanced/00-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A\\s*B\\s*C(.*)", new_content: "DATA1" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/02_advanced/01-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A\\s*B\\s*C(.*)", new_content: "DATA2" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/02_advanced/02-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A\\s*B\\s*C(.*)D", new_content: "DATA1" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/02_advanced/03-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A\\s*B\\s*C(.*)D", new_content: "DATA2" },
    // ]},
    // TestCase { replacement_type: "inline", file_path: "00_inline/02_advanced/04-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A\\s*B\\s*C(.*)D", new_content: "DATA2" },
    //     RxPatch{ regex: "^A\\s*B\\s*C(.*)D", new_content: "DATA2" },
    //     RxPatch{ regex: "^ +A\\s*B\\s*C(.*)D", new_content: "DATA2" },
    // ]},

    // block > toml

    TestCase { replacement_type: "block", file_path: "01_block/00_env/00-00.env", rx_patchs: &[
        RxPatch{ regex: r"PART2", new_content: "KEY1=VALUE1" },
    ]},

    // TestCase { replacement_type: "block", file_path: "01_block/00_env/01-00.env", rx_patchs: &[
    //     RxPatch{ regex: "# PART2", new_content: "KEY2=VALUE2\nKEY3=VALUE3" },
    // ]},

    // TestCase { replacement_type: "block", file_path: "01_block/00_simple/02-00.txt", rx_patchs: &[
    //     RxPatch{ regex: "A(.*)\n\n", new_content: "COUCOU" },
    // ]},
];
