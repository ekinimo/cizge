
export function getElemWidth(x){
    const   hiddenDiv = document.getElementById("hidden_div");
    if (hiddenDiv) {
        hiddenDiv.innerHTML = x;
        const foreign = document.getElementById("math");
        const width = foreign.scrollWidth;
        console.log("check width");
        console.log(width);
        foreign.remove();
        //elem.remove();
        //console.log("removed width");
        return width;
    }
    const elem = document.createElement("div");
    elem.id="hidden_div";
    elem.style.width = "fit-content";
    elem.style.position = "absolute";
    elem.style.visibility = "hidden";
    elem.style.left = "-1000px";
    elem.style.right = "-1000px";
    
    elem.innerHTML = x;
    //let img_elem = document.getElementById("img");
    document.body.appendChild(elem);

    const foreigner = document.getElementById("math");
    const width = foreigner.scrollWidth;
    console.log("check width");
    console.log(width);
    foreign.remove();
    //elem.remove();
    //console.log("removed width");
    return width;
}
export function getElemHeight(x){

    const   hiddenDiv = document.getElementById("hidden_div");
    if (hiddenDiv) {
       hiddenDiv.innerHTML = x;
       const foreign = document.getElementById("math");
        const height = hiddenDiv.offsetHeight;
        console.log("check width");
        foreign.remove();
        return height;
    }
    const elem = document.createElement("div");
   elem.id="hidden_div";
   elem.style.width = "fit-content";
    elem.style.position = "absolute";
    elem.style.visibility = "hidden";
    elem.style.left = "-1000px";
    elem.style.right = "-1000px";

    elem.innerHTML = x;
    document.body.appendChild(elem);
    const height = elem.scrollHeight;
    console.log("check height");
    console.log(height);
    const foreigne = document.getElementById("math");
    foreigne.remove();
    
    //elem.remove();
    //console.log("removed height");
    return height;
}