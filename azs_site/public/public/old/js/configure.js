let col_item=0;
function swapBlocks(block1, block2) {
    var $block1 = $(block1);
    var $block2 = $(block2);
    $block1.insertBefore($block2); // insert block1 before block2
  }
function setpositiontank(id_tank,pos){
    let tanks=$(".item_tank").toArray();
    for(let i=0;i<tanks.length;i++){
        if($(tanks[i]).attr("valueid")==id_tank){
            swapBlocks(tanks[i],tanks[pos]);
            break;
        }
    }
}
function linesvisible(item){
    console.log("visible");
    $(".lines").removeClass("none");
   
    
    let iditem=$(item).attr("valueid");
    let jsontrk;
    for(let i=0;i<pumparray["pumps"].length;i++){
        if(pumparray["pumps"][i]["id_trk"]==iditem){
            jsontrk=pumparray["pumps"][i];
            break;
        }
    }
    let arrline=$(".item_line").toArray();  
    for(let i=0;i<jsontrk["pists"].length;i++){
        let id=jsontrk["pists"][i]["id_tank"];
        $(arrline[i]).removeClass("none");
        let itemarr=$(arrline[i]).find(".item_color");
        for(let j=0;j<itemarr.length;j++){
            $(itemarr[j]).attr("style","background-color:rgb("+jsontrk["pists"][i]["color"]+") !important;");
        }
        // $(arrline[i]).children(".item_color").attr("style","rgb("+jsontrk["pists"][i]["color"]+") !important");
        setpositiontank(id,i);
    }   
    let p=0;
    for(let i=0;i<jsontrk["pists"].length;i++){
        
        for(let j=0;j<jsontrk["pists"].length;j++){
            let l=0;
            if(jsontrk["pists"][j]["id_tank"]==jsontrk["pists"][i]["id_tank"]){
                l++;
                if(l>=2){
                    p++;   
                }
                
                
            }
        }
        
    }
    console.log("P: "+p); 
    
    let sizetanksintrk=jsontrk["pists"].length-p-1;
    console.log("size: "+ sizetanksintrk);
    let tanks=$(".item_tank").toArray();
    if(tanks.length>=0){
        $(tanks[0]).addClass("item_start");
        if(tanks.length>=sizetanksintrk){
            $(tanks[sizetanksintrk]).addClass("item_finish");
        }
    }
    console.log(jsontrk);
    // let jsontank;
    // for(let i=0;i<pumparray["tanks"].length;i++){
    //     if(pumparray["tanks"][i]["id_tank"]==iditem){
    //         jsontank=pumparray["tanks"][i];
    //         break;
    //     }
    // }
}
function linesunvisible(item){
    console.log("unvisible");       
    $(".lines").addClass("none");
    let tanks=$(".item_tank").toArray();
    for(let i=0;i<tanks.length;i++){
        $(tanks[i]).removeClass("item_start")
        $(tanks[i]).removeClass("item_finish")
    }
    let item_lines=$(".item_line").toArray();
    for(let i=0;i<item_lines.length;i++){
        $(item_lines[i]).addClass("none");
    }
    for(let i=0;i<pumparray["tanks"].length;i++){
        setpositiontank(pumparray["tanks"][i]["id_tank"],i);
    }
}
$(".item_tank").click(function(){
    $(".settings_tank").addClass("visible");
    $(".black").addClass("visible");
});
$("#cross").click(function(){
    $(".settings_tank").removeClass("visible");
    $(".black").removeClass("visible");
})
$(".lable_trk").click(function(){
    let item=$(this).parents(".item_trk");
    let viewport=$(item).parents(".viewport");
    let gui=$(item).find(".guitrk")
    let index=$(".viewport .item_trk").index(item);
    let px=index*35;
    console.log("PX: "+px);
    if($(gui).hasClass("visible")){
        linesunvisible(item);
        
        $(gui).removeClass("visible");
        $(viewport).css({"transform":"translateY(0px)"});
    }else{
        linesunvisible(item);
            $(viewport).children('.item_trk').each(function(index) {
                $(this).find(".guitrk").removeClass("visible");
              });
       
        linesvisible(item);
        $(gui).addClass("visible");
        $(viewport).css({"transform":"translateY(-"+px+"px)"});
        
    }
    
});

$(document).ready(function () {
    col_item=$('.viewport .item_trk').length;   
    console.log(col_item);
    console.log($(".viewport").child)
    
    // $( ".item_trk" ).each(function( index ) {
    //     console.log( index + ": " + $( this ).attr("value") );
    //   });
        // let jsonObject=JSON.parse($(".item_trk").attr("value"));
        // console.log("JSON: "+jsonObject);
});