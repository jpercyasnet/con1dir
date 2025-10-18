use std::path::Path;
use std::process::Command as stdCommand;
pub fn copyit (dir_value: String, outdir_value: String, mergescrol_value: String) -> (u32, String) {
     let mut errcode:u32 = 0;
     println!("got into copyit async");
     let mut errstring  = " ".to_string();
     let mut bolok = true;
     let mut numrow = 0;
     let mut numprocess = 0;
     let mergelistvec: Vec<&str> = mergescrol_value[0..].split("\n").collect();
     let mut lenmg1 = mergelistvec.len();
     lenmg1 = lenmg1 -1;
     for indl in 0..lenmg1 {
          let str_cur_dirfrom = dir_value.clone();
          let linestr = mergelistvec[indl];
          let lineparse: Vec<&str> = linestr[0..].split(" | ").collect();
          let filefromx = lineparse[1].to_string();
          let fullfrom = str_cur_dirfrom.clone() + "/" + &filefromx;
          if !Path::new(&fullfrom).exists() {
              errcode = 1;
              errstring = format!("********* convert Copy: ERROR {} does not exist **********",fullfrom);
              bolok = false;
              break;
          }
          let str_cur_dirout = outdir_value.clone();
          let fileprex = lineparse[2].to_string();
          let filetox = lineparse[3].to_string();
          let fullto = str_cur_dirout.clone() + "/" + &fileprex + "_" + &filetox;
          if Path::new(&fullto).exists() {
              errstring = format!("********* convert Copy: ERROR {} already exists **********", fullto);
              errcode = 2;
              bolok = false;
              break;
          }
          if numprocess < 4 {
              let mut spcmd = stdCommand::new("cp");
              spcmd.arg("-p");
              spcmd.arg(&fullfrom);
              spcmd.arg(&fullto);
              match spcmd.spawn() {
                  Ok(mut _child) => {
                         println!("{:?} spawn ok", spcmd);
//                  Ok(mut child) => match child.wait() {
//                                Ok(exit_status) => {
//                                    if !exit_status.success() {
//                                        println!("{cmd} failed with {exit_status}");
//                                    } else {
//                                        println!("{cmd} done.");
//                                    }
                  }
                  Err(err) => {
                         println!("{:?} error '{}'", spcmd, err)
                  }
              }
//                           .spawn()
//                           .expect("failed to execute process");
              numprocess = numprocess + 1;
          } else {
              let mut outcmd = stdCommand::new("cp");
              outcmd.arg("-p");
              outcmd.arg(&fullfrom);
              outcmd.arg(&fullto);
              match outcmd.output() {
                  Ok(_) => {
                         println!("{:?} output ok", outcmd);
//                  Ok(mut child) => match child.wait() {
//                                Ok(exit_status) => {
//                                    if !exit_status.success() {
//                                        println!("{cmd} failed with {exit_status}");
//                                    } else {
//                                        println!("{cmd} done.");
//                                    }
                  }
                  Err(err) => {
                         println!("{:?} error '{}'", outcmd, err)
                  }
              }
//                                         .output()
//                                         .expect("failed to execute process");
              numprocess = 0;
          }

          numrow = numrow + 1;
     }
     if bolok {
         errstring = format!("convert copy copied {} files", lenmg1);
     }
     (errcode, errstring)
}
