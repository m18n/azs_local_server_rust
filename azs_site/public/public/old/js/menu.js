$( ".item" ).click(function() {
    $(".mark").addClass("disable");
    console.log("CLICK");
    let s;
    if(s=$(this).children(".mark")){
        s.removeClass("disable");
    }
  });
  const StatusMenu={OPENING:1,CLOSING:2,STOP:3};

  function ItemVisible(items,menu){
    console.log("VISIBLE: "+menu.status);
      if(menu.status==StatusMenu.OPENING){
          items.parentNode.style.overflowX='visible';
          items.parentNode.style.overflowY='visible';
          menu.status=StatusMenu.STOP;
      }
  }
  function ShowMenu(obj){
      let menu=$(obj).parents(".menu");
      menu[0].status=StatusMenu.OPENING;
      $(menu).addClass("up_layer");
      $(menu).find(".slider").first().addClass("resize");
      console.log("SHOW MENU");
      obj.style.animationName='border_rev';
      obj.style.animationDuration='1s';
      obj.style.animationDirection='normal';
      obj.style.borderRadius='0px';
      let items=obj.parentNode.querySelector(".items");
      items.style.left='0px';
      items.style.animationName='menuslider';
      items.style.animationDuration='1s';
      items.style.animationDirection='normal';
      
      setTimeout(ItemVisible, 1000,items,menu[0]);
      
    
  }
  // function ShowMiniMenu(obj){
  //     console.log("SHOW MINI MENU");
  //     let items=obj.parentNode.querySelector(".min_menu");
      
  // }
  function hideslider(obj){
    console.log("HIDE SLIDER: "+obj.status);
    if(obj.status==StatusMenu.CLOSING){
        $(obj).find(".slider").first().removeClass("resize");
    }
   
  }
  function HideMenu(obj){
    let object=obj;
    object.status=StatusMenu.CLOSING;
    $(obj).removeClass("up_layer")
    //$(obj).find(".slider").first().removeClass("resize");
      let listmark=$(obj).find(".mark");
      for(let i=0;i<listmark.length;i++){
          $(listmark[i]).addClass("disable");
      }
      console.log("HID MENU");
      
     
      //obj.style.overflowX='hidden';
      //obj.style.overflowY='hidden';
      obj=obj.querySelector(".items");
      $(obj).parent().css("overflow-x","hidden");
      $(obj).parent().css("overflow-y","hidden");
      obj.style.left='-100%';
      obj.style.animationName='menuslider_rev';
      obj.style.animationDuration='0.8s';
      obj.style.animationDirection='normal';
      let items=obj.parentNode.parentNode.querySelector("#min_menu");
      items.style.borderRadius='15px';
      items.style.animationName='border';
      items.style.animationDuration='0.8s';
      items.style.animationDirection='normal';
      setTimeout(hideslider,700, object);
  
  }
 