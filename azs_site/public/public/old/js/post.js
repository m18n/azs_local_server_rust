
function auth(){
    var json_input =$('#auth :input').serializeArray();
   
    var authdata = {
        id_user: parseInt(json_input[0]["value"]),
        password: json_input[1]["value"],
    };
    let formData=JSON.stringify(authdata);
    console.log(formData+"\n");
    $.ajax({
        type: "POST",
        url: "/api/db/auth",
        data: formData,
        success: function(data){
            if(data.hasOwnProperty("error")){
                if(data["error"]=="ConnectErr"){
                    document.location.href = '/settings/dbproperties';
                }else if(data["error"]=="RequestErr"){
                    document.location.href = '/settings/dberror';
                }
            }else{
                if(data["status"]==true){
                    document.location.href = '/view/userspace/old/main';
                }else{
                    $(".status").text("STATUS: wrong password");
                }
            }
        },
        dataType: "json",
        contentType : "application/json"
    });
}

function settings_db_check(){
    console.log("ajax");
    $.ajax({
        type: "GET",
        url: "/api/service/checkDbConnection",
        success: function(data){
            console.log("DATA: "+data);
            console.log("STATUS:"+data["status"]);
            if(data["status"]=="Connecting"){
                $("#stat_con").text("STATUS CONNECT: process");
                $("#stat_con").attr("xdata","1")
               
               // document.location.href="/main/settings/database";
            }else if(data["status"]=="Connected"){
                $("#stat_con").text("STATUS CONNECT: connect");
                $("#stat_con").attr("xdata","0")
                
            }else if(data["status"]=="Disconnected"){
                $("#stat_con").text("STATUS CONNECT: disconnect");
                $("#stat_con").attr("xdata","-1")
                
            }
        },
        error: function(data){  
            console.log("ERROR CHECK");
            //document.location.href="/settings/dberror";
        },
        dataType: "json",
        contentType : "application/json"
    });
    
}
function settings_db_error(){
    let json_input=$('#sett_azs').find(':input').serializeArray();
   
    
    var mysqlinfo = {
        ip: json_input[0]["value"],
        login: json_input[1]["value"],
        password: json_input[2]["value"],
        database:json_input[3]["value"],
        port:json_input[4]["value"]
    };
    let format=JSON.stringify(mysqlinfo);
    console.log(format+"\n");
    $.ajax({
        type: "POST",
        url: "/api/service/setDbProperties",
        data: format,
        success: function(data){
            if(data["status"]==true){

                document.location.href="/view/login";
            }else{
                console.log("Error send");
            }
        },
        
        dataType: "json",
        contentType : "application/json"
    });
   
}

function save_pump(json_object){
    console.log("JSON: "+json_object);
    $.ajax({
        type: "POST",
        url: "/api/db/userspace/admin/saveTrksPosition",
        contentType: "application/json; charset=utf-8",
        dataType: "json",
        data: json_object,
        success: function(data){
            if(data.hasOwnProperty("error")){
                if(data["error"]=="ConnectErr"){
                    document.location.href = '/settings/dbproperties';
                }else if(data["error"]=="RequestErr"){
                    document.location.href = '/settings/dberror';
                }
            }else{
                console.log(data)
            }
        },
        error: function(errMsg) {
            console.log(data)
        }
        
    });
}
