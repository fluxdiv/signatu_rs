#[path = "test_types.rs"]
mod test_types;
use test_types::{
    test, ArgType, TestParams, AddArgs, UpdateArgs, ListArgs
};

macro_rules! tinfo {
    () => {
        format!(
            // "\x1b[31m|  File: {}    Line: {}  |\x1b[0m",
            "|  File: {}  Line: {}  |",
            file!(),
            line!(),
        )
    };
}


#[test]
fn get_bin_name() {
    match std::env::current_exe() {
        Ok(exe_path) => {
            assert!(false, "Path of this executable is: {}", exe_path.display());
            // println!("Path of this executable is: {}", exe_path.display());
        },
        Err(e) => {
            assert!(false, "failed to get current exe path: {e}");
            // println!("failed to get current exe path: {e}");
        }
    };
}

pub fn test_add_new(config_path: &str) {
    println!("=====================================================");
    println!("=========================================== add-new");

    let mut config_arg: [&str; 2] = ["--config-path", config_path];
    let mut args: Vec<[&str; 2]> = Vec::with_capacity(6);
    
    //------------------------------------
    //-------- Should pass cases ---------

    // basic adding
    args.push(["-U", "some user"]);
    args.push(["-E", "some_email@x.com"]);
    let add_args = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&add_args);
    let p1 = TestParams {
        ctx: String::from("a1: adding a new identity, 1 username 1 email"),
        verbose: false,
        args: a1
    };
    test(p1);

    // assert the added key is findable
    let list_args = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("add new key")
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: String::from("Checking added key is findable via list by ID"),
        verbose: false,
        args: la1,
    };
    test(lap1);

    // ---

    // assert can add with different key but duplicate username/email
    let ad2 = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "some other key"]),
        args: Some(args.clone())
    };

    let a2 = ArgType::ShouldPass(&ad2);
    let p2 = TestParams {
        ctx: String::from("a2: adding with new key but identical username/email"),
        verbose: false,
        args: a2
    };
    test(p2);

    // assert the added key is findable
    let list_args2 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("some other key")
    };
    let la2 = ArgType::ShouldPass(&list_args2);
    let lap2 = TestParams {
        ctx: String::from("Checking added key is findable via list by ID"),
        verbose: false,
        args: la2,
    };
    test(lap2);


    // adding with multiple usernames & emails
    let ad3 = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "some third key"]),
        args: Some(args.clone())
    };

    let a3 = ArgType::ShouldPass(&ad3);
    let p3 = TestParams {
        ctx: String::from("a2.1: adding with multiple usernames/emails"),
        verbose: false,
        args: a3
    };
    test(p3);

    // assert the added key is findable
    let list_args3 = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("some third key")
    };
    let la3 = ArgType::ShouldPass(&list_args3);
    let lap3 = TestParams {
        ctx: String::from("Checking added key is findable via list by ID"),
        verbose: false,
        args: la3,
    };
    test(lap3);


    //------------------------------------
    //-------- Should fail cases ---------
    
    // try adding with existing key
    let a3 = ArgType::ShouldFail(&add_args);
    let p3 = TestParams {
        ctx: String::from("a3: trying to add same key again"),
        verbose: false,
        args: a3
    };
    test(p3);

    // try adding with no --identity flag at all
    let ad4 = AddArgs {
        config_path: Some(config_arg),
        identity: None,
        args: Some(args.clone())
    };
    let a4 = ArgType::ShouldFail(&ad4);
    let p4 = TestParams {
        ctx: String::from("a4: try adding with no --identity flag"),
        verbose: false,
        args: a4
    };
    test(p4);

    // try adding with empty --identity flag
    let ad5 = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", ""]),
        args: Some(args.clone())
    };
    let a5 = ArgType::ShouldFail(&ad5);
    let p5 = TestParams {
        ctx: String::from("a5: try adding with empty --identity flag"),
        verbose: false,
        args: a5
    };
    test(p5);

    // adding requires both 1 email && 1 username, try adding with only 1 && neither
    // try adding with no email
    args.clear();
    args.push(["-U", "some user"]);
    // args.push(["-E", "some_email@x.com"]);
    let ad6 = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "a6"]),
        args: Some(args.clone())
    };
    let a6 = ArgType::ShouldFail(&ad6);
    let p6 = TestParams {
        ctx: String::from("a6: try adding with no email"),
        verbose: false,
        args: a6
    };
    test(p6);

    // try adding with no username
    args.clear();
    args.push(["-E", "some_email@x.com"]);
    let ad7 = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "a7"]),
        args: Some(args.clone())
    };
    let a7 = ArgType::ShouldFail(&ad7);
    let p7 = TestParams {
        ctx: String::from("a7: try adding with no username"),
        verbose: false,
        args: a7
    };
    test(p7);

    // try adding with no email or username
    args.clear();
    let ad8 = AddArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "a8"]),
        args: Some(args.clone())
    };
    let a8 = ArgType::ShouldFail(&ad8);
    let p8 = TestParams {
        ctx: String::from("a8: try adding with no email or username"),
        verbose: false,
        args: a8
    };
    test(p8);

}

pub fn test_updates(config_path: &str) {
    println!("=====================================================");
    println!("=========================================== update");

    let mut config_arg: [&str; 2] = ["--config-path", config_path];
    let mut args: Vec<[&str; 2]> = Vec::with_capacity(6);
    
    //------------------------------------
    //-------- Should pass cases ---------

    // updating an added key from test_add_new (--identity add_new_key)
    // add a username & email
    args.push(["--au", "somebody"]);
    args.push(["--ae", "somebody@y.com"]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} updating added key with 1 uname + 1 email", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);

    // assert findable by new email
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-E", "somebody@y.com"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check updated identity findable by new email", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);

    // assert findable by new username
    let list_args2 = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-U", "somebody"]]),
    };
    let la2 = ArgType::ShouldPass(&list_args2);
    let lap2 = TestParams {
        ctx: format!("{} Check updated identity findable by new username", tinfo!()),
        verbose: false,
        args: la2,
    };
    test(lap2);


    // remove added email
    args.clear();
    args.push(["--re", "somebody@y.com"]);
    let uargs2 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a2 = ArgType::ShouldPass(&uargs2);
    let p2 = TestParams {
        ctx: format!("{} removing 1 email", tinfo!()),
        verbose: false,
        args: a2
    };
    test(p2);

    // assert not findable by removed email
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-E", "somebody@y.com"]]),
    };
    let la2 = ArgType::ShouldPass(&list_args);
    let lap2 = TestParams {
        ctx: format!("{} Check updated identity not findable by removed email", tinfo!()),
        verbose: false,
        args: la2,
    };
    let out = test(lap2);
    assert!(out.is_some_and(|stdout| {
        let s = String::from_utf8_lossy(&stdout.stdout).trim().to_string();
        s == String::from("No matching identities found")
    }));

    // remove added username
    args.clear();
    args.push(["--ru", "somebody"]);
    let uargs3 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a3 = ArgType::ShouldPass(&uargs3);
    let p3 = TestParams {
        ctx: format!("{} removing 1 username", tinfo!()),
        verbose: false,
        args: a3
    };
    test(p3);

    // assert not findable by removed username
    let la3 = ArgType::ShouldPass(&list_args2);
    let lap3 = TestParams {
        ctx: format!("{} Check updated identity not findable by removed username", tinfo!()),
        verbose: false,
        args: la3,
    };

    let out = test(lap3);
    assert!(out.is_some_and(|stdout| {
        let s = String::from_utf8_lossy(&stdout.stdout).trim().to_string();
        s == String::from("No matching identities found")
    }));

    // --------------------
    // remove all
    // -- emails
    // add email
    args.clear();
    args.push(["--ae", "somebody@y.com"]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} updating added key with 1 email", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);
    // assert findable by new email
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-E", "somebody@y.com"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check updated identity findable by new email", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);

    // remove all emails
    args.clear();
    args.push(["--remove-all-emails", ""]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };
    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} removing all emails", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);
    // assert not findable by removed email
    let la2 = ArgType::ShouldPass(&list_args);
    let lap2 = TestParams {
        ctx: format!("{} Check updated identity not findable by removed email", tinfo!()),
        verbose: false,
        args: la2,
    };
    let out = test(lap2);
    assert!(out.is_some_and(|stdout| {
        let s = String::from_utf8_lossy(&stdout.stdout).trim().to_string();
        s == String::from("No matching identities found")
    }));
    // assert adding works after clearing all emails
    args.clear();
    args.push(["--ae", "somebody@y.com"]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} updating added key with 1 email", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);
    // assert findable by new email
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-E", "somebody@y.com"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check updated identity findable by new email", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);


    // -- usernames remove all
    // add username
    args.clear();
    args.push(["--au", "somebody"]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} updating added key with 1 username", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);
    // assert findable by new username
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-U", "somebody"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check updated identity findable by new username", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);

    // remove all usernames
    args.clear();
    args.push(["--remove-all-usernames", ""]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };
    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} removing all usernames", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);
    // assert not findable by removed username
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-U", "somebody"]]),
    };
    let la2 = ArgType::ShouldPass(&list_args);
    let lap2 = TestParams {
        ctx: format!("{} Check updated identity not findable by removed username", tinfo!()),
        verbose: false,
        args: la2,
    };
    let out = test(lap2);
    assert!(out.is_some_and(|stdout| {
        let s = String::from_utf8_lossy(&stdout.stdout).trim().to_string();
        s == String::from("No matching identities found")
    }));

    // assert adding works after clearing all usernames
    args.clear();
    args.push(["--au", "somebody"]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} updating added key with 1 username", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);
    // assert findable by new username
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-U", "somebody"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check updated identity findable by new username", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);


    // changing identity
    args.clear();
    // the new key
    args.push(["--change-identity", "IDENTITY CHANGE"]);
    let uargs1 = UpdateArgs {
        config_path: Some(config_arg),
        identity: Some(["--identity", "add new key"]),
        args: Some(args.clone())
    };

    let a1 = ArgType::ShouldPass(&uargs1);
    let p1 = TestParams {
        ctx: format!("{} updating identity", tinfo!()),
        verbose: false,
        args: a1
    };
    test(p1);

    // assert findable by new ID
    let list_args = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("IDENTITY CHANGE")
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} checking updated key is findable via list by ID", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);
    // assert old key gone
    let list_args = ListArgs::ByID {
        config_path: Some(config_path),
        verbose: true,
        id: Some("add new key")
    };
    let la1 = ArgType::ShouldFail(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} checking old key is not findable via list by ID", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);
    // assert still findable after key change
    // assert findable by username
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-U", "somebody"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check new identity key findable by username", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);

    // assert findable by email
    let list_args = ListArgs::Find {
        config_path: Some(config_path),
        verbose: true,
        args: Some(vec![["-E", "somebody@y.com"]]),
    };
    let la1 = ArgType::ShouldPass(&list_args);
    let lap1 = TestParams {
        ctx: format!("{} Check new identity key findable by email", tinfo!()),
        verbose: false,
        args: la1,
    };
    test(lap1);



    //
    //
    //
    //
    //
    //
    // // basic adding
    // args.push(["-U", "some user"]);
    // args.push(["-E", "some_email@x.com"]);
    // let add_args = AddArgs {
    //     config_path: Some(config_arg),
    //     identity: Some(["--identity", "add new key"]),
    //     args: Some(args.clone())
    // };
    //
    // let a1 = ArgType::ShouldPass(&add_args);
    // let p1 = TestParams {
    //     ctx: String::from("a1: adding a new identity, 1 username 1 email"),
    //     verbose: false,
    //     args: a1
    // };
    // test(p1);
    //
    // // assert the added key is findable
    // let list_args = ListArgs::ByID {
    //     config_path: Some(config_path),
    //     verbose: true,
    //     id: Some("add new key")
    // };
    // let la1 = ArgType::ShouldPass(&list_args);
    // let lap1 = TestParams {
    //     ctx: String::from("Checking added key is findable via list by ID"),
    //     verbose: false,
    //     args: la1,
    // };
    // test(lap1);
    //
    // ---


}


