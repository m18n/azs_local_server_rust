$(document).ready(function() {
    check_db();
    check_start();
    });
var myVar = true;

function check_start(){
    var interval = setInterval(function() {
    if (myVar) {
        check_db();
    } else {
        clearInterval(interval);
    }
    }, 1000);
}
function check_end(){
    myVar=false;
}
function check_db(){
    console.log("check");
    if($("#stat_con").attr("xdata")!="1"){
        check_end();
    }else{
        console.log("db check");
        settings_db_check();
    }
    
}