function Hide(obj){
    console.log("HIDE");
    obj.style.display='none';
}

function OpenGlav(){
    console.log("ITEM G");
    let obj= $(this).parent().get(0);
    if(obj.cur==undefined){
         obj.cur=1;
         obj.bc="grey";
         obj.st=false;
    }
    if(obj.st==false){ 
     console.log("OBJECT CUR "+obj.cur);
     let objd=$(this).parent().find(".item_opt")[obj.cur];
     obj.bc=$(objd).css("background-color");
    console.log("BC: "+obj.bc);
     $(objd).css("background-color","grey");
     
     $(this).parent().find(".item_opt").removeClass("none");
     obj.st=true;
    }
}
$(".item_g").on("click",OpenGlav);
function OpenOpt(){
    let obj=$(this).parent().get(0);
    console.log("index: "+$(this).index());
    let parent=$(this).parent();
    let b_item=parent.find(".item_opt")[obj.cur];
   $(b_item).css("background-color",obj.bc);
    obj.cur=$(this).index();
    obj.st=false;
    let item=parent.find(".item_g");
    item.css("background-color",$(this).css("background-color"));
    item.text($(this).text());
    parent.find(".item_o").addClass("none");
    
}
$(".item_o").on("click",OpenOpt);