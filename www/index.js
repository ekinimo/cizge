import {AdjList,Vertex} from "gracalc";

let vertexClicked=(vertex)=>{console.log(vertex);};
let edgeClicked=(vertex,vertex2)=>{console.log(vertex);};

const input_text = document.getElementById("text_area");
input_text.addEventListener("input", (event) => {

    try{
     let ret = AdjList.try_parse(event.target.value);
    let elem = document.getElementById("img");
        console.log(elem);
        elem.innerHTML = ret.get_svg();

        Array.from(document.getElementById("nodes").getElementsByTagName("g")).forEach(n => {
            const [name,vertex1,vertex2] = n.id.split("--");
            if (name==="node"){
            n.addEventListener("click", () => {
                console.log("vertex " + vertex1 +" got clicked");
                vertexClicked(vertex1);
            });}else if( name =="edge"){
                n.addEventListener("click", () => {
                    console.log("edge " + vertex1 +" "+ vertex2 +" got clicked");
                    edgeClicked(vertex1,vertex2);});
            }else{
                console.log("something fishy going on");
            }
            
        });
        //document.getElementById("nodes").getElementsById

        //let err = document.getElementById("err");
        //elem.textContent = "" ;

    }catch (err){
        let elem = document.getElementById("err");
        elem.textContent = err ;
        
    }
    
});
