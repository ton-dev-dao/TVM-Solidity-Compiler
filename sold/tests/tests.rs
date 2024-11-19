/*
 * Copyright (C) ton.dev. All Rights Reserved.
 *
 * Licensed under the SOFTWARE EVALUATION License (the "License"); you may not use
 * this file except in compliance with the License.
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the  GNU General Public License for more details at: https://www.gnu.org/licenses/gpl-3.0.html
 */

use assert_cmd::Command;
use predicates::prelude::*;
use sold_lib::ERROR_MSG_NO_OUTPUT;

type Status = Result<(), Box<dyn std::error::Error>>;
const BIN_NAME: &str = "sold";

fn remove_all_outputs(name: &str) -> Status {
    std::fs::remove_file(format!("tests/{}.abi.json", name))?;
    std::fs::remove_file(format!("tests/{}.code", name))?;
    std::fs::remove_file(format!("tests/{}.debug.json", name))?;
    std::fs::remove_file(format!("tests/{}.tvc", name))?;
    Ok(())
}

#[test]
fn test_trivial() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/Trivial.sol")
        .arg("--output-dir")
        .arg("tests")
        .assert()
        .success();

    remove_all_outputs("Trivial")?;
    Ok(())
}

#[test]
fn test_combined() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/Combined.sol")
        .arg("--output-dir")
        .arg("tests")
        .assert()
        .success();

    remove_all_outputs("Combined")?;
    Ok(())
}

#[test]
fn test_multi() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/Multi.sol")
        .arg("--output-dir")
        .arg("tests")
        .arg("--contract")
        .arg("Contract1")
        .assert()
        .success();

    remove_all_outputs("Multi")?;
    Ok(())
}

#[test]
fn test_abi_json() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/AbiJson.sol")
        .arg("--output-dir")
        .arg("tests")
        .arg("--abi-json")
        .arg("--contract")
        .arg("Contract")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    std::fs::remove_file("tests/AbiJson.abi.json")?;
    Ok(())
}

#[test]
fn test_library() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/Library.sol")
        .arg("--output-dir")
        .arg("tests")
        .assert()
        .success()
        .stderr(predicate::str::contains(ERROR_MSG_NO_OUTPUT));

    Ok(())
}

#[test]
fn test_abstract() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/Abstract.sol")
        .arg("--output-dir")
        .arg("tests")
        .arg("--abi-json")
        .assert()
        .success()
        .stdout(predicate::str::is_empty());

    std::fs::remove_file("tests/Abstract.abi.json")?;
    Ok(())
}

#[test]
fn test_error_reporting() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/ErrorReporting.sol")
        .arg("--output-dir")
        .arg("tests")
        .assert()
        .failure()
        .stderr(predicate::str::contains("Compilation failed"));

    Ok(())
}

#[test]
fn test_cycle() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/CycleA.sol")
        .arg("--output-dir")
        .arg("tests")
        .arg("--base-path")
        .arg("tests")
        .assert()
        .success();

    remove_all_outputs("CycleA")?;
    Ok(())
}

#[test]
fn test_private_function_ids() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/FunctionId.sol")
        .arg("--private-function-ids")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"[
  {
    "id": 7460,
    "scope": "C",
    "sign": "f(int19,uint256,uint256)"
  },
  {
    "id": 7504,
    "scope": "C",
    "sign": "add(uint256,uint256)"
  },
  {
    "id": 10141,
    "scope": "C",
    "sign": "sub(uint256,uint256)"
  },
  {
    "id": 10143,
    "scope": "Math",
    "sign": "mul(uint256,uint256)"
  }
"#,
        ));
    Ok(())
}

#[test]
fn test_remapping() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/ImportRemote.sol")
        .arg("--output-dir")
        .arg("tests")
        .arg("github.com/ton-dev-dao/debots/=tests/remote/")
        .assert()
        .success();

    remove_all_outputs("ImportRemote")?;
    Ok(())
}

#[test]
fn test_userdoc_devdoc() -> Status {
    Command::cargo_bin(BIN_NAME)?
        .arg("tests/Trivial.sol")
        .arg("--userdoc")
        .arg("--devdoc")
        .assert()
        .success()
        .stdout(predicate::str::contains(
            r#"Developer Documentation
{
  "kind": "dev",
  "methods": {},
  "version": 1
}
User Documentation
{
  "kind": "user",
  "methods": {},
  "version": 1
}
"#,
        ));
    Ok(())
}
