function SaveTovare(){
    let sett=$(".settings_tovar");
    let name=sett.find("#name").val();
    let name_p=sett.find("#name_p").val();
    let name_p_f=sett.find("#name_p_f").val();
    let nd_code=sett.find("#nd_code").val();
    let wog_code=sett.find("#wog_code").val();
    let id=sett.attr("id");
    let stringsave="id:"+id+"\r"+"name:"+name+"\r"+"name_p:"+name_p+"\r"+"name_p_f:"+name_p_f+"\r"+"nd_code:"+nd_code+"\r"+"wog_code:"+wog_code+"\r";
    SaveTovar(stringsave);
}
function SaveAzs(){
    let sett=$("#sett_azs");
    let host=sett.find("#host").val();
    let user=sett.find("#user").val();
    let database=sett.find("#database").val();
    let password=sett.find("#pass").val();
    let stringsave="host:"+host+"\r"+"user:"+user+"\r"+"password:"+password+"\r"+"database:"+database+"\r";
    SaveAZS(stringsave);
}