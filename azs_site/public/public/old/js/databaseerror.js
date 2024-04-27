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
    if($("#stat_con").attr("xdata")=="0"){
        console.log("");
        check_end();
        location.reload();
        return;
    }else if($("#stat_con").attr("xdata")=="-1"){
        console.log("disconnect");
        $.ajax({
            type: "GET",
            url: "/api",
            
            dataType: "json",
            contentType : "application/json"
        });
    }
    settings_db_check();
}