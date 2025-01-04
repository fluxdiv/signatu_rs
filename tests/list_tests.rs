#[path = "test_types.rs"]
mod test_types;
use test_types::{
    test, ListArgs, ArgType, TestParams
};

pub fn test_list_all(config_path: &str) {
    println!("=====================================================");
    println!("=========================================== list-all");
    // println!("=====================================================");
    //------------------------------------
    //-------- Should pass cases ---------
    let arg1 = ListArgs::All {
        config_path: Some(config_path),
        verbose: true
    };
    let a1 = ArgType::ShouldPass(&arg1);
    let p1 = TestParams {
        ctx: String::from("a1: verbose"),
        verbose: false,
        args: a1
    };
    test(p1);

    let arg2 = ListArgs::All {
        config_path: Some(config_path),
        verbose: false
    };
    let a2 = ArgType::ShouldPass(&arg2);
    let p2 = TestParams {
        ctx: String::from("a2: not verbose"),
        verbose: false,
        args: a2
    };
    test(p2);

    //------------------------------------
    //-------- Should fail cases ---------
    let a3 = ArgType::ShouldFail(&ListArgs::All {
        config_path: Some("./doesntexist.sigrs"),
        verbose: true
    });
    let p3 = TestParams {
        ctx: String::from("p3: non existent config_path + verbose"),
        verbose: false,
        args: a3
    };
    test(p3);

    let a4 = ArgType::ShouldFail(&ListArgs::All {
        config_path: Some("./doesntexist.sigrs"),
        verbose: false
    });
    let p4 = TestParams {
        ctx: String::from("p4: non existent config_path + not verbose"),
        verbose: false,
        args: a4
    };
    test(p4);

    let a5 = ArgType::ShouldFail(&ListArgs::All {
        config_path: Some(""),
        verbose: true
    });
    let p5 = TestParams {
        ctx: String::from("p5: empty string as config_path + verbose"),
        verbose: false,
        args: a5
    };
    test(p5);

    let a6 = ArgType::ShouldFail(&ListArgs::All {
        config_path: Some(""),
        verbose: false
    });
    let p6 = TestParams {
        ctx: String::from("p6: empty string as config_path + not verbose"),
        verbose: false,
        args: a6
    };
    test(p6);
}

pub fn test_list_find(config_path: &str) {
    println!("=====================================================");
    println!("=========================================== list-find");
    // println!("=====================================================");
    //------------------------------------
    //-------- Should pass cases ---------
    let mut args: Vec<[&str; 2]> = Vec::with_capacity(6);
    args.push(["-U", "uname 2AA"]);

    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a1 = ArgType::ShouldPass(&list_args);
    let p1 = TestParams {
        ctx: String::from("a1: single username match"),
        verbose: false,
        args: a1
    };
    test(p1);


    args.push(["-U", "uname 2B"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a2 = ArgType::ShouldPass(&list_args);
    let p2 = TestParams {
        ctx: String::from("a1: double username match on same identity"),
        verbose: false,
        args: a2
    };
    test(p2);


    args.push(["-U", "uname 2CC"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a3 = ArgType::ShouldPass(&list_args);
    let p3 = TestParams {
        ctx: String::from("a3: triple username match on same identity"),
        verbose: false,
        args: a3
    };
    test(p3);


    args.clear();
    args.push(["-E", "3A@x.com"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a4 = ArgType::ShouldPass(&list_args);
    let p4 = TestParams {
        ctx: String::from("a4: single email match"),
        verbose: false,
        args: a4
    };
    test(p4);


    args.push(["-E", "3B@x.com"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a5 = ArgType::ShouldPass(&list_args);
    let p5 = TestParams {
        ctx: String::from("a5: double email match on same identity"),
        verbose: false,
        args: a5
    };
    test(p5);


    args.push(["-E", "3C@x.com"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a6 = ArgType::ShouldPass(&list_args);
    let p6 = TestParams {
        ctx: String::from("a6: triple email match on same identity"),
        verbose: false,
        args: a6
    };
    test(p6);


    args.clear();
    args.push(["-U", "uname 3A"]);
    args.push(["-E", "3B@x.com"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a7 = ArgType::ShouldPass(&list_args);
    let p7 = TestParams {
        ctx: String::from("a7: single username + single email match on same identity"),
        verbose: false,
        args: a7
    };
    test(p7);


    args.push(["-U", "uname 2AA"]);
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: false,
        args: Some(args.clone())
    };
    let a8 = ArgType::ShouldPass(&list_args);
    let p8 = TestParams {
        ctx: String::from("a8: single uname&email same identity ++ single uname match"),
        verbose: false,
        args: a8
    };
    test(p8);
}

pub fn test_list_by_id(config_path: &str) {
    println!("=====================================================");
    println!("=========================================== list-by-id");
    // println!("=====================================================");
    //------------------------------------
    //-------- Should pass cases ---------
    let arg1 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("key 2")
    };
    let a1 = ArgType::ShouldPass(&arg1);
    let p1 = TestParams {
        ctx: String::from("a1: existing key + valid config_path + verbose"),
        verbose: false,
        args: a1
    };
    test(p1);

    let arg2 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: false,
        id: Some("key 2")
    };
    let a2 = ArgType::ShouldPass(&arg2);
    let p2 = TestParams {
        ctx: String::from("a2: existing key + valid config_path + not verbose"),
        verbose: false,
        args: a2
    };
    test(p2);


    //------------------------------------
    //-------- Should fail cases ---------
    let arg3 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("doesnt exist")
    };
    let a3 = ArgType::ShouldFail(&arg3);
    let p3 = TestParams {
        ctx: String::from("a3: nonexist key + verbose"),
        verbose: false,
        args: a3
    };
    test(p3);

    let arg4 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: false,
        id: Some("doesnt exist")
    };
    let a4 = ArgType::ShouldFail(&arg4);
    let p4 = TestParams {
        ctx: String::from("a4: nonexist key + not verbose"),
        verbose: false,
        args: a4
    };
    test(p4);

    let arg5 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: false,
        id: Some("\n")
    };
    let a5 = ArgType::ShouldFail(&arg5);
    let p5 = TestParams {
        ctx: String::from("a5: newline as key"),
        verbose: false,
        args: a5
    };
    test(p5);

    let arg6 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: false,
        id: Some("")
    };
    let a6 = ArgType::ShouldFail(&arg6);
    let p6 = TestParams {
        ctx: String::from("a6: empty string as key"),
        verbose: false,
        args: a6
    };
    test(p6);

    let arg7 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: false,
        id: None
    };
    let a7 = ArgType::ShouldFail(&arg7);
    let p7 = TestParams {
        ctx: String::from("a7: no key provided"),
        verbose: false,
        args: a7
    };
    test(p7);
}

