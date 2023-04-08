
pub const PRTL_SHORTHAND_SCRIPT: &str = r#"
function p() {
   if [ $1 = "get" ]; then 
     cd $(prtl "$@")
   elif [ $1 = "set" ]; then
     $(prtl $@)
   else
     echo Global options will not work. Type \'prtl -h\' for more info.
     echo \'p\' short-hand only supports \'get\' and \'set\' commands. 
   fi
}
"#;