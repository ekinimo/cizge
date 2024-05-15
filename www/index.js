import {AdjList,Vertex} from "gracalc";

function drawGraph(ret){
    let elem = document.getElementById("img");
    console.log(elem);
    elem.innerHTML = ret.get_svg();
    
    Array.from(document.getElementById("nodes").getElementsByTagName("g")).forEach(n => {
        const [name,vertex1,vertex2] = n.id.split("--");
        if (name==="node"){
            n.addEventListener("click", () => {
                console.log("vertex " + vertex1 +" got clicked");
                vertexClicked(ret,vertex1);
            });}
        else if( name =="edge"){
            n.addEventListener("click", () => {
                console.log("edge " + vertex1 +" "+ vertex2 +" got clicked");
                edgeClicked(ret,vertex1,vertex2);});
        }else{
            console.log("something fishy going on");
        }
    });
}


let vertexClicked=(graph,vertex)=>{
    console.log(self);
    if (self.count)
        self.count +=1;
    else{ self.count = 1;}
    if (self.last_click && self.count && self.count == 2){
        //graph.add_edge(Vertex.new(self.last_click),Vertex.new(vertex));
        drawGraph(graph);
        self.count=0;
    }
    self.last_click =  vertex;
};
let edgeClicked=(graph,vertex1,vertex2)=>{
    graph.remove_edge_between(Vertex.new(vertex1),Vertex.new(vertex2));
    drawGraph(graph);
};

const errElem = document.createElement("textarea");
errElem.setAttribute("id", "ErrDiv");
errElem.setAttribute("cols", "80");
errElem.setAttribute("rows", "30");
errElem.setAttribute("readonly", "true");

const input_text = document.getElementById("text_area");
input_text.addEventListener("input", (event) => {
    try{
        let ret = AdjList.try_parse(event.target.value);
        drawGraph(ret);
        if (document.body.contains(errElem)){
            let errDiv = document.getElementById("ErrDiv");
            errDiv.remove();
        }
    }catch (errorMessage){
        if (document.body.contains(errElem)){
            const   errElem = document.getElementById("ErrDiv");
            errElem.textContent = errorMessage ;
        }else{
            errElem.textContent = errorMessage ;
            input_text.parentNode.appendChild(errElem);
        }
    }
});
