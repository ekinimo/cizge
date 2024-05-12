import {AdjList,Vertex} from "gracalc";

const input_text = document.getElementById("text_area");
input_text.addEventListener("input", (event) => {

    try{
     let ret = AdjList.try_parse(event.target.value);
    let elem = document.getElementById("img");
        console.log(elem);
        elem.innerHTML = ret.get_svg();

    }catch (err){
        let elem = document.getElementById("err");
        elem.textContent = err ;
        
    }
    
});
